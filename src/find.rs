//! Find bar: regex search (F3 / Enter = next match) and Replace all. Line-based.

use std::ops::Range;

use eframe::egui;
use egui::Key;
use regex::Regex;

use crate::document::{Caret, Document, byte_index};
use crate::editor_view::Reveal;

const QUERY_WIDTH: f32 = 240.0;
const REPLACEMENT_WIDTH: f32 = 160.0;

#[derive(Default)]
pub struct FindState {
    pub open: bool,
    pub focus_query: bool,
    query: String,
    replacement: String,
    compiled_query: String,
    pub regex: Option<Regex>,
    error: Option<String>,
    status: String,
    /// The match the caret was moved to: line and byte range.
    pub current: Option<(usize, Range<usize>)>,
}

pub enum FindAction {
    None,
    Next,
    ReplaceAll,
    Close,
}

impl FindState {
    fn recompile(&mut self) {
        if self.query == self.compiled_query {
            return;
        }
        self.compiled_query.clone_from(&self.query);
        self.current = None;
        self.status.clear();
        (self.regex, self.error) = if self.query.is_empty() {
            (None, None)
        } else {
            match Regex::new(&self.query) {
                Ok(regex) => (Some(regex), None),
                Err(error) => (None, Some(error.to_string())),
            }
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> FindAction {
        let mut action = FindAction::None;
        ui.horizontal(|ui| {
            ui.label("Find (regex):");
            let query = ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .desired_width(QUERY_WIDTH)
                    .hint_text("e.g. compute_\\d+"),
            );
            if std::mem::take(&mut self.focus_query) {
                query.request_focus();
            }
            if query.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter)) {
                action = FindAction::Next;
                query.request_focus();
            }
            if ui.button("Next match (F3)").clicked() {
                action = FindAction::Next;
            }
            ui.separator();
            ui.label("Replace with:");
            ui.add(
                egui::TextEdit::singleline(&mut self.replacement)
                    .desired_width(REPLACEMENT_WIDTH)
                    .hint_text("$1 inserts group 1"),
            );
            if ui.button("Replace all").clicked() {
                action = FindAction::ReplaceAll;
            }
            if ui.button("Close (Esc)").clicked() {
                action = FindAction::Close;
            }
            if let Some(error) = &self.error {
                let summary = error.lines().last().unwrap_or("invalid pattern");
                ui.colored_label(
                    ui.visuals().error_fg_color,
                    format!("Invalid pattern: {summary}"),
                );
            } else if !self.status.is_empty() {
                ui.label(&self.status);
            }
        });
        self.recompile();
        action
    }

    /// Moves the caret to the end of the next match after it, wrapping around once.
    pub fn find_next(&mut self, doc: &mut Document) {
        self.recompile();
        let Some(regex) = self.regex.clone() else {
            return;
        };
        let line_count = doc.lines.len();
        let start_line = doc.caret.line;
        let start_byte = byte_index(&doc.lines[start_line], doc.caret.col);
        // Step 0 is the caret line after the caret; step `line_count` is the caret line again, from its start.
        for step in 0..=line_count {
            let index = (start_line + step) % line_count;
            let line = &doc.lines[index];
            let found = regex.find_iter(line).find(|found| {
                step != 0
                    || (found.start() >= start_byte
                        && !(found.is_empty() && found.start() == start_byte))
            });
            if let Some(found) = found {
                doc.caret = Caret {
                    line: index,
                    col: line[..found.end()].chars().count(),
                };
                doc.reveal = Some(Reveal::Center);
                self.current = Some((index, found.range()));
                self.status.clear();
                return;
            }
        }
        self.current = None;
        self.status = "No matches".to_owned();
    }

    pub fn replace_all(&mut self, doc: &mut Document) {
        self.recompile();
        let Some(regex) = self.regex.clone() else {
            return;
        };
        let mut count = 0;
        for line in &mut doc.lines {
            let matches = regex.find_iter(line).count();
            if matches > 0 {
                *line = regex
                    .replace_all(line, self.replacement.as_str())
                    .into_owned();
                count += matches;
            }
        }
        if count > 0 {
            doc.edited_everywhere();
        }
        self.current = None;
        self.status = format!("Replaced {count} matches");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_next_wraps_and_replace_all_counts() {
        let mut doc = Document::from_text(None, "foo bar\nbaz foo\nfoo", false);
        let mut find = FindState {
            query: "fo+".to_owned(),
            replacement: "x".to_owned(),
            ..FindState::default()
        };
        find.find_next(&mut doc);
        assert_eq!(doc.caret, Caret { line: 0, col: 3 });
        find.find_next(&mut doc);
        assert_eq!(doc.caret, Caret { line: 1, col: 7 });
        find.find_next(&mut doc);
        find.find_next(&mut doc);
        assert_eq!(doc.caret, Caret { line: 0, col: 3 }, "wraps to the top");
        find.replace_all(&mut doc);
        assert_eq!(doc.lines, ["x bar", "baz x", "x"]);
        assert!(doc.modified);
    }
}
