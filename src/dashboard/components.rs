//! Reusable card surfaces. Content comes from the typed presentation model.
use super::{
    charts,
    model::{Breakdown, CardChart, Metric},
    theme,
};
use gpui_kit::{assets::IconName, base::Button, component::Icon, prelude::FluentBuilder as _, *};

pub(super) fn icon(name: IconName, color: u32, size: f32) -> Icon {
    Icon::new(name).size(px(size)).text_color(rgb(color))
}

fn badge(name: IconName, color: u32) -> Div {
    div()
        .size(px(27.))
        .flex_shrink_0()
        .rounded(px(9.))
        .bg(theme::tint(color, 0.15))
        .flex()
        .items_center()
        .justify_center()
        .child(icon(name, color, 15.))
}

fn heading(name: IconName, title: &'static str, color: u32) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(10.))
        .child(badge(name, color))
        .child(
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(color))
                .child(title),
        )
}

pub(super) fn metric_card(metric: &Metric, header: Button) -> Div {
    let chart = match &metric.chart {
        CardChart::Bars(samples) => charts::bars(samples, metric.color).into_any_element(),
        CardChart::Providers => provider_usage().into_any_element(),
    };
    theme::surface()
        .h(px(228.))
        .flex()
        .flex_col()
        .justify_between()
        .child(
            header
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .accessibility_label(format!("Open {} details", metric.title))
                .child(heading(metric.section.icon(), metric.title, metric.color))
                .child(icon(IconName::ChevronRight, 0xa8a8af, 14.)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(3.))
                .child(
                    div()
                        .text_color(rgb(theme::MUTED))
                        .child(metric.caption.clone()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(4.))
                        .child(
                            div()
                                .flex()
                                .items_baseline()
                                .gap(px(8.))
                                .child(
                                    div()
                                        .text_size(px(34.))
                                        .line_height(relative(1.2))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(metric.value.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(17.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(theme::MUTED))
                                        .child(metric.unit),
                                ),
                        )
                        .when_some(metric.status, |this, status| {
                            this.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(5.))
                                    .px(px(10.))
                                    .py(px(4.))
                                    .rounded_full()
                                    .bg(theme::tint(theme::GREEN, 0.12))
                                    .text_color(rgb(theme::GREEN))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(icon(IconName::Check, theme::GREEN, 12.))
                                    .child(status),
                            )
                        }),
                ),
        )
        .child(
            div()
                .grid()
                .grid_cols(3)
                .gap(px(5.))
                .children(metric.statistics.iter().map(|stat| {
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(3.))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(5.))
                                .text_size(px(12.5))
                                .text_color(rgb(theme::MUTED))
                                .when_some(stat.color, |this, color| this.child(dot(color)))
                                .child(stat.label),
                        )
                        .child(
                            div()
                                .text_size(px(14.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(stat.value.clone()),
                        )
                })),
        )
        .child(chart)
}

fn dot(color: u32) -> Div {
    div()
        .size(px(7.))
        .flex_shrink_0()
        .rounded_full()
        .bg(rgb(color))
}

fn provider_usage() -> Div {
    let providers = [
        ("Claude", IconName::Sparkles, 0xe47758, 0.72),
        ("OpenAI", IconName::Aperture, 0x0ca585, 0.88),
        ("GitHub Copilot", IconName::Bot, 0x894ce5, 0.36),
        ("Cursor", IconName::Box, 0x202022, 0.48),
    ];
    div()
        .h(px(34.))
        .flex()
        .items_center()
        .gap(px(14.))
        .children(
            providers
                .into_iter()
                .map(|(name, symbol, color, remaining)| {
                    div()
                        .id(name)
                        .flex_1()
                        .flex()
                        .items_center()
                        .gap(px(8.))
                        .role(Role::Image)
                        .aria_label(format!("{name}: {:.0}% left (sample)", remaining * 100.))
                        .child(icon(symbol, theme::MUTED, 15.))
                        .child(
                            div()
                                .flex_1()
                                .h(px(5.))
                                .rounded_full()
                                .bg(theme::tint(color, 0.15))
                                .child(
                                    div()
                                        .w(relative(remaining))
                                        .h_full()
                                        .rounded_full()
                                        .bg(rgb(color)),
                                ),
                        )
                }),
        )
}

pub(super) fn breakdown_card(data: &Breakdown) -> Div {
    let segments = data
        .items
        .iter()
        .map(|item| (item.amount, item.color))
        .collect();
    theme::surface()
        .h(px(219.))
        .p(px(21.))
        .flex()
        .flex_col()
        .justify_between()
        .child(heading(data.icon, data.title, data.color))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(22.))
                .child(
                    div()
                        .relative()
                        .size(px(108.))
                        .flex_shrink_0()
                        .child(charts::donut(segments))
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_size(px(if data.center_value.len() > 5 {
                                            15.
                                        } else {
                                            19.
                                        }))
                                        .font_weight(FontWeight::BOLD)
                                        .child(data.center_value.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.5))
                                        .text_color(rgb(theme::MUTED))
                                        .child(data.center_caption),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.))
                        .flex()
                        .flex_col()
                        .gap(px(10.))
                        .children(data.items.iter().map(|item| {
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap(px(6.))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(8.))
                                        .child(dot(item.color))
                                        .when_some(item.icon, |this, name| {
                                            this.child(icon(name, item.color, 14.))
                                        })
                                        .child(item.label),
                                )
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .text_color(rgb(theme::MUTED))
                                        .child(item.display_value.clone()),
                                )
                        })),
                ),
        )
}
