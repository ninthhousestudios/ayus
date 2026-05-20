pub mod theme;

use iced::widget::{column, container, text};
use iced::{Center, Element, Fill, Size, Task, Theme};

const NOTO_DEVA: &[u8] = include_bytes!("../../fonts/NotoSansDevanagari-Regular.ttf");
const NOTO_SANS: &[u8] = include_bytes!("../../fonts/NotoSans-Regular.ttf");

struct State {
    status: String,
}

#[derive(Debug, Clone)]
enum Message {}

fn boot() -> (State, Task<Message>) {
    let state = State {
        status: "Āyus — Ayurveda Knowledge Explorer".to_string(),
    };
    (state, Task::none())
}

fn update(_state: &mut State, _message: Message) -> Task<Message> {
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let content = column![
        text(&state.status)
            .size(24)
            .font(theme::latin())
            .color(theme::TEXT_COLOR),
        text("Ready")
            .size(14)
            .font(theme::latin())
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(12)
    .align_x(Center);

    container(content)
        .center(Fill)
        .into()
}

fn app_theme(_state: &State) -> Theme {
    theme::ayus_theme()
}

pub fn run() -> iced::Result {
    iced::application(boot, update, view)
        .title("Āyus")
        .theme(app_theme)
        .font(NOTO_DEVA)
        .font(NOTO_SANS)
        .default_font(theme::latin())
        .window_size(Size::new(1100.0, 800.0))
        .centered()
        .antialiasing(true)
        .run()
}
