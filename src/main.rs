use iced::{executor, Application, Command, Element, Settings,};

mod ui;
use ui::{message, chat_view};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    chat_view: chat_view::ChatView
}

#[derive(Debug)]
enum HelloMessage {
    Hello(chat_view::ChatViewMessage)
}

impl Application for Hello {
    type Executor = executor::Default;
    type Message = HelloMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello { 
            chat_view: chat_view::ChatView::new(vec![
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading,
                message::Message::Loading])
            },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.chat_view
            .view()
            .map(HelloMessage::Hello)
    }
}
