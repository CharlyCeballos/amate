//! Syntax highlighting with syntect (pure-Rust fancy-regex), only for the lines on screen.

use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use std::sync::LazyLock;
use std::time::Instant;

use eframe::egui::Color32;
use syntect::highlighting::{
    HighlightIterator, HighlightState, Highlighter as ThemeHighlighter, Theme, ThemeSet,
};
use syntect::parsing::{ParseState, ParsingError, ScopeStack, SyntaxReference, SyntaxSet};

/// Parse state is kept at the start of every Nth line, so an edit re-parses at most N lines above it.
const CHECKPOINT_EVERY: usize = 16;
/// Above this many cached lines, spans outside the current view are dropped.
const MAX_CACHED_LINES: usize = 4096;
const THEME_NAME: &str = "base16-ocean.dark";

static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME: LazyLock<Theme> = LazyLock::new(|| {
    ThemeSet::load_defaults()
        .themes
        .remove(THEME_NAME)
        .expect("syntect bundles the base16-ocean.dark theme")
});
static THEME_HIGHLIGHTER: LazyLock<ThemeHighlighter<'static>> =
    LazyLock::new(|| ThemeHighlighter::new(&THEME));

pub type Spans = Vec<(Range<usize>, Color32)>;
type State = (ParseState, HighlightState);

/// Loads the syntax and theme dumps off the UI thread, so the first frame does not wait for them.
pub fn preload() {
    std::thread::spawn(|| {
        LazyLock::force(&THEME_HIGHLIGHTER);
        LazyLock::force(&SYNTAXES);
    });
}

pub struct Highlighter {
    syntax: &'static SyntaxReference,
    /// `checkpoints[k]` is the state at the start of line `k * CHECKPOINT_EVERY`.
    checkpoints: Vec<State>,
    spans: HashMap<usize, Spans>,
    pub error: Option<String>,
}

impl Highlighter {
    /// JSON and C++ only, by file extension (spike scope).
    pub fn for_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_ascii_lowercase();
        let token = match extension.as_str() {
            "json" => "json",
            "cpp" | "cc" | "cxx" | "hpp" | "hh" | "h" => "cpp",
            _ => return None,
        };
        let syntax = SYNTAXES.find_syntax_by_extension(token)?;
        Some(Self {
            syntax,
            checkpoints: Vec::new(),
            spans: HashMap::new(),
            error: None,
        })
    }

    pub fn name(&self) -> &str {
        &self.syntax.name
    }

    fn initial_state(&self) -> State {
        (
            ParseState::new(self.syntax),
            HighlightState::new(&THEME_HIGHLIGHTER, ScopeStack::new()),
        )
    }

    /// Lines before `line` are unchanged, so their spans and checkpoints stay valid.
    pub fn invalidate_from(&mut self, line: usize) {
        self.checkpoints.truncate(line / CHECKPOINT_EVERY + 1);
        self.spans.retain(|&cached, _| cached < line);
    }

    pub fn spans(&self, line: usize) -> Option<&Spans> {
        self.spans.get(&line)
    }

    /// Highlights `visible`, extending checkpoints from the top until `deadline`.
    /// Returns false when out of time: the caller paints those lines plain and repaints.
    pub fn prepare(&mut self, lines: &[String], visible: Range<usize>, deadline: Instant) -> bool {
        if self.error.is_some() || visible.clone().all(|line| self.spans.contains_key(&line)) {
            return true;
        }
        match self.try_prepare(lines, visible, deadline) {
            Ok(done) => done,
            Err(error) => {
                self.error = Some(error.to_string());
                self.spans.clear();
                true
            }
        }
    }

    fn try_prepare(
        &mut self,
        lines: &[String],
        visible: Range<usize>,
        deadline: Instant,
    ) -> Result<bool, ParsingError> {
        if self.checkpoints.is_empty() {
            self.checkpoints.push(self.initial_state());
        }
        let start_checkpoint = visible.start / CHECKPOINT_EVERY;
        while self.checkpoints.len() <= start_checkpoint {
            if Instant::now() >= deadline {
                return Ok(false);
            }
            let index = self.checkpoints.len() - 1;
            let mut state = self.checkpoints[index].clone();
            let first = index * CHECKPOINT_EVERY;
            for line in &lines[first..first + CHECKPOINT_EVERY] {
                parse_line(&mut state, line)?;
            }
            self.checkpoints.push(state);
        }
        if self.spans.len() > MAX_CACHED_LINES {
            self.spans.retain(|line, _| visible.contains(line));
        }
        let first = start_checkpoint * CHECKPOINT_EVERY;
        let mut state = self.checkpoints[start_checkpoint].clone();
        for (index, line) in lines.iter().enumerate().take(visible.end).skip(first) {
            let spans = parse_line(&mut state, line)?;
            if index >= visible.start {
                self.spans.insert(index, spans);
            }
            let next = index + 1;
            if next.is_multiple_of(CHECKPOINT_EVERY)
                && self.checkpoints.len() == next / CHECKPOINT_EVERY
            {
                self.checkpoints.push(state.clone());
            }
        }
        Ok(true)
    }
}

fn parse_line(state: &mut State, line: &str) -> Result<Spans, ParsingError> {
    // The "newlines" syntax set expects every line to end with '\n'.
    let text = format!("{line}\n");
    let ops = state.0.parse_line(&text, &SYNTAXES)?;
    let mut spans = Spans::new();
    let mut start = 0;
    for (style, piece) in HighlightIterator::new(&mut state.1, &ops, &text, &THEME_HIGHLIGHTER) {
        let end = (start + piece.len()).min(line.len());
        if end > start {
            let color = style.foreground;
            spans.push((start..end, Color32::from_rgb(color.r, color.g, color.b)));
        }
        start += piece.len();
    }
    Ok(spans)
}

/// Bench only: time to parse every line from the top, i.e. the cost of a full-file highlight.
pub fn full_parse_ms(path: &Path, lines: &[String]) -> Result<f64, String> {
    let highlighter = Highlighter::for_path(path).ok_or("no syntax for this file")?;
    let mut state = highlighter.initial_state();
    let started = Instant::now();
    for line in lines {
        parse_line(&mut state, line).map_err(|error| error.to_string())?;
    }
    Ok(started.elapsed().as_secs_f64() * 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_cover_cpp_line_and_survive_edits() {
        let mut highlighter = Highlighter::for_path(Path::new("x.cpp")).expect("C++ syntax");
        let lines: Vec<String> = (0..40)
            .map(|n| format!("int value{n} = {n}; // note"))
            .collect();
        let far = Instant::now() + std::time::Duration::from_secs(10);
        assert!(highlighter.prepare(&lines, 30..40, far));
        let spans = highlighter.spans(35).expect("highlighted");
        assert_eq!(
            spans.last().map(|(range, _)| range.end),
            Some(lines[35].len())
        );
        assert!(
            spans.len() > 3,
            "keyword, number and comment get their own colors"
        );
        highlighter.invalidate_from(33);
        assert!(highlighter.spans(32).is_some() && highlighter.spans(33).is_none());
        assert!(highlighter.prepare(&lines, 30..40, far));
        assert!(highlighter.spans(39).is_some());
    }
}
