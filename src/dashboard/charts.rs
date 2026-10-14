//! Small native chart primitives; all drawing scales to the measured bounds.
use gpui_kit::*;
use std::f32::consts::{FRAC_PI_2, TAU};

use super::theme;

pub(super) fn bars(samples: &[u8], color: u32) -> Div {
    div()
        .w_full()
        .h(px(34.))
        .rounded(px(10.))
        .overflow_hidden()
        .bg(theme::tint(color, 0.08))
        .px(px(7.))
        .child(
            div()
                .size_full()
                .flex()
                .items_end()
                .gap(px(2.4))
                .children(samples.iter().map(|sample| {
                    div()
                        .flex_1()
                        .min_w(px(0.))
                        .h(px(f32::from(*sample).min(100.) * 0.30))
                        .rounded_t(px(2.))
                        .bg(rgb(color))
                })),
        )
}

/// Draw normalized segments, with a small surface-colored gap between them.
/// Empty and non-positive data intentionally produce the neutral track.
pub(super) fn donut(segments: Vec<(f32, u32)>) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let center = bounds.center();
            let radius = bounds.size.width.min(bounds.size.height) / 2. - px(8.);
            let mut track = PathBuilder::stroke(px(14.));
            for step in 0..=128 {
                let angle = step as f32 / 128. * TAU;
                let position = center + point(radius * angle.cos(), radius * angle.sin());
                if step == 0 {
                    track.move_to(position);
                } else {
                    track.line_to(position);
                }
            }
            if let Ok(path) = track.build() {
                window.paint_path(path, rgb(theme::TRACK));
            }
            let total: f32 = segments.iter().map(|(amount, _)| amount.max(0.)).sum();
            if total <= 0. || !total.is_finite() {
                return;
            }
            let mut start = -FRAC_PI_2;
            for (amount, color) in segments {
                let sweep = amount.max(0.) / total * TAU;
                if sweep > 0.025 {
                    let mut arc = PathBuilder::stroke(px(14.));
                    for step in 0..=64 {
                        let angle = start + 0.0125 + (sweep - 0.025) * step as f32 / 64.;
                        let position = center + point(radius * angle.cos(), radius * angle.sin());
                        if step == 0 {
                            arc.move_to(position);
                        } else {
                            arc.line_to(position);
                        }
                    }
                    if let Ok(path) = arc.build() {
                        window.paint_path(path, rgb(color));
                    }
                }
                start += sweep;
            }
        },
    )
    .size_full()
}
