use iced::{
    button, text_input, Align, Button, Column, Element, Length, Radio, Row, Text, TextInput,
};

use crate::common::{BaseType, Message};

pub struct BaseView {
    pub base_radio: Option<BaseType>,
    pub from_state: text_input::State,
    pub from_value: String,
    pub btn_change: button::State,
    pub to_2: String,
    pub to_8: String,
    pub to_10: String,
    pub to_16: String,
    pub to_26: String,
    pub to_32: String,
    pub to_36: String,
    pub to_52: String,
    pub to_58: String,
    pub to_62: String,
    pub to_64: String,
}

impl Default for BaseView {
    fn default() -> Self {
        Self {
            base_radio: Some(BaseType::Base10),
            from_state: text_input::State::new(),
            from_value: String::new(),
            btn_change: button::State::new(),
            to_2: String::new(),
            to_8: String::new(),
            to_10: String::new(),
            to_16: String::new(),
            to_26: String::new(),
            to_32: String::new(),
            to_36: String::new(),
            to_52: String::new(),
            to_58: String::new(),
            to_62: String::new(),
            to_64: String::new(),
        }
    }
}

impl BaseView {
    pub fn convert(&mut self) {
        if let Some(t) = self.base_radio {
            let v = self.from_value.clone();
            match t {
                BaseType::Base2 => {
                    self.to_2 = v;
                }
                BaseType::Base8 => {
                    self.to_8 = v;
                }
                BaseType::Base10 => {
                    self.to_10 = v;
                }
                BaseType::Base16 => {
                    self.to_16 = v;
                }
                BaseType::Base26 => {
                    self.to_26 = v;
                }
                BaseType::Base32 => {
                    self.to_32 = v;
                }
                BaseType::Base36 => {
                    self.to_36 = v;
                }
                BaseType::Base52 => {
                    self.to_52 = v;
                }
                BaseType::Base58 => {
                    self.to_58 = v;
                }
                BaseType::Base62 => {
                    self.to_62 = v;
                }
                BaseType::Base64 => {
                    self.to_64 = v;
                }
            }
        } else {
            self.to_2 = "hahah".into();
            self.to_8 = "hahah".into();
            self.to_10 = "hahah".into();
            self.to_16 = "hahah".into();
            self.to_26 = "hahah".into();
            self.to_32 = "hahah".into();
            self.to_36 = "hahah".into();
            self.to_52 = "hahah".into();
            self.to_58 = "hahah".into();
            self.to_62 = "hahah".into();
            self.to_64 = "hahah".into();
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let convert = |name, value, desc| {
            Row::new()
                .spacing(20)
                .push(Text::new(name).width(Length::Units(200)))
                .push(Text::new(value).width(Length::Units(200)))
                .push(Text::new(desc).width(Length::Units(300)))
        };
        Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .width(Length::Fill)
            .push(BaseType::all().iter().cloned().fold(
                Row::new().padding(5).spacing(20),
                |c, t| {
                    c.push(Radio::new(
                        t,
                        t,
                        self.base_radio,
                        Message::BaseRadioSelected,
                    ))
                },
            ))
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.from_state,
                            "",
                            &self.from_value,
                            Message::BaseFromValueChanged,
                        )
                        .size(20)
                        .padding(10)
                        .max_width(100),
                    )
                    .push(
                        Button::new(&mut self.btn_change, Text::new("转换"))
                            .on_press(Message::BaseBtnChanged)
                            .width(Length::Units(50)),
                    ),
            )
            .push(convert("2进制", &self.to_2, ""))
            .push(convert("8进制", &self.to_8, ""))
            .push(convert("10进制", &self.to_10, ""))
            .push(convert("16进制", &self.to_16, ""))
            .push(convert("26进制", &self.to_26, "小写字母"))
            .push(convert("32进制", &self.to_32, "不包含 ILOU 字符"))
            .push(convert("36进制", &self.to_36, "数字 + 小写字母"))
            .push(convert("52进制", &self.to_52, "大写字母 + 小写字母"))
            .push(convert("58进制", &self.to_58, "不包含 0OlI 字符"))
            .push(convert("62进制", &self.to_62, "数字 + 小写字母 + 大写字母"))
            .push(convert(
                "64进制",
                &self.to_64,
                "数字 + 小写字母 + 大写字母 + =*",
            ))
            .into()
    }
}
