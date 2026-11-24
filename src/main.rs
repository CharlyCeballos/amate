//! Throwaway spike: measures whether egui/eframe can carry Amate's code-editor view.
//!
//! `amate-spike [--bench] [--font PATH] FILE...`

mod bench;
mod document;
mod editor_view;
mod find;
mod highlight;

use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Instant;

use eframe::egui;
use egui::text::{FontData, FontFamily};
use egui::{Key, Modifiers};

use document::{Document, LoadMessage};
use editor_view::{FindHighlight, ViewOutput};
use find::{FindAction, FindState};

const APP_NAME: &str = "Amate spike (egui)";
const WINDOW_SIZE: [f32; 2] = [1200.0, 800.0];
const USER_FONT_NAME: &str = "user-monospace";

struct Args {
    bench: bool,
    font: Option<PathBuf>,
    files: Vec<PathBuf>,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        bench: false,
        font: None,
        files: Vec::new(),
    };
    let mut raw = std::env::args_os().skip(1);
    while let Some(arg) = raw.next() {
        match arg.to_str() {
            Some("--bench") => args.bench = true,
            Some("--font") => args.font = Some(raw.next().ok_or("--font needs a path")?.into()),
            _ => args.files.push(arg.into()),
        }
    }
    Ok(args)
}

enum TabState {
    Loading {
        path: PathBuf,
        progress: f32,
        receiver: Receiver<LoadMessage>,
    },
    Ready(Box<Document>),
}

struct Tab {
    id: u64,
    state: TabState,
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

impl Tab {
    fn title(&self) -> String {
        match &self.state {
            TabState::Loading { path, progress, .. } => {
                format!("{} ({:.0}%)", file_name(path), progress * 100.0)
            }
            TabState::Ready(doc) if doc.modified => format!("{} ●", doc.display_name()),
            TabState::Ready(doc) => doc.display_name(),
        }
    }
}

struct App {
    tabs: Vec<Tab>,
    active: usize,
    next_tab_id: u64,
    find: FindState,
    errors: Vec<String>,
    focus_editor: bool,
    /// The active tab's view output for this frame, read by the bench at the next frame start.
    last_view: Option<(u64, ViewOutput)>,
    bench: Option<bench::Bench>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, args: Args, started: Instant) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        let mut app = Self {
            tabs: Vec::new(),
            active: 0,
            next_tab_id: 0,
            find: FindState::default(),
            errors: Vec::new(),
            focus_editor: true,
            last_view: None,
            bench: args.bench.then(|| bench::Bench::new(started)),
        };
        if let Some(font) = &args.font {
            app.install_font(&cc.egui_ctx, font);
        }
        if !args.bench {
            for path in args.files {
                app.open_file(path, &cc.egui_ctx);
            }
        }
        if app.tabs.is_empty() {
            app.add_untitled();
        }
        app.active = 0;
        app
    }

    /// Puts a user font first in the monospace family, e.g. to check ligatures (Cascadia Code).
    fn install_font(&mut self, ctx: &egui::Context, path: &Path) {
        match std::fs::read(path) {
            Ok(bytes) => ctx.add_font(egui::epaint::text::FontInsert::new(
                USER_FONT_NAME,
                FontData::from_owned(bytes),
                vec![egui::epaint::text::InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: egui::epaint::text::FontPriority::Highest,
                }],
            )),
            Err(error) => self
                .errors
                .push(format!("--font {}: {error}", path.display())),
        }
    }

    fn add_untitled(&mut self) {
        self.tabs.push(Tab {
            id: self.next_tab_id,
            state: TabState::Ready(Box::new(Document::untitled())),
        });
        self.next_tab_id += 1;
    }

    /// Starts loading on a background thread and activates the new tab.
    fn open_file(&mut self, path: PathBuf, ctx: &egui::Context) -> u64 {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        let receiver = document::spawn_load(path.clone(), ctx.clone());
        self.tabs.push(Tab {
            id,
            state: TabState::Loading {
                path,
                progress: 0.0,
                receiver,
            },
        });
        self.active = self.tabs.len() - 1;
        id
    }

    fn active_doc(&mut self) -> Option<&mut Document> {
        match &mut self.tabs.get_mut(self.active)?.state {
            TabState::Ready(doc) => Some(doc),
            TabState::Loading { .. } => None,
        }
    }

    fn poll_loads(&mut self) {
        let mut failed = Vec::new();
        for tab in &mut self.tabs {
            let TabState::Loading {
                path,
                progress,
                receiver,
            } = &mut tab.state
            else {
                continue;
            };
            let mut finished = None;
            loop {
                match receiver.try_recv() {
                    Ok(LoadMessage::Progress(value)) => *progress = value,
                    Ok(LoadMessage::Done(result)) => {
                        finished =
                            Some(result.map_err(|error| format!("{}: {error}", path.display())));
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        finished = Some(Err(format!("{}: loader thread stopped", path.display())));
                        break;
                    }
                }
            }
            match finished {
                Some(Ok(doc)) => tab.state = TabState::Ready(doc),
                Some(Err(error)) => failed.push((tab.id, error)),
                None => {}
            }
        }
        if failed.is_empty() {
            return;
        }
        let active_id = self.tabs.get(self.active).map(|tab| tab.id);
        for (id, error) in failed {
            self.tabs.retain(|tab| tab.id != id);
            self.errors.push(error);
        }
        if self.tabs.is_empty() {
            self.add_untitled();
        }
        self.active = active_id
            .and_then(|id| self.tabs.iter().position(|tab| tab.id == id))
            .unwrap_or(0);
    }

    fn handle_shortcuts(&mut self, ui: &egui::Ui) {
        let find_open = self.find.open;
        let (save, open_find, next, close_find) = ui.input_mut(|input| {
            (
                input.consume_key(Modifiers::COMMAND, Key::S),
                input.consume_key(Modifiers::COMMAND, Key::F),
                input.consume_key(Modifiers::NONE, Key::F3),
                find_open && input.consume_key(Modifiers::NONE, Key::Escape),
            )
        });
        if save && let Some(doc) = self.active_doc() {
            let result = doc.save();
            if let Err(error) = result {
                self.errors.push(error);
            }
        }
        if open_find {
            self.find.open = true;
            self.find.focus_query = true;
        }
        if next {
            self.find_next();
        }
        if close_find {
            self.close_find();
        }
    }

    fn find_next(&mut self) {
        let Self {
            tabs, active, find, ..
        } = self;
        if let Some(Tab {
            state: TabState::Ready(doc),
            ..
        }) = tabs.get_mut(*active)
        {
            find.find_next(doc);
        }
    }

    fn close_find(&mut self) {
        self.find.open = false;
        self.find.current = None;
        self.focus_editor = true;
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        let mut clicked = None;
        egui::Panel::top("tabs").show(ui, |ui| {
            ui.horizontal(|ui| {
                for (index, tab) in self.tabs.iter().enumerate() {
                    if ui
                        .selectable_label(index == self.active, tab.title())
                        .clicked()
                    {
                        clicked = Some(index);
                    }
                }
            });
        });
        if let Some(index) = clicked {
            self.active = index;
            self.focus_editor = true;
        }
    }

    fn find_bar(&mut self, ui: &mut egui::Ui) {
        if !self.find.open {
            return;
        }
        let action = egui::Panel::top("find")
            .show(ui, |ui| self.find.ui(ui))
            .inner;
        match action {
            FindAction::Next => self.find_next(),
            FindAction::ReplaceAll => {
                let Self {
                    tabs, active, find, ..
                } = self;
                if let Some(Tab {
                    state: TabState::Ready(doc),
                    ..
                }) = tabs.get_mut(*active)
                {
                    find.replace_all(doc);
                }
            }
            FindAction::Close => self.close_find(),
            FindAction::None => {}
        }
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(Tab {
                    state: TabState::Ready(doc),
                    ..
                }) = self.tabs.get(self.active)
                {
                    ui.label(format!(
                        "Ln {}, Col {}",
                        doc.caret.line + 1,
                        doc.caret.col + 1
                    ));
                    ui.separator();
                    ui.label(format!("{} lines", doc.lines.len()));
                    ui.separator();
                    ui.label(if doc.crlf { "CRLF" } else { "LF" });
                    ui.separator();
                    ui.label(doc.mode_label());
                }
                if !self.errors.is_empty() {
                    ui.separator();
                    for error in &self.errors {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                    }
                    if ui.button("Dismiss").clicked() {
                        self.errors.clear();
                    }
                }
            });
        });
    }

    fn central(&mut self, ui: &mut egui::Ui) {
        let Some(tab) = self.tabs.get_mut(self.active) else {
            return;
        };
        match &mut tab.state {
            TabState::Loading { path, progress, .. } => {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    ui.label(format!("Loading {}…", path.display()));
                    ui.add(egui::ProgressBar::new(*progress).show_percentage());
                });
            }
            TabState::Ready(doc) => {
                let find = FindHighlight {
                    regex: self.find.regex.as_ref().filter(|_| self.find.open),
                    current: self.find.current.clone(),
                };
                let output = editor_view::show(
                    ui,
                    egui::Id::unique(("editor", tab.id)),
                    doc,
                    &find,
                    std::mem::take(&mut self.focus_editor),
                );
                self.last_view = Some((tab.id, output));
            }
        }
    }
}

impl eframe::App for App {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.bench_frame_start(ctx, raw_input);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.last_view = None;
        self.poll_loads();
        self.handle_shortcuts(ui);
        self.tab_bar(ui);
        self.find_bar(ui);
        self.status_bar(ui);
        egui::CentralPanel::default().show(ui, |ui| self.central(ui));
        if let Some(bench) = &mut self.bench {
            bench.after_ui(frame.info().cpu_usage);
        }
    }
}

fn main() -> eframe::Result {
    let started = Instant::now();
    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}\nusage: amate-spike [--bench] [--font PATH] FILE...");
            std::process::exit(2);
        }
    };
    highlight::preload();
    if args.bench {
        bench::start_watchdog();
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_NAME)
            .with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(move |cc| Ok(Box::new(App::new(cc, args, started)))),
    )
}
