use iced::{
    button, text_input, Align, Button, Column, Element, Length, Radio, Row, Text, TextInput,
};

use crate::common::{EnDeCodeType, Message};

pub struct EnDeCodeView {
    pub endecode_radio: Option<EnDeCodeType>,
    pub encode_state: text_input::State,
    pub encode_value: String,
    pub encode_btn: button::State,
    pub decode_state: text_input::State,
    pub decode_value: String,
    pub decode_btn: button::State,
}

impl Default for EnDeCodeView {
    fn default() -> Self {
        Self {
            endecode_radio: Some(EnDeCodeType::Unicode),
            encode_state: text_input::State::new(),
            encode_value: String::new(),
            encode_btn: button::State::new(),
            decode_state: text_input::State::new(),
            decode_value: String::new(),
            decode_btn: button::State::new(),
        }
    }
}

impl EnDeCodeView {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20)
            .push(EnDeCodeType::all().iter().cloned().fold(
                Row::new().padding(5).spacing(20),
                |c, t| {
                    c.push(Radio::new(
                        t,
                        t,
                        self.endecode_radio,
                        Message::EnDeCodeRadioSelected,
                    ))
                },
            ))
            .push(
                Row::new()
                    .padding(10)
                    .spacing(20)
                    .push(
                        TextInput::new(
                            &mut self.encode_state,
                            "",
                            &self.encode_value,
                            Message::EnDeCodeEncodeChanged,
                        )
                        .size(30),
                    )
                    .push(
                        Column::new()
                            .padding(10)
                            .push(
                                Button::new(&mut self.encode_btn, Text::new("encode->"))
                                    .on_press(Message::EnDeCodeBtnEncodeToggled).padding(10),
                            )
                            .push(
                                Button::new(&mut self.decode_btn, Text::new("<-decode"))
                                    .on_press(Message::EnDeCodeBtnDecodeToggled).padding(10),
                            ),
                    )
                    .push(
                        TextInput::new(
                            &mut self.decode_state,
                            "",
                            &self.decode_value,
                            Message::EnDeCodeDecodeChanged,
                        )
                        .size(30),
                    ),
            )
            .into()
    }
}
