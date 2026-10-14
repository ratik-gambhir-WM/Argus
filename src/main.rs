mod dashboard;

use dashboard::Dashboard;
use gpui_kit::*;

fn main() {
    application().with_assets(assets::AllAssets).run(|cx| {
        init(cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1420.), px(840.)), cx)),
            window_min_size: Some(size(px(640.), px(480.))),
            titlebar: Some(TitlebarOptions {
                title: Some("Token Guard".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(20.), px(24.))),
            }),
            ..Default::default()
        };
        open_window(options, cx, |_, cx| cx.new(|_| Dashboard::new()))
            .expect("failed to open Token Guard window");
        cx.activate(true);
    });
}
