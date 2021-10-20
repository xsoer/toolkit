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
    pub fn gen_uuid(&mut self) {
        let mut res = vec![];
        for _ in 0..self.count_value {
            let mut v = if self.check_split {
                uuid::Uuid::new_v4().to_string()
            } else {
                uuid::Uuid::new_v4().to_simple().to_string()
            };

            if self.check_upper {
                v = v.to_uppercase();
            }
            if self.check_brace {
                v = format!("{{{}}}", v);
            }
            res.push(v);
        }
        self.uuid_value = res.join("\n");
    }

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
                            .on_press(Message::UuidBtnPressed),
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
