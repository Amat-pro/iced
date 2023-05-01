use iced::{Element, Result, Settings, Sandbox, Alignment};
use iced::widget::{button, column, text};

pub fn counter() -> Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Demo-Iced: Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(100),
            button("Decrement").on_press(Message::DecrementPressed),
        ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}