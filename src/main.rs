use iced::{executor, Application, Command, Element, Settings,};

mod ui;
use ui::message;

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    messages: message::Message
}

#[derive(Debug)]
enum HelloMessage {
    Hello(message::MessageMessage)
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = HelloMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello { messages: message::Message::new()} , Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.messages = message::Message::Loading;
        self.messages
            .view()
            .map(HelloMessage::Hello)
    }
}
