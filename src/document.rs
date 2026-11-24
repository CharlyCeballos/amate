//! The text model: a `Vec<String>` of lines, loaded on a background thread.

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;

use eframe::egui;

use crate::editor_view::Reveal;
use crate::highlight::Highlighter;

/// Above this size a file opens in degraded mode: no highlighting, no wrapping (decision Q11).
pub const LARGE_FILE_BYTES: u64 = 10 * 1024 * 1024;
const READ_CHUNK_BYTES: usize = 1024 * 1024;

/// Caret position; `col` counts chars, not bytes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caret {
    pub line: usize,
    pub col: usize,
}

pub struct Document {
    pub path: Option<PathBuf>,
    pub lines: Vec<String>,
    pub crlf: bool,
    pub modified: bool,
    pub degraded: bool,
    pub caret: Caret,
    /// IME composition shown inline at the caret; not part of the text until committed.
    pub preedit: String,
    /// Longest line seen, in chars; sizes the horizontal scroll range (never shrinks).
    pub max_line_chars: usize,
    pub highlighter: Option<Highlighter>,
    /// Scroll the caret into view on the next frame.
    pub reveal: Option<Reveal>,
    /// Read + UTF-8 check + line split on the loader thread (bench breakdown).
    pub load_ms: f64,
}

pub enum LoadMessage {
    Progress(f32),
    Done(Result<Box<Document>, String>),
}

/// Byte offset of char `col` in `line` (end of line when past it).
pub fn byte_index(line: &str, col: usize) -> usize {
    line.char_indices()
        .nth(col)
        .map_or(line.len(), |(index, _)| index)
}

impl Document {
    pub fn untitled() -> Self {
        Self::from_text(None, "", false)
    }

    pub fn from_text(path: Option<PathBuf>, text: &str, degraded: bool) -> Self {
        let lines: Vec<String> = text
            .split('\n')
            .map(|line| line.strip_suffix('\r').unwrap_or(line).to_owned())
            .collect();
        let max_line_chars = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        let highlighter = if degraded {
            None
        } else {
            path.as_deref().and_then(Highlighter::for_path)
        };
        Self {
            path,
            crlf: text.contains("\r\n"),
            lines,
            modified: false,
            degraded,
            caret: Caret::default(),
            preedit: String::new(),
            max_line_chars,
            highlighter,
            reveal: None,
            load_ms: 0.0,
        }
    }

    pub fn display_name(&self) -> String {
        self.path.as_deref().and_then(Path::file_name).map_or_else(
            || "untitled".to_owned(),
            |name| name.to_string_lossy().into_owned(),
        )
    }

    pub fn mode_label(&self) -> String {
        if self.degraded {
            return "Large file (> 10 MB): no highlighting, no wrapping".to_owned();
        }
        match &self.highlighter {
            Some(highlighter) => match &highlighter.error {
                Some(error) => format!("Highlighting stopped: {error}"),
                None => highlighter.name().to_owned(),
            },
            None => "Plain text".to_owned(),
        }
    }

    pub fn line_chars(&self, line: usize) -> usize {
        self.lines[line].chars().count()
    }

    pub fn save(&mut self) -> Result<(), String> {
        let path = self
            .path
            .as_ref()
            .ok_or("Untitled tab: Save As is outside the spike's scope")?;
        let eol = if self.crlf { "\r\n" } else { "\n" };
        std::fs::write(path, self.lines.join(eol))
            .map_err(|error| format!("{}: {error}", path.display()))?;
        self.modified = false;
        Ok(())
    }

    fn edited(&mut self, from_line: usize) {
        self.modified = true;
        self.max_line_chars = self.max_line_chars.max(self.line_chars(self.caret.line));
        if let Some(highlighter) = &mut self.highlighter {
            highlighter.invalidate_from(from_line);
        }
    }

    /// Replaces the whole text state after a multi-line operation (Replace all).
    pub fn edited_everywhere(&mut self) {
        self.max_line_chars = self
            .lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        self.caret.col = self.caret.col.min(self.line_chars(self.caret.line));
        self.edited(0);
    }

    pub fn insert_text(&mut self, text: &str) {
        // shortcut: lone CRs are dropped; mixed line endings are normalized on save.
        let text = text.replace('\r', "");
        let Caret { line, col } = self.caret;
        let at = byte_index(&self.lines[line], col);
        let tail = self.lines[line].split_off(at);
        let mut parts = text.split('\n');
        self.lines[line].push_str(parts.next().unwrap_or_default());
        let mut current = line;
        for part in parts {
            current += 1;
            self.lines.insert(current, part.to_owned());
        }
        let new_col = self.line_chars(current);
        self.lines[current].push_str(&tail);
        self.caret = Caret {
            line: current,
            col: new_col,
        };
        self.edited(line);
    }

    pub fn backspace(&mut self) {
        let Caret { line, col } = self.caret;
        if col > 0 {
            let text = &mut self.lines[line];
            let range = byte_index(text, col - 1)..byte_index(text, col);
            text.replace_range(range, "");
            self.caret.col -= 1;
            self.edited(line);
        } else if line > 0 {
            let removed = self.lines.remove(line);
            let previous_chars = self.line_chars(line - 1);
            self.lines[line - 1].push_str(&removed);
            self.caret = Caret {
                line: line - 1,
                col: previous_chars,
            };
            self.edited(line - 1);
        }
    }

    pub fn delete(&mut self) {
        let Caret { line, col } = self.caret;
        if col < self.line_chars(line) {
            let text = &mut self.lines[line];
            let range = byte_index(text, col)..byte_index(text, col + 1);
            text.replace_range(range, "");
            self.edited(line);
        } else if line + 1 < self.lines.len() {
            let next = self.lines.remove(line + 1);
            self.lines[line].push_str(&next);
            self.edited(line);
        }
    }

    pub fn move_left(&mut self) {
        if self.caret.col > 0 {
            self.caret.col -= 1;
        } else if self.caret.line > 0 {
            self.caret.line -= 1;
            self.caret.col = self.line_chars(self.caret.line);
        }
    }

    pub fn move_right(&mut self) {
        if self.caret.col < self.line_chars(self.caret.line) {
            self.caret.col += 1;
        } else if self.caret.line + 1 < self.lines.len() {
            self.caret = Caret {
                line: self.caret.line + 1,
                col: 0,
            };
        }
    }

    /// shortcut: keeps the char column, not the visual x; wide chars make it drift.
    pub fn move_vertical(&mut self, delta: isize) {
        let line = self
            .caret
            .line
            .saturating_add_signed(delta)
            .min(self.lines.len() - 1);
        self.caret = Caret {
            line,
            col: self.caret.col.min(self.line_chars(line)),
        };
    }

    pub fn move_to_end(&mut self) {
        let line = self.lines.len() - 1;
        self.caret = Caret {
            line,
            col: self.line_chars(line),
        };
    }
}

/// Reads and splits the file on a background thread; the UI polls the receiver each frame.
pub fn spawn_load(path: PathBuf, ctx: egui::Context) -> Receiver<LoadMessage> {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let result = read_document(&path, &sender, &ctx);
        if sender.send(LoadMessage::Done(result)).is_ok() {
            ctx.request_repaint();
        }
        // Otherwise the tab is gone and nobody waits for this file.
    });
    receiver
}

fn read_document(
    path: &Path,
    sender: &Sender<LoadMessage>,
    ctx: &egui::Context,
) -> Result<Box<Document>, String> {
    let started = Instant::now();
    let mut file = File::open(path).map_err(|error| error.to_string())?;
    let total = file.metadata().map_err(|error| error.to_string())?.len();
    let mut bytes = Vec::with_capacity(usize::try_from(total).unwrap_or(0));
    let mut chunk = vec![0_u8; READ_CHUNK_BYTES];
    loop {
        let read = file.read(&mut chunk).map_err(|error| error.to_string())?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..read]);
        if total > 0 {
            let progress = bytes.len() as f32 / total as f32;
            if sender.send(LoadMessage::Progress(progress)).is_err() {
                return Err("load cancelled".to_owned());
            }
            ctx.request_repaint();
        }
    }
    let text = String::from_utf8(bytes).map_err(|error| {
        format!(
            "not valid UTF-8 (first invalid byte at offset {}); skipped",
            error.utf8_error().valid_up_to()
        )
    })?;
    let degraded = total > LARGE_FILE_BYTES;
    let mut doc = Document::from_text(Some(path.to_path_buf()), &text, degraded);
    doc.load_ms = started.elapsed().as_secs_f64() * 1000.0;
    Ok(Box::new(doc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_split_and_join_lines() {
        let mut doc = Document::from_text(None, "añb\r\ncd", false);
        assert!(doc.crlf);
        doc.caret = Caret { line: 0, col: 2 };
        doc.insert_text("X\nY");
        assert_eq!(doc.lines, ["añX", "Yb", "cd"]);
        assert_eq!(doc.caret, Caret { line: 1, col: 1 });
        doc.caret = Caret { line: 1, col: 0 };
        doc.backspace();
        assert_eq!(doc.lines, ["añXYb", "cd"]);
        assert_eq!(doc.caret, Caret { line: 0, col: 3 });
        doc.caret = Caret { line: 0, col: 5 };
        doc.delete();
        assert_eq!(doc.lines, ["añXYbcd"]);
        doc.caret = Caret { line: 0, col: 1 };
        doc.delete();
        assert_eq!(doc.lines, ["aXYbcd"]);
        assert!(doc.modified);
    }
}
