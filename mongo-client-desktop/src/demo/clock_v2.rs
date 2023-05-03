use iced::{executor, Renderer};
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{Application, Color, Command, Element, Length, Point, Rectangle, Settings, Subscription, Theme, Vector};

pub fn clock() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    now: time::OffsetDateTime,
    clock: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Clock {
                now: time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                clock: Default::default(),
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Michael - Clock")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                    self.clock.clear();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Message::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
            )
        })
    }
}

impl<Message> canvas::Program<Message> for Clock {
    type State = ();

    fn draw(&self, _state: &Self::State, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            // 填充时钟背景色
            let background_area = Path::circle(center, radius);
            // 蓝色
            frame.fill(&background_area, Color::from_rgb8(0x12, 0x93, 0xD8));
            // 黑色
            // frame.fill(&background_area, Color::from_rgb8(0x0, 0x0, 0x0));
            // 绿色
            // frame.fill(&background_area, Color::from_rgb8(0, 255, 0));

            // 时针
            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            // 分针
            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;
            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(Color::WHITE),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(Color::WHITE),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            // 平移定位
            frame.translate(Vector::new(center.x, center.y));

            // 时针
            frame.with_save(|frame| {
                // 计算旋转角度
                frame.rotate(hand_rotation(self.now.hour(), 12));
                // 画线
                frame.stroke(&short_hand, wide_stroke());
            });

            // 分针
            frame.with_save(|frame| {
                // 计算旋转角度
                frame.rotate(hand_rotation(self.now.minute(), 60));
                // 画线
                frame.stroke(&long_hand, wide_stroke());
            });

            // 秒针
            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.second(), 60));
                frame.stroke(&long_hand, thin_stroke());
            })
        });
        vec![clock]
    }
}

// 计算旋转角度
fn hand_rotation(n: u8, total: u8) -> f32 {
    let turns = n as f32 / total as f32;

    2.0 * std::f32::consts::PI * turns
}