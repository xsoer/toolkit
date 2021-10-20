use chrono::{DateTime, Local};
use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    Align, Color, Column, Element, Length, Point, Rectangle, Text, Vector,
};

use crate::common::Message;

pub struct ClockView {
    now: chrono::DateTime<chrono::Local>,
    clock: Cache,
}

impl Default for ClockView {
    fn default() -> Self {
        Self {
            now: chrono::Local::now(),
            clock: Default::default(),
        }
    }
}

impl ClockView {
    pub fn update(&mut self, local_time: DateTime<Local>) {
        if local_time != self.now {
            self.now = local_time;
            self.clock.clear();
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20)
            .push(Text::new(self.now.format("%Y-%m-%d %H:%M:%S").to_string()))
            .push(
                Canvas::new(self)
                    .width(Length::Units(400))
                    .height(Length::Units(400)),
            )
            .into()
    }
}

impl canvas::Program<Message> for ClockView {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        use chrono::Timelike;

        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let thin_stroke = Stroke {
                width: radius / 100.0,
                color: Color::WHITE,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let wide_stroke = Stroke {
                width: thin_stroke.width * 3.0,
                ..thin_stroke
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.hour(), 12));
                frame.stroke(&short_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.second(), 60));
                frame.stroke(&long_hand, thin_stroke);
            })
        });

        vec![clock]
    }
}

fn hand_rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    2.0 * std::f32::consts::PI * turns
}
