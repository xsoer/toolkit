use iced::{button, Button, Column, Element, Text};

use crate::common::Message;

#[derive(Debug, Clone)]
pub struct TimeView {
    pub time_state: button::State,
    pub time_value: String,
}

impl Default for TimeView {
    fn default() -> Self {
        Self {
            time_state: button::State::new(),
            time_value: String::new(),
        }
    }
}

impl TimeView {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.time_state, Text::new("生成时间"))
                    .on_press(Message::BtnTimePressed),
            )
            .push(Text::new(self.time_value.as_str()))
            .into()
    }
}
