use gpui::*;
use gpui_telegram_desktop::views::AppShell;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.open_window(
            WindowOptions {
                focus: true,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| AppShell::new(cx)),
        )
        .expect("Failed to open main window");
    });
}
