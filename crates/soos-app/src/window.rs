//! Window sizes, and the shared frame for the example and converters
//! overlays.

use eframe::egui;

use crate::editor::{APP_PADDING, COLUMN_GAP, MIN_GUTTER};
use crate::example::EXAMPLE_COLUMN;
use crate::style::{Palette, CHAR_WIDTH};

/// Space between elements inside an overlay.
pub(crate) const MODAL_GAP: f32 = 12.0;
pub(crate) const MODAL_INNER_MARGIN: i8 = 18;
/// Space kept between an overlay and the window's edges.
const MODAL_WINDOW_MARGIN: f32 = 20.0;

pub(crate) const EXAMPLE_MODAL_WIDTH: f32 = 480.0;
pub(crate) const SETTINGS_MODAL_WIDTH: f32 = 420.0;

/// A scratchpad's width, not an editor's: results stay near the lines they
/// answer. Fits the overlays.
pub(crate) const DEFAULT_WIDTH: f32 = 560.0;
pub(crate) const DEFAULT_HEIGHT: f32 = 680.0;
/// A typical line ([`EXAMPLE_COLUMN`] characters) beside the narrowest
/// result column, plus the scrollbar.
pub(crate) const MAIN_MIN_WIDTH: f32 =
    APP_PADDING as f32 * 2.0 + CHAR_WIDTH * EXAMPLE_COLUMN as f32 + COLUMN_GAP + MIN_GUTTER + 20.0;
pub(crate) const MAIN_MIN_HEIGHT: f32 = 480.0;

/// How much of the window an overlay's body may use, after its margins.
pub(crate) fn modal_room(ctx: &egui::Context) -> egui::Vec2 {
    ctx.content_rect().size()
        - egui::Vec2::splat(2.0 * (MODAL_INNER_MARGIN as f32 + MODAL_WINDOW_MARGIN))
}

/// An overlay `width` wide, or narrower if the window is; its body scrolls
/// when it needs more room. The width doesn't follow the body, so the
/// overlay's edges don't move as its content changes.
pub(crate) fn soos_modal<R>(
    ui: &egui::Ui,
    palette: Palette,
    id: &str,
    width: f32,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::ModalResponse<R> {
    let width = width.min(modal_room(ui.ctx()).x).max(0.0);
    egui::Modal::new(egui::Id::new(id))
        .frame(
            egui::Frame::popup(&ui.style().clone())
                .fill(palette.background)
                .inner_margin(MODAL_INNER_MARGIN),
        )
        .backdrop_color(palette.backdrop)
        .show(ui.ctx(), |ui| {
            ui.set_width(width);
            body(ui)
        })
}
