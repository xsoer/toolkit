use iced::{qr_code, text_input, Align, Column, Element, Length, QRCode, TextInput};

use crate::common::Message;

pub struct QrCodeView {
    pub code_state: text_input::State,
    pub code_value: String,
    pub qr_code: Option<qr_code::State>,
}

impl Default for QrCodeView {
    fn default() -> QrCodeView {
        QrCodeView {
            qr_code: qr_code::State::new("").ok(),
            code_state: text_input::State::new(),
            code_value: String::new(),
        }
    }
}

impl QrCodeView {
    pub fn gen_code(&mut self, mut v: String) {
        v.truncate(100);
        self.qr_code = qr_code::State::new(&v).ok();
        self.code_value = v;
    }

    pub fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.code_state,
            "请输入需要编码的内容",
            &self.code_value,
            Message::QrCodeDataChanged,
        )
        .size(30)
        .padding(15);

        let mut content = Column::new()
            .width(Length::Fill)
            .spacing(20)
            .align_items(Align::Center)
            .push(input);
        if let Some(qr_code) = self.qr_code.as_mut() {
            content = content.push(QRCode::new(qr_code).cell_size(10));
        }
        content.into()
    }
}
