//! The --bench mode: startup, open and keystroke-latency measurements, printed as one JSON object.
//!
//! Every "end of frame" below is the start of the next frame (eframe's raw_input_hook), which
//! eframe only reaches after painting and swap_buffers returned: the closest point to "presented"
//! that eframe exposes. The bench requests a repaint every frame, so the next frame starts at once.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use eframe::egui;

use crate::document::{Caret, Document};
use crate::editor_view::Reveal;
use crate::{App, TabState, highlight};

const WARMUP_KEYS: usize = 20;
const MEASURED_KEYS: usize = 500;
/// Frames with nothing to do after the first one, to see the presentation cadence.
const IDLE_FRAMES: u64 = 60;
/// Frames to wait after jumping to the middle of the file, so the scroll animation ends.
const SETTLE_FRAMES: u32 = 30;
const WATCHDOG: Duration = Duration::from_secs(180);
const MIB: usize = 1024 * 1024;
const SMALL_FILE_BYTES: usize = MIB;
/// At most the degraded-mode threshold, so the 10 MB file still gets highlighting (Q11).
const LARGE_FILE_BYTES: usize = 10 * MIB;
const TYPED_TEXT: &str = "auto total = compute(values, 42); ";

/// Ends a stuck bench (e.g. a minimized window never paints) instead of hanging the script.
pub fn start_watchdog() {
    std::thread::spawn(|| {
        std::thread::sleep(WATCHDOG);
        eprintln!("bench: watchdog expired after {WATCHDOG:?}; was the window visible?");
        std::process::exit(2);
    });
}

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("spike-data")
}

fn small_path() -> PathBuf {
    data_dir().join("spike-1mb.cpp")
}

fn large_path() -> PathBuf {
    data_dir().join("spike-10mb.cpp")
}

fn ensure_data_files() -> std::io::Result<()> {
    fs::create_dir_all(data_dir())?;
    for (path, max_bytes) in [
        (small_path(), SMALL_FILE_BYTES),
        (large_path(), LARGE_FILE_BYTES),
    ] {
        if !path.exists() {
            fs::write(&path, synthetic_cpp(max_bytes))?;
        }
    }
    Ok(())
}

/// Deterministic C++-like text, cut at a block boundary so it never exceeds max_bytes.
fn synthetic_cpp(max_bytes: usize) -> String {
    let mut out = String::with_capacity(max_bytes);
    for block in 0_u32.. {
        let text = format!(
            r#"// Block {block}: synthetic C++ for the Amate egui spike.
#include <vector>
namespace spike_{block} {{
struct Point{block} {{ int x; int y; double weight = 1.5; }};
static int compute_{block}(const std::vector<int>& values, int factor) {{
    int total = 0; // running sum
    for (std::size_t i = 0; i < values.size(); ++i) {{
        if (values[i] % 2 == 0) {{ total += values[i] * factor; }} else {{ total -= 0x1F; }}
    }}
    const char* label = "block {block} \"quoted\"";
    return total + static_cast<int>(sizeof(label));
}}
}}  // namespace spike_{block}

"#
        );
        if out.len() + text.len() > max_bytes {
            break;
        }
        out.push_str(&text);
    }
    out
}

/// About max_bytes of JSON records, one per line, to compare syntect's JSON and C++ throughput.
fn synthetic_json_lines(max_bytes: usize) -> Vec<String> {
    let mut lines = vec!["[".to_owned()];
    let mut bytes = 0;
    for id in 0_u32.. {
        let line = format!(
            r#"  {{"id": {id}, "name": "item {id}", "price": {id}.25, "tags": ["a", "b\"c"], "active": true, "parent": null}},"#
        );
        bytes += line.len() + 1;
        if bytes > max_bytes {
            break;
        }
        lines.push(line);
    }
    lines.push("]".to_owned());
    lines
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// Nearest-rank percentile of sorted, non-empty samples.
fn percentile(sorted: &[f64], percent: f64) -> f64 {
    let rank = (percent / 100.0 * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

fn stats_json(samples: &[f64]) -> String {
    let mut sorted: Vec<f64> = samples
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    if sorted.is_empty() {
        return "null".to_owned();
    }
    sorted.sort_by(f64::total_cmp);
    let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
    format!(
        r#"{{"p50":{:.3},"p95":{:.3},"p99":{:.3},"max":{:.3},"mean":{:.3},"n":{}}}"#,
        percentile(&sorted, 50.0),
        percentile(&sorted, 95.0),
        percentile(&sorted, 99.0),
        sorted[sorted.len() - 1],
        mean,
        sorted.len()
    )
}

fn full_parse_json(path: &Path, lines: &[String]) -> String {
    match highlight::full_parse_ms(path, lines) {
        Ok(value) => format!("{value:.1}"),
        Err(error) => {
            eprintln!("bench: full parse of {} failed: {error}", path.display());
            "null".to_owned()
        }
    }
}

enum Stage {
    Startup,
    OpenSmall {
        started: Instant,
        tab: u64,
    },
    OpenLarge {
        started: Instant,
        tab: u64,
        small_tab: u64,
    },
    Settle {
        frames: u32,
        small_tab: u64,
    },
    Typing {
        small_tab: u64,
        highlighted: bool,
    },
    Done,
}

/// One typing run: per injected key, aligned by index; the first WARMUP_KEYS are dropped.
#[derive(Default)]
struct Samples {
    latency: Vec<f64>,
    to_ui_end: Vec<f64>,
    frame_cpu: Vec<f64>,
}

impl Samples {
    fn json(&self) -> String {
        let measured = |samples: &[f64]| stats_json(samples.get(WARMUP_KEYS..).unwrap_or_default());
        format!(
            r#"{{"key_latency_ms":{},"key_to_ui_end_ms":{},"key_frame_cpu_ms":{}}}"#,
            measured(&self.latency),
            measured(&self.to_ui_end),
            measured(&self.frame_cpu)
        )
    }
}

#[derive(Default)]
struct Report {
    startup_ms: f64,
    idle_frame_gaps: Vec<f64>,
    open_small_ms: f64,
    small_load_thread_ms: f64,
    small_lines: usize,
    open_large_ms: f64,
    large_load_thread_ms: f64,
    large_lines: usize,
    max_frame_gap_large_load_ms: f64,
}

pub struct Bench {
    started: Instant,
    stage: Stage,
    frame: u64,
    frame_start: Instant,
    frame_gap_ms: Option<f64>,
    injected_at: Option<Instant>,
    last_ui_ms: Option<f64>,
    awaiting_cpu: bool,
    report: Report,
    samples: Samples,
    highlighted: Option<Samples>,
}

impl Bench {
    pub fn new(started: Instant) -> Self {
        Self {
            started,
            stage: Stage::Startup,
            frame: 0,
            frame_start: started,
            frame_gap_ms: None,
            injected_at: None,
            last_ui_ms: None,
            awaiting_cpu: false,
            report: Report::default(),
            samples: Samples::default(),
            highlighted: None,
        }
    }

    /// Called at the end of App::ui (it may run twice per frame; the last call wins).
    pub fn after_ui(&mut self, cpu_usage: Option<f32>) {
        if let Some(injected) = self.injected_at {
            self.last_ui_ms = Some(ms(injected.elapsed()));
        }
        if std::mem::take(&mut self.awaiting_cpu)
            && let Some(seconds) = cpu_usage
        {
            self.samples.frame_cpu.push(f64::from(seconds) * 1000.0);
        }
    }

    fn to_json(&self, pixels_per_point: f32, small_lines: &[String]) -> String {
        let report = &self.report;
        let file_bytes = |path: PathBuf| fs::metadata(path).map_or(0, |meta| meta.len());
        let highlighted = self
            .highlighted
            .as_ref()
            .map_or_else(|| "null".to_owned(), Samples::json);
        // Measured last: each blocks the UI thread for seconds.
        let cpp_parse = full_parse_json(&small_path(), small_lines);
        let json_parse = full_parse_json(
            Path::new("synthetic.json"),
            &synthetic_json_lines(SMALL_FILE_BYTES),
        );
        format!(
            r#"{{"toolkit":"eframe 0.37 glow, vsync on","pixels_per_point":{pixels_per_point},"startup_to_first_frame_ms":{:.1},"idle_frame_interval_ms":{},"open_1mb_ms":{:.1},"open_1mb_load_thread_ms":{:.1},"open_10mb_ms":{:.1},"open_10mb_load_thread_ms":{:.1},"file_1mb":{{"bytes":{},"lines":{}}},"file_10mb":{{"bytes":{},"lines":{}}},"max_frame_gap_during_10mb_load_ms":{:.1},"highlight_full_parse_1mb_cpp_ms":{cpp_parse},"highlight_full_parse_1mb_json_ms":{json_parse},"typing_highlighted":{highlighted},"typing_plain":{},"keys":{{"warmup":{WARMUP_KEYS},"measured":{MEASURED_KEYS}}}}}"#,
            report.startup_ms,
            stats_json(&report.idle_frame_gaps),
            report.open_small_ms,
            report.small_load_thread_ms,
            report.open_large_ms,
            report.large_load_thread_ms,
            file_bytes(small_path()),
            report.small_lines,
            file_bytes(large_path()),
            report.large_lines,
            report.max_frame_gap_large_load_ms,
            self.samples.json(),
        )
    }
}

fn fail(message: &str) -> ! {
    eprintln!("bench failed: {message}");
    std::process::exit(1);
}

impl App {
    fn doc_by_id(&mut self, id: u64) -> Option<&mut Document> {
        self.tabs
            .iter_mut()
            .find(|tab| tab.id == id)
            .and_then(|tab| match &mut tab.state {
                TabState::Ready(doc) => Some(doc.as_mut()),
                TabState::Loading { .. } => None,
            })
    }

    /// True when the previous frame painted tab id completely (highlighting included).
    fn shown(&self, id: u64) -> bool {
        self.last_view
            .is_some_and(|(shown, view)| shown == id && view.complete)
    }

    fn ensure_tab(&self, id: u64) {
        if !self.tabs.iter().any(|tab| tab.id == id) {
            fail(&self.errors.join("; "));
        }
    }

    pub(crate) fn bench_frame_start(
        &mut self,
        ctx: &egui::Context,
        raw_input: &mut egui::RawInput,
    ) {
        let Some(mut bench) = self.bench.take() else {
            return;
        };
        let now = Instant::now();
        bench.frame += 1;
        bench.frame_gap_ms = (bench.frame > 1).then(|| ms(now - bench.frame_start));
        bench.frame_start = now;
        let stage = std::mem::replace(&mut bench.stage, Stage::Done);
        bench.stage = self.bench_step(&mut bench, stage, ctx, raw_input);
        if !matches!(bench.stage, Stage::Done) {
            ctx.request_repaint();
        }
        self.bench = Some(bench);
    }

    fn bench_step(
        &mut self,
        bench: &mut Bench,
        stage: Stage,
        ctx: &egui::Context,
        raw_input: &mut egui::RawInput,
    ) -> Stage {
        let now = bench.frame_start;
        match stage {
            Stage::Startup if bench.frame < 2 => Stage::Startup,
            Stage::Startup if bench.frame == 2 => {
                // Frame 2 starts once frame 1 was painted and swapped.
                bench.report.startup_ms = ms(now - bench.started);
                Stage::Startup
            }
            Stage::Startup if bench.frame <= IDLE_FRAMES + 2 => {
                bench.report.idle_frame_gaps.extend(bench.frame_gap_ms);
                Stage::Startup
            }
            Stage::Startup => {
                if let Err(error) = ensure_data_files() {
                    fail(&format!("cannot write the bench files: {error}"));
                }
                let started = Instant::now();
                let tab = self.open_file(small_path(), ctx);
                Stage::OpenSmall { started, tab }
            }
            Stage::OpenSmall { started, tab } => {
                self.ensure_tab(tab);
                if !self.shown(tab) {
                    return Stage::OpenSmall { started, tab };
                }
                bench.report.open_small_ms = ms(now - started);
                if let Some(doc) = self.doc_by_id(tab) {
                    bench.report.small_load_thread_ms = doc.load_ms;
                    bench.report.small_lines = doc.lines.len();
                }
                let started = Instant::now();
                let large = self.open_file(large_path(), ctx);
                Stage::OpenLarge {
                    started,
                    tab: large,
                    small_tab: tab,
                }
            }
            Stage::OpenLarge {
                started,
                tab,
                small_tab,
            } => {
                self.ensure_tab(tab);
                if let Some(gap) = bench.frame_gap_ms {
                    let max = &mut bench.report.max_frame_gap_large_load_ms;
                    *max = max.max(gap);
                }
                if !self.shown(tab) {
                    return Stage::OpenLarge {
                        started,
                        tab,
                        small_tab,
                    };
                }
                bench.report.open_large_ms = ms(now - started);
                if let Some(doc) = self.doc_by_id(tab) {
                    bench.report.large_load_thread_ms = doc.load_ms;
                    bench.report.large_lines = doc.lines.len();
                }
                self.prepare_typing(small_tab);
                Stage::Settle {
                    frames: 0,
                    small_tab,
                }
            }
            Stage::Settle { frames, small_tab } => {
                let ready = frames >= SETTLE_FRAMES
                    && self.last_view.is_some_and(|(shown, view)| {
                        shown == small_tab && view.complete && view.caret_visible
                    });
                if ready {
                    self.type_key(bench, ctx, raw_input, small_tab, true)
                } else {
                    Stage::Settle {
                        frames: frames + 1,
                        small_tab,
                    }
                }
            }
            Stage::Typing {
                small_tab,
                highlighted,
            } => self.type_key(bench, ctx, raw_input, small_tab, highlighted),
            Stage::Done => Stage::Done,
        }
    }

    /// Activates the 1 MB tab with the caret in the middle of the file, centered on screen.
    fn prepare_typing(&mut self, small_tab: u64) {
        let Some(index) = self.tabs.iter().position(|tab| tab.id == small_tab) else {
            fail("the 1 MB tab disappeared");
        };
        self.active = index;
        self.focus_editor = true;
        let Some(doc) = self.doc_by_id(small_tab) else {
            fail("the 1 MB tab is not loaded");
        };
        doc.caret = Caret {
            line: doc.lines.len() / 2,
            col: 0,
        };
        doc.reveal = Some(Reveal::Center);
    }

    /// Records the key injected last frame (that frame has been presented now), then injects the
    /// next one. Runs once with highlighting, then once more with it switched off.
    fn type_key(
        &mut self,
        bench: &mut Bench,
        ctx: &egui::Context,
        raw_input: &mut egui::RawInput,
        small_tab: u64,
        highlighted: bool,
    ) -> Stage {
        if let Some(injected) = bench.injected_at.take() {
            let samples = &mut bench.samples;
            samples.latency.push(ms(bench.frame_start - injected));
            samples
                .to_ui_end
                .push(bench.last_ui_ms.take().unwrap_or(f64::NAN));
            bench.awaiting_cpu = true;
        }
        if bench.samples.latency.len() >= WARMUP_KEYS + MEASURED_KEYS {
            bench.awaiting_cpu = false;
            let Some(doc) = self.doc_by_id(small_tab) else {
                fail("the 1 MB tab is not loaded");
            };
            if highlighted {
                doc.highlighter = None;
                bench.highlighted = Some(std::mem::take(&mut bench.samples));
                return Stage::Typing {
                    small_tab,
                    highlighted: false,
                };
            }
            // The pristine file, not the typed-into document.
            let original = fs::read_to_string(small_path())
                .unwrap_or_else(|error| fail(&format!("cannot reread the 1 MB file: {error}")));
            let small_lines: Vec<String> = original.lines().map(str::to_owned).collect();
            println!("{}", bench.to_json(ctx.pixels_per_point(), &small_lines));
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return Stage::Done;
        }
        let typed = TYPED_TEXT
            .chars()
            .cycle()
            .nth(bench.samples.latency.len())
            .unwrap_or('x');
        raw_input.events.push(egui::Event::Text(typed.to_string()));
        bench.injected_at = Some(Instant::now());
        Stage::Typing {
            small_tab,
            highlighted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_is_nearest_rank() {
        let samples: Vec<f64> = (1..=100).map(f64::from).collect();
        assert_eq!(percentile(&samples, 50.0), 50.0);
        assert_eq!(percentile(&samples, 99.0), 99.0);
        assert_eq!(percentile(&[7.0], 95.0), 7.0);
    }

    #[test]
    fn synthetic_files_respect_their_size_cap() {
        let text = synthetic_cpp(10_000);
        assert!(text.len() <= 10_000 && text.len() > 9_000);
        assert!(text.ends_with("\n\n"));
        let json_bytes: usize = synthetic_json_lines(10_000)
            .iter()
            .map(|line| line.len() + 1)
            .sum();
        assert!(json_bytes <= 10_100);
    }
}
