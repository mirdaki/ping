use chrono::{DateTime, Utc};
use iced::{Align, Container, Element, HorizontalAlignment, Length, Row, Text};

#[derive(Debug)]
pub enum Message {
    Loading,
    Visible(MessageState),
    // Hover(MessageState),
    // Edit(MessageState),
}

#[derive(Debug)]
pub struct MessageState {
    text: String,
    sent_time: DateTime<Utc>,
    user_name: String,
}

#[derive(Debug, Clone)]
pub enum MessageMessage {
    // StartEdit,
// SaveEdit,
// CancleEdit,
// React,
// Reply,
// Delete,
}

impl Message {
    pub fn new(/*text: String, sent_time: DateTime<Utc>, user_name: String*/) -> Self {
        // Message {
        //     MessageState: {
        //         text,
        //         sent_time,
        //         user_name
        //     },
        // }
        Message::Loading
    }

    pub fn update(&mut self, message: MessageMessage) {
        todo!()
    }

    pub fn view(&mut self) -> Element<MessageMessage> {
        match self {
            Message::Loading => {
                let message_body: Element<_> = Container::new(
                    Text::new("Loading")
                        .width(Length::Fill)
                        .size(25)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .into();

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(message_body)
                    .into()
            }
            Message::Visible(state) => {
                let message_body: Element<_> = Container::new(
                    Text::new(&state.text)
                        .width(Length::Fill)
                        .size(25)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .into();

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(message_body)
                    .into()
            }
        }
    }
}
