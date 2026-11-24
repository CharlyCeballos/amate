//! The virtualized editor view: only the lines on screen are laid out and painted.

use std::ops::Range;
use std::sync::Arc;
use std::time::{Duration, Instant};

use eframe::egui;
use egui::emath::TSTransform;
use egui::text::{CCursor, CCursorRange, LayoutJob, TextFormat};
use egui::{
    Align, Align2, Color32, EventFilter, FontId, Galley, Id, ImeEvent, Key, Pos2, Rect, Sense,
    Stroke, Vec2,
};
use regex::Regex;

use crate::document::{Caret, Document, byte_index};
use crate::highlight::Spans;

const FONT_SIZE: f32 = 14.0;
const GUTTER_PADDING: f32 = 16.0;
const TEXT_MARGIN: f32 = 6.0;
const CARET_WIDTH: f32 = 2.0;
const MIN_GUTTER_DIGITS: usize = 3;
/// Characters kept visible right of the caret when scrolling it into view.
const REVEAL_MARGIN_CHARS: f32 = 4.0;
/// Highlighting work allowed per frame; the rest continues on the next frames.
const HIGHLIGHT_BUDGET: Duration = Duration::from_millis(8);
/// Up to this many lines the screen reader gets the whole text; above it, only the visible lines.
const A11Y_FULL_TEXT_MAX_LINES: usize = 1000;
const TAB_AS_SPACES: &str = "    ";
const MATCH_FILL: Color32 = Color32::from_rgba_premultiplied(90, 80, 0, 90);
const CURRENT_MATCH_FILL: Color32 = Color32::from_rgba_premultiplied(160, 90, 0, 160);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reveal {
    Nearest,
    Center,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ViewOutput {
    /// Every visible line was painted with its final colors.
    pub complete: bool,
    pub caret_visible: bool,
}

pub struct FindHighlight<'a> {
    pub regex: Option<&'a Regex>,
    pub current: Option<(usize, Range<usize>)>,
}

struct Metrics {
    font_id: FontId,
    line_height: f32,
    text_color: Color32,
}

pub fn show(
    ui: &mut egui::Ui,
    id: Id,
    doc: &mut Document,
    find: &FindHighlight<'_>,
    request_focus: bool,
) -> ViewOutput {
    let font_id = FontId::monospace(FONT_SIZE);
    let (line_height, char_width) =
        ui.fonts_mut(|fonts| (fonts.row_height(&font_id), fonts.glyph_width(&font_id, 'M')));
    let metrics = Metrics {
        font_id,
        line_height,
        text_color: ui.visuals().text_color(),
    };

    // Input first, so this frame already paints the edit.
    let has_focus = ui.memory(|memory| memory.has_focus(id));
    if has_focus {
        let page_lines = (ui.available_height() / line_height).floor().max(1.0) as usize;
        if handle_input(ui, doc, page_lines) {
            doc.reveal.get_or_insert(Reveal::Nearest);
        }
    }

    let line_count = doc.lines.len();
    let digits = line_count.to_string().len().max(MIN_GUTTER_DIGITS);
    let gutter_width = digits as f32 * char_width + GUTTER_PADDING;
    let content_size = Vec2::new(
        gutter_width + TEXT_MARGIN + (doc.max_line_chars as f32 + REVEAL_MARGIN_CHARS) * char_width,
        line_count as f32 * line_height,
    );

    egui::ScrollArea::both()
        .id_salt(id)
        .auto_shrink(false)
        .show_viewport(ui, |ui, viewport| {
            let (_, rect) = ui.allocate_space(content_size.max(viewport.size()));
            let response = ui.interact(rect, id, Sense::click_and_drag());
            if request_focus || response.clicked() || response.drag_started() {
                response.request_focus();
            }
            if response.has_focus() {
                ui.memory_mut(|memory| {
                    memory.set_focus_lock_filter(
                        id,
                        EventFilter {
                            tab: true,
                            horizontal_arrows: true,
                            vertical_arrows: true,
                            escape: false,
                        },
                    );
                });
            }

            let text_left = rect.min.x + gutter_width + TEXT_MARGIN;
            if let Some(pos) = response
                .interact_pointer_pos()
                .filter(|_| response.clicked() || response.dragged())
            {
                let line =
                    (((pos.y - rect.min.y) / line_height).max(0.0) as usize).min(line_count - 1);
                let galley = layout_line(ui, &doc.lines[line], None, None, &metrics);
                let cursor =
                    galley.cursor_from_pos(Vec2::new(pos.x - text_left, line_height / 2.0));
                doc.caret = Caret {
                    line,
                    col: cursor.index.0,
                };
                doc.preedit.clear();
            }

            let first = ((viewport.min.y / line_height).max(0.0) as usize).min(line_count);
            let end = ((viewport.max.y / line_height).ceil() as usize + 1).min(line_count);
            let complete = match &mut doc.highlighter {
                Some(highlighter) => {
                    let done = highlighter.prepare(
                        &doc.lines,
                        first..end,
                        Instant::now() + HIGHLIGHT_BUDGET,
                    );
                    if !done {
                        ui.ctx().request_repaint();
                    }
                    done
                }
                None => true,
            };

            let caret_x = paint_lines(ui, doc, find, &metrics, rect.min, text_left, first..end)
                .unwrap_or_else(|| {
                    let line = &doc.lines[doc.caret.line];
                    let galley = layout_line(ui, line, None, None, &metrics);
                    galley.pos_from_cursor(CCursor::new(doc.caret.col)).min.x
                });
            let caret_rect = Rect::from_min_size(
                Pos2::new(
                    text_left + caret_x,
                    rect.min.y + doc.caret.line as f32 * line_height,
                ),
                Vec2::new(CARET_WIDTH, line_height),
            );
            if response.has_focus() {
                ui.painter()
                    .rect_filled(caret_rect, 0.0, ui.visuals().strong_text_color());
            }

            paint_gutter(ui, &metrics, rect.min, viewport, gutter_width, first..end);

            if let Some(reveal) = doc.reveal.take() {
                let margin = REVEAL_MARGIN_CHARS * char_width;
                let target = Rect::from_min_max(
                    caret_rect.min - Vec2::new(gutter_width + TEXT_MARGIN + margin, 0.0),
                    caret_rect.max + Vec2::new(margin, 0.0),
                );
                let align = (reveal == Reveal::Center).then_some(Align::Center);
                ui.scroll_to_rect(target, align);
            }

            let visible_rect = viewport.translate(rect.min.to_vec2());
            if response.has_focus() {
                set_ime_area(ui, visible_rect, caret_rect);
            }
            let a11y = A11yView {
                id,
                visible_rect,
                text_origin: Pos2::new(text_left, rect.min.y),
                visible: first..end,
            };
            update_accessibility(ui, doc, &metrics, &a11y);

            ViewOutput {
                complete,
                caret_visible: visible_rect.contains_rect(caret_rect),
            }
        })
        .inner
}

/// Returns true when the text or the caret changed.
fn handle_input(ui: &egui::Ui, doc: &mut Document, page_lines: usize) -> bool {
    let events = ui.input(|input| input.events.clone());
    let mut changed = false;
    for event in events {
        changed |= match event {
            egui::Event::Text(text)
                if doc.preedit.is_empty() && !text.chars().any(char::is_control) =>
            {
                doc.insert_text(&text);
                true
            }
            egui::Event::Paste(text) => {
                doc.insert_text(&text);
                true
            }
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } => handle_key(doc, key, modifiers, page_lines),
            egui::Event::Ime(ImeEvent::Preedit { text, .. }) => {
                doc.preedit = text;
                true
            }
            egui::Event::Ime(ImeEvent::Commit(text)) => {
                doc.preedit.clear();
                if !text.is_empty() && text != "\n" && text != "\r" {
                    doc.insert_text(&text);
                }
                true
            }
            _ => false,
        };
    }
    changed
}

fn handle_key(doc: &mut Document, key: Key, modifiers: egui::Modifiers, page_lines: usize) -> bool {
    let page = isize::try_from(page_lines).unwrap_or(isize::MAX);
    match key {
        Key::ArrowLeft => doc.move_left(),
        Key::ArrowRight => doc.move_right(),
        Key::ArrowUp => doc.move_vertical(-1),
        Key::ArrowDown => doc.move_vertical(1),
        Key::PageUp => doc.move_vertical(-page),
        Key::PageDown => doc.move_vertical(page),
        Key::Home if modifiers.command => doc.caret = Caret::default(),
        Key::End if modifiers.command => doc.move_to_end(),
        Key::Home => doc.caret.col = 0,
        Key::End => doc.caret.col = doc.line_chars(doc.caret.line),
        Key::Backspace => doc.backspace(),
        Key::Delete => doc.delete(),
        Key::Enter => doc.insert_text("\n"),
        // shortcut: Tab inserts spaces; no tab-width setting in the spike.
        Key::Tab => doc.insert_text(TAB_AS_SPACES),
        _ => return false,
    }
    true
}

/// Paints the visible lines and find matches; returns the caret's x if its line is visible.
fn paint_lines(
    ui: &egui::Ui,
    doc: &Document,
    find: &FindHighlight<'_>,
    metrics: &Metrics,
    origin: Pos2,
    text_left: f32,
    visible: Range<usize>,
) -> Option<f32> {
    let painter = ui.painter();
    let mut caret_x = None;
    for (index, line) in doc
        .lines
        .iter()
        .enumerate()
        .take(visible.end)
        .skip(visible.start)
    {
        let top = origin.y + index as f32 * metrics.line_height;
        let is_caret_line = index == doc.caret.line;
        let preedit = (is_caret_line && !doc.preedit.is_empty())
            .then_some((doc.caret.col, doc.preedit.as_str()));
        let spans = doc
            .highlighter
            .as_ref()
            .and_then(|highlighter| highlighter.spans(index));
        let galley = layout_line(ui, line, spans, preedit, metrics);

        if let Some(regex) = find.regex {
            for found in regex.find_iter(line) {
                let x_at = |byte: usize| {
                    let col = line[..byte].chars().count();
                    galley.pos_from_cursor(CCursor::new(col)).min.x
                };
                let (start_x, end_x) = (x_at(found.start()), x_at(found.end()));
                let is_current = find.current.as_ref() == Some(&(index, found.range()));
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(text_left + start_x, top),
                        Pos2::new(
                            text_left + end_x.max(start_x + CARET_WIDTH),
                            top + metrics.line_height,
                        ),
                    ),
                    0.0,
                    if is_current {
                        CURRENT_MATCH_FILL
                    } else {
                        MATCH_FILL
                    },
                );
            }
        }
        if is_caret_line {
            let preedit_chars = doc.preedit.chars().count();
            caret_x = Some(
                galley
                    .pos_from_cursor(CCursor::new(doc.caret.col + preedit_chars))
                    .min
                    .x,
            );
        }
        painter.galley(Pos2::new(text_left, top), galley, metrics.text_color);
    }
    caret_x
}

/// One galley per line; egui's galley cache makes unchanged lines a hash lookup.
fn layout_line(
    ui: &egui::Ui,
    line: &str,
    spans: Option<&Spans>,
    preedit: Option<(usize, &str)>,
    metrics: &Metrics,
) -> Arc<Galley> {
    let format = |color| TextFormat::simple(metrics.font_id.clone(), color);
    let plain = metrics.text_color;
    let mut job = LayoutJob::default();
    match (preedit, spans) {
        (Some((col, composition)), _) => {
            // shortcut: the line being composed loses its colors until the IME commits.
            let at = byte_index(line, col);
            job.append(&line[..at], 0.0, format(plain));
            job.append(
                composition,
                0.0,
                TextFormat {
                    underline: Stroke::new(1.0, plain),
                    ..format(plain)
                },
            );
            job.append(&line[at..], 0.0, format(plain));
        }
        (None, Some(spans))
            if spans
                .iter()
                .all(|(range, _)| line.get(range.clone()).is_some()) =>
        {
            let mut position = 0;
            for (range, color) in spans {
                if range.start > position {
                    job.append(&line[position..range.start], 0.0, format(plain));
                }
                job.append(&line[range.clone()], 0.0, format(*color));
                position = range.end;
            }
            if position < line.len() {
                job.append(&line[position..], 0.0, format(plain));
            }
        }
        _ => job.append(line, 0.0, format(plain)),
    }
    ui.painter().layout_job(job)
}

/// Line numbers stay pinned to the left edge while the text scrolls horizontally under them.
fn paint_gutter(
    ui: &egui::Ui,
    metrics: &Metrics,
    origin: Pos2,
    viewport: Rect,
    gutter_width: f32,
    visible: Range<usize>,
) {
    let painter = ui.painter();
    let gutter = Rect::from_min_size(
        origin + viewport.min.to_vec2(),
        Vec2::new(gutter_width, viewport.height()),
    );
    painter.rect_filled(gutter, 0.0, ui.visuals().extreme_bg_color);
    let number_color = ui.visuals().weak_text_color();
    for index in visible {
        painter.text(
            Pos2::new(
                gutter.max.x - GUTTER_PADDING / 2.0,
                origin.y + index as f32 * metrics.line_height,
            ),
            Align2::RIGHT_TOP,
            (index + 1).to_string(),
            metrics.font_id.clone(),
            number_color,
        );
    }
}

/// Tells the platform IME where the caret is, so candidate windows open next to it.
fn set_ime_area(ui: &egui::Ui, visible_rect: Rect, caret_rect: Rect) {
    let to_global = ui
        .ctx()
        .layer_transform_to_global(ui.layer_id())
        .unwrap_or_default();
    ui.output_mut(|output| {
        output.ime = Some(egui::output::IMEOutput {
            purpose: egui::IMEPurpose::Normal,
            rect: to_global * visible_rect,
            cursor_rect: to_global * caret_rect,
            should_interrupt_composition: false,
        });
    });
}

struct A11yView {
    id: Id,
    visible_rect: Rect,
    text_origin: Pos2,
    visible: Range<usize>,
}

/// Exposes the editor to screen readers as a multiline text input with text runs and a caret.
fn update_accessibility(ui: &egui::Ui, doc: &Document, metrics: &Metrics, view: &A11yView) {
    let ctx = ui.ctx();
    let label = format!("Editor: {}", doc.display_name());
    let active = ctx
        .accesskit_node_builder(view.id, |node| {
            node.set_role(egui::accesskit::Role::MultilineTextInput);
            node.set_label(label);
            node.set_bounds(egui::accesskit::Rect {
                x0: view.visible_rect.min.x.into(),
                y0: view.visible_rect.min.y.into(),
                x1: view.visible_rect.max.x.into(),
                y1: view.visible_rect.max.y.into(),
            });
        })
        .is_some();
    if !active {
        return; // No assistive technology connected: skip the extra layout.
    }
    let window = if doc.lines.len() <= A11Y_FULL_TEXT_MAX_LINES {
        0..doc.lines.len()
    } else {
        view.visible.clone()
    };
    let text = doc.lines[window.clone()].join("\n");
    let caret = window.contains(&doc.caret.line).then(|| {
        let before: usize = doc.lines[window.start..doc.caret.line]
            .iter()
            .map(|line| line.chars().count() + 1)
            .sum();
        CCursorRange::one(CCursor::new(before + doc.caret.col))
    });
    ctx.accesskit_node_builder(view.id, |node| node.set_value(text.clone()));
    let galley = ui.painter().layout_job(LayoutJob::simple(
        text,
        metrics.font_id.clone(),
        metrics.text_color,
        f32::INFINITY,
    ));
    let origin = Vec2::new(
        view.text_origin.x,
        view.text_origin.y + window.start as f32 * metrics.line_height,
    );
    egui::text_selection::accesskit_text::update_accesskit_for_text_widget(
        ctx,
        view.id,
        caret,
        TSTransform::from_translation(origin),
        &galley,
    );
}
