//! The status bar: the version, the exchange-rate status, one message slot
//! in the middle, and the toggles on the right.

use std::time::Duration;

use eframe::egui::text::LayoutJob;
use eframe::egui::{self, Color32, ViewportCommand};

use crate::editor::APP_PADDING;
use crate::style::{cap_center_offset, mono, status_symbol, status_text, Palette, LINE_HEIGHT};
use crate::tray::FULL_DESKTOP;
use crate::update::{open_url, update_message};
use crate::SoosApp;

/// How long "Copied ..." stays in the message slot.
const COPIED_FOR: Duration = Duration::from_millis(1500);
/// Between the words on the right of the status bar.
const STATUS_WORD_GAP: f32 = 16.0;
/// The settings panel's rows: side padding, corner radius, and how far
/// hover or focus tints a row toward the text colour.
const SETTING_ROW_PADDING: f32 = 10.0;
const SETTING_ROW_RADIUS: f32 = 4.0;
const SETTING_ROW_TINT: f32 = 0.08;
/// Rates older than this get an age warning.
const RATES_OLD_AFTER: Duration = Duration::from_secs(24 * 60 * 60);

/// The message in the middle of the status bar.
pub(crate) struct StatusMessage {
    pub(crate) text: String,
    pub(crate) color: Color32,
    /// Clicking the message opens this.
    pub(crate) url: Option<String>,
}

impl SoosApp {
    pub(crate) fn status_bar(&mut self, ui: &mut egui::Ui, palette: Palette) {
        #[cfg(not(target_os = "linux"))]
        if self.capturing_hotkey {
            self.handle_hotkey_capture(ui.ctx());
        }
        egui::Panel::bottom("status")
            .frame(
                // The page's side margin, so the bar lines up with the
                // editor, and its bottom margin below the bar.
                egui::Frame::new()
                    .inner_margin(egui::Margin {
                        left: APP_PADDING,
                        right: APP_PADDING,
                        top: 2,
                        bottom: 2 + APP_PADDING,
                    })
                    .fill(palette.background),
            )
            .show_separator_line(false)
            .show(ui, |ui| {
                let mut free = (0.0, 0.0);
                let row = ui
                    .horizontal(|ui| {
                        let version = status_symbol(
                            ui,
                            concat!("v", env!("CARGO_PKG_VERSION")),
                            palette.comment,
                            palette,
                        );
                        if version.on_hover_text("Check for updates").clicked() {
                            self.check_for_update(ui.ctx());
                        }
                        self.rates_status(ui, palette);
                        let left_end = ui.min_rect().right();
                        // Right to left, so this reads "example 0.1 settings".
                        let icons = ui
                            .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.spacing_mut().item_spacing.x = STATUS_WORD_GAP;
                                self.settings_control(ui, palette);
                                self.decimal_comma_control(ui, palette);
                                self.example_control(ui, palette);
                            })
                            .response
                            .rect;
                        free = (left_end, icons.left());
                    })
                    .response
                    .rect;
                if let Some(message) = self.status_message(ui.ctx(), palette) {
                    paint_message(ui, row, free, message);
                }
            });
    }

    /// "rates offline" with no rates at all, or the rates' age once they're
    /// a day old; nothing while they're current.
    fn rates_status(&self, ui: &mut egui::Ui, palette: Palette) {
        let (text, tooltip) = match self.rates.age() {
            None => (
                "rates offline".to_string(),
                "Exchange rates haven't been downloaded yet. Soos keeps trying while it's open."
                    .to_string(),
            ),
            Some(age) if age > RATES_OLD_AFTER => {
                let days = age.as_secs() / (24 * 60 * 60);
                let days = if days == 1 {
                    "1 day".to_string()
                } else {
                    format!("{days} days")
                };
                (
                    format!("rates {days} old"),
                    format!("Currency results use exchange rates from {days} ago."),
                )
            }
            Some(_) => return,
        };
        status_text(ui, &text, palette.comment).on_hover_text(tooltip);
    }

    /// A hotkey prompt or error first, then a failed converters export, a
    /// recent copy, the result of an update check, and last the launch
    /// warning that the saved tabs couldn't be read.
    fn status_message(&self, ctx: &egui::Context, palette: Palette) -> Option<StatusMessage> {
        let plain = |text: String, color: Color32| StatusMessage {
            text,
            color,
            url: None,
        };
        if let Some(message) = &self.hotkey_message {
            return Some(plain(message.clone(), palette.error));
        }
        if self.capturing_hotkey {
            return Some(plain(
                "Press the new hotkey, or Esc to cancel".to_string(),
                palette.plain,
            ));
        }
        if let Some(error) = &self.export_error {
            return Some(plain(error.clone(), palette.error));
        }
        if let Some((copied, at)) = &self.copied {
            if let Some(left) = COPIED_FOR.checked_sub(at.elapsed()) {
                ctx.request_repaint_after(left);
                return Some(plain(format!("Copied {copied}"), palette.comment));
            }
        }
        self.update_status
            .as_ref()
            .map(|status| update_message(status, palette))
            .or_else(|| {
                let warning = self.load_warning.clone()?;
                Some(plain(warning, palette.error))
            })
    }

    /// The label shows how a decimal reads now, so its state isn't told by
    /// colour. Recalculates, since the
    /// text sent to the engine changes with it, though the text itself
    /// doesn't.
    fn decimal_comma_control(&mut self, ui: &mut egui::Ui, palette: Palette) {
        let (glyph, tooltip) = if self.decimal_comma {
            (
                "0,1",
                "Decimal comma is on: 1.234,5. Click for a decimal point, 1,234.5",
            )
        } else {
            (
                "0.1",
                "Decimal point is on: 1,234.5. Click for a decimal comma, 1.234,5",
            )
        };
        if status_symbol(ui, glyph, palette.comment, palette)
            .on_hover_text(tooltip)
            .clicked()
        {
            self.decimal_comma = !self.decimal_comma;
            self.force_recalc();
        }
    }

    pub(crate) fn apply_always_on_top(&self, ctx: &egui::Context) {
        let level = if self.always_on_top {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        };
        ctx.send_viewport_cmd(ViewportCommand::WindowLevel(level));
    }

    fn example_control(&mut self, ui: &mut egui::Ui, palette: Palette) {
        let tooltip = if self.show_example {
            "Hide the example"
        } else {
            "Show an example of what Soos understands"
        };
        if status_symbol(ui, "example", palette.comment, palette)
            .on_hover_text(tooltip)
            .clicked()
        {
            self.show_example = !self.show_example;
            if self.show_example {
                // So the document doesn't take keystrokes behind the overlay.
                ui.ctx().memory_mut(|m| m.stop_text_input());
            }
        }
    }

    fn settings_control(&mut self, ui: &mut egui::Ui, palette: Palette) {
        if status_symbol(ui, "settings", palette.comment, palette)
            .on_hover_text("Theme, always on top, precision, hotkey and your converters")
            .clicked()
        {
            self.show_settings = true;
            // So the document doesn't take keystrokes behind the panel.
            ui.ctx().memory_mut(|m| m.stop_text_input());
        }
    }

    /// The settings panel: one row per setting, its name on the left and
    /// what it is now on the right, so no state is told by colour or by a
    /// symbol. A row is as wide as the panel and a line tall, takes Tab
    /// focus, and Enter or Space works like a click. Returns whether to
    /// close the panel.
    pub(crate) fn settings_panel(&mut self, ui: &mut egui::Ui, palette: Palette) -> bool {
        use egui::ThemePreference;
        let mut close = false;
        ui.spacing_mut().item_spacing.y = 0.0;

        let preference = ui.ctx().options(|o| o.theme_preference);
        let (theme, next) = match preference {
            ThemePreference::System => ("same as the system", ThemePreference::Light),
            ThemePreference::Light => ("light", ThemePreference::Dark),
            ThemePreference::Dark => ("dark", ThemePreference::System),
        };
        if setting_row(ui, palette, "Theme", theme).clicked() {
            ui.ctx().set_theme(next);
        }
        if FULL_DESKTOP
            && setting_row(ui, palette, "Always on top", on_off(self.always_on_top)).clicked()
        {
            self.always_on_top = !self.always_on_top;
            self.apply_always_on_top(ui.ctx());
        }
        let digits = if self.high_precision {
            "every digit"
        } else {
            "rounded"
        };
        if setting_row(ui, palette, "Currency amounts", digits).clicked() {
            self.high_precision = !self.high_precision;
        }
        #[cfg(not(target_os = "linux"))]
        {
            let hotkey = if self.hotkeys.is_some() {
                crate::hotkey::hotkey_label(&self.current_hotkey)
            } else {
                "not available here".to_string()
            };
            if setting_row(ui, palette, "Hotkey", &hotkey).clicked() && self.hotkeys.is_some() {
                self.capturing_hotkey = true;
                self.hotkey_message = None;
                close = true;
            }
        }
        let count = match self.converters.len() {
            0 => "none yet".to_string(),
            1 => "1 unit".to_string(),
            n => format!("{n} units"),
        };
        if setting_row(ui, palette, "Your converters", &count).clicked() {
            self.show_converters = true;
            self.focus_converter = Some(0);
            close = true;
        }
        close
    }
}

fn on_off(on: bool) -> &'static str {
    if on {
        "on"
    } else {
        "off"
    }
}

/// One row of the settings panel: `label` in plain text on the left, its
/// current `value` in the accent on the right. Hover and keyboard focus
/// tint the row, and focus also draws an outline.
fn setting_row(ui: &mut egui::Ui, palette: Palette, label: &str, value: &str) -> egui::Response {
    let size = egui::vec2(ui.available_width(), LINE_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let lit = ui
            .ctx()
            .animate_bool(response.id, response.hovered() || response.has_focus());
        let fill = palette
            .background
            .lerp_to_gamma(palette.plain, lit * SETTING_ROW_TINT);
        ui.painter().rect_filled(rect, SETTING_ROW_RADIUS, fill);
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect,
                SETTING_ROW_RADIUS,
                egui::Stroke::new(1.0, palette.keyword),
                egui::StrokeKind::Inside,
            );
        }
        let text = |text: &str, color: Color32| {
            ui.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), mono(), color))
        };
        let label_galley = text(label, palette.plain);
        let value_galley = text(value, palette.keyword);
        let y =
            |galley: &egui::Galley| rect.center().y - galley.size().y / 2.0 + cap_center_offset(ui);
        let value_pos = egui::pos2(
            rect.right() - SETTING_ROW_PADDING - value_galley.size().x,
            y(&value_galley),
        );
        let label_pos = egui::pos2(rect.left() + SETTING_ROW_PADDING, y(&label_galley));
        ui.painter().galley(label_pos, label_galley, palette.plain);
        ui.painter()
            .galley(value_pos, value_galley, palette.keyword);
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, format!("{label}: {value}"))
    });
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Paints `message` centred on `row`, within the free span `(left, right)`
/// between the version and the icons; a message too long for it is cut,
/// with the full text on hover. Painted over the row rather than laid out
/// in it, so the row doesn't change size as messages come and go.
fn paint_message(ui: &mut egui::Ui, row: egui::Rect, free: (f32, f32), message: StatusMessage) {
    const GAP: f32 = 16.0;
    let (left, right) = (free.0 + GAP, free.1 - GAP);
    let mut job = LayoutJob::simple(message.text.clone(), mono(), message.color, f32::INFINITY);
    job.wrap = egui::text::TextWrapping::truncate_at_width((right - left).max(0.0));
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    let width = galley.rect.width();
    let x = (row.center().x - width / 2.0).clamp(left, (right - width).max(left));
    let pos = egui::pos2(
        x,
        row.center().y - galley.rect.height() / 2.0 + cap_center_offset(ui),
    );
    let rect = egui::Rect::from_min_size(pos, galley.rect.size());
    let elided = galley.elided;
    ui.painter().galley(pos, galley, message.color);
    let sense = if message.url.is_some() {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let mut response = ui.interact(rect, ui.id().with("message"), sense);
    if elided {
        response = response.on_hover_text(&message.text);
    }
    if let Some(url) = message.url {
        let response = response
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text("Open the release page");
        if response.clicked() {
            open_url(&url);
        }
    }
}
