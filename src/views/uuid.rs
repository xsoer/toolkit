use iced::{
    button, text_input, Align, Button, Checkbox, Column, Element, Length, Row, Text, TextInput,
};

use crate::common::Message;

#[derive(Debug, Clone)]
pub struct UuidView {
    pub uuid_state: button::State,
    pub uuid_value: String,
    pub count_value: u8,
    pub count_state: text_input::State,
    pub check_upper: bool,
    pub check_split: bool,
    pub check_brace: bool,
}

impl Default for UuidView {
    fn default() -> Self {
        Self {
            uuid_state: button::State::new(),
            uuid_value: String::new(),
            count_value: 1,
            count_state: text_input::State::new(),
            check_upper: false,
            check_split: false,
            check_brace: false,
        }
    }
}

impl UuidView {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .padding(20)
            .push(
                Row::new()
                    .spacing(10)
                    .push(Text::new("数量[1,100]:"))
                    .push(
                        TextInput::new(
                            &mut self.count_state,
                            "",
                            &self.count_value.to_string(),
                            Message::UuidCountInputChanged,
                        )
                        .width(Length::Units(30)),
                    )
                    .push(
                        Button::new(&mut self.uuid_state, Text::new("生成"))
                            .on_press(Message::BtnUuidPressed),
                    ),
            )
            .push(
                Row::new()
                    .spacing(20)
                    .push(Checkbox::new(
                        self.check_upper,
                        "大写",
                        Message::UuidCheckUpperToggled,
                    ))
                    .push(Checkbox::new(
                        self.check_split,
                        "*-*-*",
                        Message::UuidCheckSplitToggled,
                    ))
                    .push(Checkbox::new(
                        self.check_brace,
                        "{***}",
                        Message::UuidCheckBraceToggled,
                    )),
            )
            .push(Text::new(self.uuid_value.as_str()))
            .into()
    }
}
