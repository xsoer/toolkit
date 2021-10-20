use iced::{
    executor, scrollable, Application, Command, Container, Element, Length, Row, Scrollable,
};

use crate::common::Message;
use crate::views::{control::Controls, pwd::PwdView, time::TimeView, uuid::UuidView};

pub struct ToolKit {
    controls: Controls,
    active: Active,
    uuid_view: UuidView,
    pwd_view: PwdView,
    time_view: TimeView,
    ctr_scroll: scrollable::State,
    view_scroll: scrollable::State,
}

enum Active {
    Uuid,
    Pwd,
    Time,
}

impl Default for Active {
    fn default() -> Self {
        Active::Uuid
    }
}

impl Application for ToolKit {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                controls: Controls::default(),
                uuid_view: UuidView::default(),
                pwd_view: PwdView::default(),
                time_view: TimeView::default(),
                active: Active::default(),
                ctr_scroll: scrollable::State::new(),
                view_scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "工具箱".into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::MenuUuidPressed => self.active = Active::Uuid,
            Message::MenuPwdPressed => self.active = Active::Pwd,
            Message::MenuTimePressed => self.active = Active::Time,
            Message::UuidBtnPressed => self.uuid_view.gen_uuid(),
            Message::PwdBtnPressed => self.pwd_view.gen_pwd(),
            Message::TimeBtnPressed => self.time_view.time_value = chrono::Local::now().to_string(),
            Message::UuidCountInputChanged(v) => {
                self.uuid_view.count_value = v.parse::<u8>().unwrap_or(1)
            }
            Message::UuidCheckUpperToggled(v) => self.uuid_view.check_upper = v,
            Message::UuidCheckSplitToggled(v) => self.uuid_view.check_split = v,
            Message::UuidCheckBraceToggled(v) => self.uuid_view.check_brace = v,
            Message::PwdCheckUpperToggled(v) => self.pwd_view.upper_literal = v,
            Message::PwdCheckLowerToggled(v) => self.pwd_view.lower_literal = v,
            Message::PwdCheckDigitToggled(v) => self.pwd_view.digit = v,
            Message::PwdCheckSpecialToggled(v) => self.pwd_view.special = v,
            Message::PwdSliderChanged(v) => self.pwd_view.len_value = v,
            Message::PwdLenInputChanged(v) => {
                self.pwd_view.len_value = v.parse::<u8>().unwrap_or(1)
            }
            Message::PwdNumInputChanged(v) => {
                self.pwd_view.num_value = v.parse::<u8>().unwrap_or(1)
            }
            Message::TimeTimeStampChanged(v) => {
                self.time_view.timestamp_changed(v);
            }
            Message::TimeDateTimeChanged(v) => {
                self.time_view.datetime_changed(v);
            }
            _ => {
                println!("unhandled message")
            }
        }
        Command::none()
    }
    // 如何封装一个tabpanel组件
    fn view(&mut self) -> Element<'_, Self::Message> {
        let ctr_scrol = Scrollable::new(&mut self.ctr_scroll)
            .width(Length::Units(150))
            .height(Length::Units(1000))
            .push(self.controls.view());

        let v = match self.active {
            Active::Uuid => self.uuid_view.view(),
            Active::Time => self.time_view.view(),
            Active::Pwd => self.pwd_view.view(),
        };
        let view_scrol = Scrollable::new(&mut self.view_scroll)
            .width(Length::Fill)
            .push(v);

        let col = Row::new().spacing(20).push(ctr_scrol).push(view_scrol);

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            .center_y()
            .padding(10)
            .into()
    }
}

// mod style {
//     use iced::{button, Background, Color, Vector};

//     pub enum Button {
//         Filter { selected: bool },
//         Icon,
//         Destructive,
//     }

//     impl button::StyleSheet for Button {
//         fn active(&self) -> button::Style {
//             match self {
//                 Button::Filter { selected } => {
//                     if *selected {
//                         button::Style {
//                             background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
//                             border_radius: 10.0,
//                             text_color: Color::WHITE,
//                             ..button::Style::default()
//                         }
//                     } else {
//                         button::Style::default()
//                     }
//                 }
//                 Button::Icon => button::Style {
//                     text_color: Color::from_rgb(0.5, 0.5, 0.5),
//                     ..button::Style::default()
//                 },
//                 Button::Destructive => button::Style {
//                     background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
//                     border_radius: 5.0,
//                     text_color: Color::WHITE,
//                     shadow_offset: Vector::new(1.0, 1.0),
//                     ..button::Style::default()
//                 },
//             }
//         }

//         fn hovered(&self) -> button::Style {
//             let active = self.active();

//             button::Style {
//                 text_color: match self {
//                     Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
//                     Button::Filter { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
//                     _ => active.text_color,
//                 },
//                 shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
//                 ..active
//             }
//         }
//     }
// }
