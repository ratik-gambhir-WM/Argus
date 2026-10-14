# Token Guard

A desktop app built with Rust, GPUI, and GPUI Kit. The main view is a system overview based on the supplied dashboard reference, with category navigation, metric cards, activity bars, and memory/power breakdowns.

## Run

Install the [GPUI Kit platform prerequisites](https://gpui-kit.com/docs/installation/), then run:

```sh
cargo run
```

The dashboard currently displays **sample data**, not live machine or account readings. Category buttons and card headings open the corresponding view; categories without fixtures show an empty state. The layout adapts to the window width, and the content and navigation scroll when needed.

## Dashboard structure

- `src/main.rs`: application startup and native window configuration.
- `src/dashboard/mod.rs`: navigation state and responsive page composition.
- `src/dashboard/model.rs`: typed metric and breakdown data.
- `src/dashboard/sample.rs`: reference fixtures, replaceable by a future telemetry source.
- `src/dashboard/components.rs`: shared metric cards, summaries, and icon treatments.
- `src/dashboard/charts.rs`: activity bars and native donut drawing.
- `src/dashboard/theme.rs`: shared colors, spacing, and surface tokens.

No additional dependencies are required. Run `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` to check changes.
