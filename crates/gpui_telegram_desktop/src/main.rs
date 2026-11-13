use gpui::*;
use gpui_telegram_desktop::views::AppShell;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        let bounds = Bounds::centered(None, size(px(1280.0), px(820.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                focus: true,
                show: true,
                window_min_size: Some(size(px(800.0), px(560.0))),
                ..Default::default()
            },
            |window, cx| {
                window.activate_window();
                cx.new(|cx| AppShell::new(cx))
            },
        )
        .expect("Failed to open main window");
        cx.activate(true);
    });
}
