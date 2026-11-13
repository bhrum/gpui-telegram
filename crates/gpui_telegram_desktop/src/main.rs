use gpui::*;
use gpui_telegram_client::{ClientConfig, TelegramClient};
use gpui_telegram_desktop::views::AppShell;
use std::sync::Arc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V" || a == "-v") {
        println!("GPUI Telegram Desktop v{}", env!("CARGO_PKG_VERSION"));
        println!("gpui_telegram_desktop {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("GPUI Telegram Desktop Client");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!();
        println!("USAGE:");
        println!("    gpui_telegram_desktop [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -s, --server-url <URL>    Set the remote Telegram server URL");
        println!("    -h, --help                Print help information");
        println!("    -V, --version             Print version information");
        std::process::exit(0);
    }

    let config = ClientConfig::resolve();
    println!(
        "[GPUI Telegram] Resolved Server URL: {} (Source: {:?})",
        config.server_url, config.config_source
    );

    let client = Arc::new(TelegramClient::new(config));
    let _tokio_guard = client.runtime().enter();

    Application::new().run(move |cx: &mut App| {
        cx.activate(true);
        let bounds = Bounds::centered(None, size(px(1280.0), px(820.0)), cx);
        let app_client = client.clone();

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
                cx.new(|cx| AppShell::new_with_client(app_client, cx))
            },
        )
        .expect("Failed to open main window");
        cx.activate(true);
    });
}
