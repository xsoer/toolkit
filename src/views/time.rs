use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone};
use iced::{button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput};

use crate::common::Message;

#[derive(Debug, Clone)]
pub struct TimeView {
    pub time_state: button::State,
    pub time_value: String,
    pub from_timestamp_state: text_input::State,
    pub from_timestamp_value: i64,
    pub to_datetime_value: String,
    pub from_datetime_state: text_input::State,
    pub from_datetime_value: String,
    pub to_timestamp_value: String,
}

impl Default for TimeView {
    fn default() -> Self {
        let now = chrono::Local::now();
        let timestamp = now.timestamp();
        Self {
            time_state: button::State::new(),
            time_value: String::new(),
            from_timestamp_state: text_input::State::new(),
            from_timestamp_value: timestamp,
            to_datetime_value: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            from_datetime_state: text_input::State::new(),
            from_datetime_value: String::new(),
            to_timestamp_value: String::new(),
        }
    }
}

impl TimeView {
    pub fn timestamp_changed(&mut self, v: String) {
        let t = Local::now().timestamp();
        self.from_timestamp_value = v.parse::<i64>().unwrap_or(t);
        let naive_datetime = NaiveDateTime::from_timestamp(self.from_timestamp_value, 0);
        // 本地时间
        let d = DateTime::<Local>::from_utc(naive_datetime, FixedOffset::east(8 * 3600));
        self.to_datetime_value = d.format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn datetime_changed(&mut self, v: String) {
        let a = match Local.datetime_from_str(&v, "%Y-%m-%d %H:%M:%S") {
            Ok(r) => r.timestamp().to_string(),
            Err(_) => "error".to_owned(),
        };
        self.to_timestamp_value = a;
        self.from_datetime_value = v;
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .spacing(20)
                    .push(
                        Button::new(&mut self.time_state, Text::new("生成时间"))
                            .on_press(Message::TimeBtnPressed),
                    )
                    .push(Text::new(self.time_value.as_str())),
            )
            .push(
                Row::new()
                    .spacing(20)
                    .push(Text::new("时间戳:").width(Length::Units(80)))
                    .push(
                        TextInput::new(
                            &mut self.from_timestamp_state,
                            "",
                            &self.from_timestamp_value.to_string(),
                            Message::TimeTimeStampChanged,
                        )
                        .width(Length::Units(200)),
                    )
                    .push(Text::new("-->"))
                    .push(Text::new(&self.to_datetime_value).width(Length::Units(200))),
            )
            .push(
                Row::new()
                    .spacing(20)
                    .push(Text::new("日期时间:").width(Length::Units(80)))
                    .push(
                        TextInput::new(
                            &mut self.from_datetime_state,
                            "",
                            &self.from_datetime_value,
                            Message::TimeDateTimeChanged,
                        )
                        .width(Length::Units(200)),
                    )
                    .push(Text::new("-->"))
                    .push(Text::new(&self.to_timestamp_value).width(Length::Units(200))),
            )
            .into()
    }
}
