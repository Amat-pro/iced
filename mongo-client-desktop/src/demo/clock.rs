use std::fmt::Debug;
use std::time::{SystemTime};
use chrono::{Local};

use iced::{Command};
use iced::widget::{Column, Text, container};
use iced::{Alignment, Application, Element, Settings, Subscription, Theme, Length};
use iced::time::{Duration, Instant};

pub fn clock() -> iced::Result {
    Clock::run(Settings::default())
}

struct Clock {
    time: SystemTime,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

impl Application for Clock {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Clock, Command<Self::Message>) {
        (
            Clock {
                time: SystemTime::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick(_instant) => {
                self.time = SystemTime::now();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let current_time_sec = self.time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
            .to_string();
        let data_time = chrono::DateTime::<Local>::from(self.time);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Text::new(current_time_sec).size(50))
            .push(Text::new(data_time.to_string()).size(50));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_secs(1)).map(Message::Tick)
    }
}