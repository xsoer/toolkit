use iced::{button, Button, Column, Element, Length, Text};

use crate::common::Message;
pub struct Controls {
    pub uuid_btn: button::State,
    pub pwd_state: button::State,
    pub time_state: button::State,
    pub encoder_state: button::State,
    pub base_state: button::State,
    pub regex_state: button::State,
    pub qrcode_state: button::State,
    pub color_state: button::State,
    pub crontab_state: button::State,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            uuid_btn: button::State::new(),
            pwd_state: button::State::new(),
            time_state: button::State::new(),
            encoder_state: button::State::new(),
            base_state: button::State::new(),
            regex_state: button::State::new(),
            qrcode_state: button::State::new(),
            color_state: button::State::new(),
            crontab_state: button::State::new(),
        }
    }
}

impl Controls {
    pub fn view(&mut self) -> Element<Message> {
        let menu_btn = |state, text: &str, msg: Message| {
            Button::new(state, Text::new(text))
                .on_press(msg)
                .width(Length::Units(100))
        };
        let btns = Column::new()
            .spacing(10)
            .push(menu_btn(
                &mut self.uuid_btn,
                "UUID",
                Message::MenuUuidPressed,
            ))
            .push(menu_btn(
                &mut self.pwd_state,
                "密码",
                Message::MenuPwdPressed,
            ))
            .push(menu_btn(
                &mut self.time_state,
                "时间戳",
                Message::MenuTimePressed,
            ))
            .push(menu_btn(
                &mut self.encoder_state,
                "编解码",
                Message::MenuEncoderPressed,
            ))
            .push(menu_btn(
                &mut self.base_state,
                "进制转化",
                Message::MenuBasePressed,
            ))
            .push(menu_btn(
                &mut self.regex_state,
                "正则表达式",
                Message::MenuRegexPressed,
            ))
            .push(menu_btn(
                &mut self.qrcode_state,
                "二维码",
                Message::MenuQrcodePressed,
            ))
            .push(menu_btn(
                &mut self.color_state,
                "拾色器",
                Message::MenuColorPressed,
            ))
            .push(menu_btn(
                &mut self.crontab_state,
                "定时任务",
                Message::MenuCrontabPressed,
            ));
        btns.into()
    }
}
