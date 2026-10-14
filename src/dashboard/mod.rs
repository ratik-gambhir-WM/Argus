//! Dashboard composition and navigation. Charts, fixtures, and styling live in
//! focused modules so new cards do not duplicate layout or interaction code.
mod charts;
mod components;
mod model;
mod sample;
mod theme;

use gpui_kit::{base::Button, prelude::FluentBuilder as _, *};
use model::{DashboardData, Section};

pub(crate) struct Dashboard {
    selected: Section,
    data: DashboardData,
}

impl Dashboard {
    pub(crate) fn new() -> Self {
        Self {
            selected: Section::Overview,
            data: DashboardData::sample(),
        }
    }

    fn select(&mut self, section: Section, cx: &mut Context<Self>) {
        self.selected = section;
        cx.notify();
    }

    fn navigation(&self, cx: &mut Context<Self>) -> Div {
        div()
            .h(px(60.))
            .flex_shrink_0()
            .flex()
            .items_center()
            .pl(px(100.))
            .pr(px(18.))
            .child(
                div()
                    .id("navigation-scroll")
                    .overflow_x_scroll()
                    .rounded_full()
                    .bg(rgb(theme::SURFACE))
                    .p(px(5.))
                    .flex()
                    .items_center()
                    .gap(px(2.))
                    .children(
                        Section::ALL
                            .into_iter()
                            .enumerate()
                            .map(|(index, section)| {
                                let selected = self.selected == section;
                                Button::new(("section", index))
                                    .flex_shrink_0()
                                    .selected(selected)
                                    .accessibility_label(format!(
                                        "{}{}",
                                        section.label(),
                                        if selected { ", selected" } else { "" }
                                    ))
                                    .h(px(34.))
                                    .px(px(11.))
                                    .rounded_full()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.))
                                    .text_size(px(14.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(if selected {
                                        theme::BLUE
                                    } else {
                                        theme::MUTED
                                    }))
                                    .when(selected, |this| this.bg(theme::tint(theme::BLUE, 0.16)))
                                    .when(!selected, |this| {
                                        this.hover(|style| style.bg(theme::tint(theme::BLUE, 0.08)))
                                    })
                                    .focus_visible(|style| {
                                        style.border_2().border_color(rgb(theme::BLUE))
                                    })
                                    .on_click(
                                        cx.listener(move |view, _, _, cx| view.select(section, cx)),
                                    )
                                    .child(components::icon(
                                        section.icon(),
                                        if selected { theme::BLUE } else { theme::MUTED },
                                        14.,
                                    ))
                                    .child(section.label())
                            }),
                    ),
            )
    }

    fn content(&self, width: Pixels, cx: &mut Context<Self>) -> Stateful<Div> {
        let columns = if width >= px(1200.) {
            4
        } else if width >= px(640.) {
            2
        } else {
            1
        };
        let overview = self.selected == Section::Overview;
        let mut content = div()
            .id("dashboard-scroll")
            .overflow_y_scroll()
            .flex_1()
            .min_h(px(0.))
            .px(px(18.))
            .pt(px(10.))
            .pb(px(18.))
            .flex()
            .flex_col()
            .gap(px(theme::GAP));
        let metrics: Vec<_> = self
            .data
            .metrics
            .iter()
            .filter(|metric| overview || metric.section == self.selected)
            .collect();
        if !metrics.is_empty() {
            content = content.child(
                div()
                    .grid()
                    .grid_cols(if overview { columns } else { 1 })
                    .w_full()
                    .when(!overview, |this| this.max_w(px(560.)))
                    .gap(px(theme::GAP))
                    .children(metrics.into_iter().map(|metric| {
                        let section = metric.section;
                        let header = Button::new(section.label())
                            .rounded(px(5.))
                            .focus_visible(|style| style.border_2().border_color(rgb(theme::BLUE)))
                            .on_click(cx.listener(move |view, _, _, cx| view.select(section, cx)));
                        components::metric_card(metric, header)
                    })),
            );
        } else {
            content = content.child(
                theme::surface()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .h(px(250.))
                    .gap(px(12.))
                    .child(components::icon(self.selected.icon(), theme::BLUE, 28.))
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(self.selected.label()),
                    )
                    .child(
                        div()
                            .text_color(rgb(theme::MUTED))
                            .child("No sample data for this category yet."),
                    )
                    .child(
                        Button::new("back-to-overview")
                            .accessibility_label("Back to Overview")
                            .px(px(14.))
                            .py(px(8.))
                            .rounded_full()
                            .bg(theme::tint(theme::BLUE, 0.12))
                            .text_color(rgb(theme::BLUE))
                            .focus_visible(|style| style.border_2().border_color(rgb(theme::BLUE)))
                            .on_click(
                                cx.listener(|view, _, _, cx| view.select(Section::Overview, cx)),
                            )
                            .child("Back to Overview"),
                    ),
            );
        }
        let summaries: Vec<_> = self
            .data
            .breakdowns
            .iter()
            .filter(|breakdown| overview || breakdown.section == self.selected)
            .collect();
        if !summaries.is_empty() {
            content = content.child(
                div()
                    .grid()
                    .grid_cols(if width >= px(1200.) { 3 } else { 1 })
                    .gap(px(theme::GAP))
                    .children(summaries.into_iter().map(components::breakdown_card)),
            );
        }
        content.child(
            div()
                .text_size(px(11.))
                .text_color(rgb(theme::MUTED))
                .child("Sample data · Live monitoring is not connected"),
        )
    }
}

impl Render for Dashboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(theme::BACKGROUND))
            .font_family(".SystemUIFont")
            .text_size(px(14.))
            .text_color(rgb(theme::INK))
            .child(self.navigation(cx))
            .child(self.content(window.viewport_size().width, cx))
    }
}
