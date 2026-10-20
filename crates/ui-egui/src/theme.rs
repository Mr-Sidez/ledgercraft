//! LedgerCraft's own palette. Colours are defined here and nowhere else.

use eframe::egui::{self, Color32};

pub(crate) const ACCENT: Color32 = Color32::from_rgb(0x1f, 0x6f, 0x8b);
pub(crate) const ACCENT_SOFT: Color32 = Color32::from_rgb(0xd6, 0xea, 0xf1);
pub(crate) const NEGATIVE: Color32 = Color32::from_rgb(0xb0, 0x30, 0x30);
pub(crate) const POSITIVE: Color32 = Color32::from_rgb(0x2e, 0x7d, 0x4f);
pub(crate) const MUTED: Color32 = Color32::from_rgb(0x6b, 0x72, 0x80);
pub(crate) const NAV_BG: Color32 = Color32::from_rgb(0x1d, 0x2b, 0x36);
pub(crate) const NAV_TEXT: Color32 = Color32::from_rgb(0xe8, 0xee, 0xf2);

pub(crate) fn apply(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let mut v = egui::Visuals::light();
    v.selection.bg_fill = ACCENT_SOFT;
    v.selection.stroke.color = ACCENT;
    v.hyperlink_color = ACCENT;
    style.visuals = v;
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 4.0);
    ctx.set_style(style);
}
