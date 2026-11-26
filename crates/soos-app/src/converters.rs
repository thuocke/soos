//! The converters page: the user's own units, one per line, kept apart
//! from the documents. It reads like a document: what's typed on the left,
//! what each line means on the right.

use std::time::Instant;

use eframe::egui::{self, text::LayoutJob};

use crate::editor::{
    caret_line, error_budget, layout_job, COLUMN_GAP, ERROR_HOLD, GUTTER_PAD, MAX_GUTTER_FRACTION,
    MIN_GUTTER,
};
use crate::style::{mono, status_text, Palette, CHAR_WIDTH, LINE_HEIGHT};
use crate::window::{modal_room, MODAL_GAP};

pub(crate) const CONVERTERS_PAGE_WIDTH: f32 = 520.0;
/// Shown in an empty page, and the shape every line follows.
const PLACEHOLDER: &str = "1 lap (laps) = 400 m";
const CAPTION: &str = "One unit per line. Esc closes.";
/// The page is at least this many lines tall, so an empty one still looks
/// like somewhere to type.
const MIN_LINES: usize = 5;

/// The converters page: `text` to edit, and beside each line what the unit
/// is worth, or why it can't be used. An error on the line being typed
/// waits [`ERROR_HOLD`], as in a document.
pub(crate) fn converters_page(
    ui: &mut egui::Ui,
    palette: Palette,
    text: &mut String,
    statuses: &[Option<Result<String, String>>],
    decimal_comma: bool,
) {
    let id = egui::Id::new("converters-text");
    let shown: Vec<Option<Result<String, &str>>> = statuses
        .iter()
        .map(|status| match status.as_ref()? {
            Ok(worth) => Some(Ok(crate::in_decimal_mark(worth, decimal_comma))),
            Err(why) => Some(Err(why.as_str())),
        })
        .collect();

    let caption_height = LINE_HEIGHT + MODAL_GAP;
    egui::ScrollArea::vertical()
        .max_height((modal_room(ui.ctx()).y - caption_height).max(LINE_HEIGHT * 2.0))
        .show(ui, |ui| {
            let available_width = ui.available_width();
            // Sized by the values; an error may run left into the room its
            // line leaves.
            let longest = shown
                .iter()
                .filter_map(|s| s.as_ref()?.as_ref().ok())
                .map(|worth| worth.chars().count())
                .max()
                .unwrap_or(0);
            let gutter_width = (longest as f32 * CHAR_WIDTH + GUTTER_PAD)
                .clamp(MIN_GUTTER, available_width * MAX_GUTTER_FRACTION);
            let text_width = (available_width - gutter_width - COLUMN_GAP).max(100.0);
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let job = layout_job(ui.ctx(), buf.as_str(), wrap_width, palette);
                ui.fonts_mut(|f| f.layout_job(job))
            };
            let output = egui::TextEdit::multiline(text)
                .id(id)
                .frame(egui::Frame::NONE)
                .desired_width(text_width)
                .desired_rows(MIN_LINES)
                .font(mono())
                .layouter(&mut layouter)
                .show(ui);
            // Nothing else here takes the keyboard.
            output.response.request_focus();

            if text.is_empty() {
                let hint =
                    LayoutJob::simple_singleline(PLACEHOLDER.to_owned(), mono(), palette.comment);
                let galley = ui.fonts_mut(|f| f.layout_job(hint));
                ui.painter()
                    .galley(output.galley_pos, galley, palette.comment);
            }

            if output.response.changed() {
                ui.data_mut(|d| d.insert_temp(id, Instant::now()));
            }
            let typing_line = ui
                .data(|d| d.get_temp::<Instant>(id))
                .and_then(|at| ERROR_HOLD.checked_sub(at.elapsed()))
                .filter(|left| !left.is_zero())
                .and_then(|left| {
                    ui.ctx().request_repaint_after(left);
                    caret_line(text, output.cursor_range)
                });

            let gutter_right =
                output.response.rect.right() + COLUMN_GAP + gutter_width - GUTTER_PAD;
            let gutter_budget = gutter_width - GUTTER_PAD;
            let mut line = 0usize;
            let mut starts_line = true;
            for row in &output.galley.rows {
                let this_line = line;
                let first_row = std::mem::replace(&mut starts_line, row.ends_with_newline);
                if row.ends_with_newline {
                    line += 1;
                }
                let Some(Some(status)) = shown.get(this_line).filter(|_| first_row) else {
                    continue;
                };
                let row_rect = row.rect().translate(output.galley_pos.to_vec2());
                let (label, color, budget) = match status {
                    Ok(worth) => (worth.as_str(), palette.result, gutter_budget),
                    Err(_) if typing_line == Some(this_line) => continue,
                    Err(why) => (
                        *why,
                        palette.error,
                        error_budget(gutter_right, row_rect.right(), gutter_budget),
                    ),
                };
                let mut job = LayoutJob::simple(label.to_owned(), mono(), color, f32::INFINITY);
                job.wrap = egui::text::TextWrapping::truncate_at_width(budget);
                let galley = ui.fonts_mut(|f| f.layout_job(job));
                let size = galley.rect.size();
                let pos = egui::pos2(gutter_right - size.x, row_rect.top());
                let elided = galley.elided;
                ui.painter().galley(pos, galley, color);
                if elided {
                    ui.allocate_rect(egui::Rect::from_min_size(pos, size), egui::Sense::hover())
                        .on_hover_text(label);
                }
            }
        });

    ui.add_space(MODAL_GAP);
    status_text(ui, CAPTION, palette.comment);
}
