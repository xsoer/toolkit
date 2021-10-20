use iced::{
    button, slider, text_input, Align, Button, Checkbox, Column, Element, Length, Row, Slider,
    Text, TextInput,
};

use crate::common::Message;
use crate::utils::pwd;

#[derive(Debug, Clone)]
pub struct PwdView {
    pub pwd_state: button::State,
    pub pwd_value: Vec<String>,
    pub upper_literal: bool,
    pub lower_literal: bool,
    pub digit: bool,
    pub special: bool,
    pub len_state: slider::State,
    pub len_value: u8,
    pub len_input: text_input::State,
    pub num_state: text_input::State,
    pub num_value: u8,
}

impl Default for PwdView {
    fn default() -> Self {
        Self {
            pwd_state: button::State::new(),
            pwd_value: vec![],
            upper_literal: true,
            lower_literal: true,
            digit: true,
            special: true,
            len_state: slider::State::new(),
            len_value: 5,
            len_input: text_input::State::new(),
            num_state: text_input::State::new(),
            num_value: 1,
        }
    }
}

impl PwdView {
    pub fn gen_pwd(&mut self) {
        let rule = self.get_rule();
        self.pwd_value = pwd::gen_pwd(self.num_value, self.len_value, rule)
    }

    pub fn get_rule(&self) -> usize {
        let mut rule = 0;
        if self.upper_literal {
            rule |= pwd::PwdRule::Upper as usize;
        }
        if self.lower_literal {
            rule |= pwd::PwdRule::Lower as usize;
        }
        if self.digit {
            rule |= pwd::PwdRule::Digit as usize;
        }
        if self.special {
            rule |= pwd::PwdRule::Special as usize;
        }
        rule
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20)
            .push(
                Row::new()
                    // .padding(10)
                    .spacing(20)
                    .push(Text::new("长度"))
                    .push(
                        TextInput::new(
                            &mut self.len_input,
                            "",
                            &self.len_value.to_string(),
                            Message::PwdLenInputChanged,
                        )
                        .width(Length::Units(30)),
                    )
                    .push(Slider::new(
                        &mut self.len_state,
                        5..=128,
                        self.len_value,
                        Message::PwdSliderChanged,
                    )),
            )
            .push(
                Row::new().spacing(5).push(Text::new("数量[1,100]")).push(
                    TextInput::new(
                        &mut self.num_state,
                        "",
                        &self.num_value.to_string(),
                        Message::PwdNumInputChanged,
                    )
                    .width(Length::Units(30)),
                ),
            )
            .push(
                Row::new()
                    .padding(20)
                    .spacing(20)
                    .push(Checkbox::new(
                        self.upper_literal,
                        "大写",
                        Message::PwdCheckUpperToggled,
                    ))
                    .push(Checkbox::new(
                        self.lower_literal,
                        "小写",
                        Message::PwdCheckLowerToggled,
                    ))
                    .push(Checkbox::new(
                        self.digit,
                        "数字",
                        Message::PwdCheckDigitToggled,
                    ))
                    .push(Checkbox::new(
                        self.special,
                        "$#@!^&*/<>",
                        Message::PwdCheckSpecialToggled,
                    )),
            )
            .push(
                Button::new(&mut self.pwd_state, Text::new("生成密码"))
                    .on_press(Message::PwdBtnPressed),
            )
            .push(Text::new(self.pwd_value.join("\n")))
            .into()
    }
}
