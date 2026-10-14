//! Shared visual tokens for the reference's light, flat surfaces.
use gpui_kit::{Div, Hsla, Styled, div, px, rgb};

pub(super) const BACKGROUND: u32 = 0xffffff;
pub(super) const SURFACE: u32 = 0xf5f5f7;
pub(super) const INK: u32 = 0x202022;
pub(super) const MUTED: u32 = 0x7c7c82;
pub(super) const BLUE: u32 = 0x2d7fe9;
pub(super) const PURPLE: u32 = 0x5040b5;
pub(super) const PINK: u32 = 0xed70a5;
pub(super) const ORANGE: u32 = 0xf3a000;
pub(super) const TEAL: u32 = 0x15b386;
pub(super) const GREEN: u32 = 0x008b00;
pub(super) const VIOLET: u32 = 0x8b46ee;
pub(super) const TRACK: u32 = 0xe7e7ed;
pub(super) const GAP: f32 = 14.;
pub(super) const CARD_PADDING: f32 = 17.;
pub(super) const CARD_RADIUS: f32 = 22.;

pub(super) fn tint(color: u32, opacity: f32) -> Hsla {
    rgb(color).alpha(opacity).into()
}

pub(super) fn surface() -> Div {
    div()
        .bg(rgb(SURFACE))
        .rounded(px(CARD_RADIUS))
        .p(px(CARD_PADDING))
}
