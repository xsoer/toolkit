mod common;
mod kit;
mod utils;
mod views;

use iced::{window, Application, Settings};

fn main() -> iced::Result {
    let icon = window::Icon::from_rgba(vec![100, 200, 20, 1], 1, 1).unwrap();
    let setting = Settings {
        window: window::Settings {
            size: (800, 600),
            resizable: true,
            icon: Some(icon),
            ..window::Settings::default()
        },
        flags: (),
        default_font: Some(include_bytes!("static/kaishu.ttf")), //设置中文的.ttf字体可以支持中文
        antialiasing: true,
        default_text_size: 20u16,
        exit_on_close_request: true,
    };
    kit::ToolKit::run(setting)
}
