use chrono::{DateTime, Utc};
use iced::{Align, Container, Element, HorizontalAlignment, Length, Row, Column, Text, Image};

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

                // TODO: I have the message mostly organized. Images are being weird

                let message_image: Element<_> = Container::new(
                    Text::new("mirdaki2"),
                // Image::new("/home/matthew/Downloads/Mirror-Profile-Square.jpg"),
                    // .width(Length::Fill)
                )
                .width(Length::Units(75))
                .into();

                let message_name: Element<_> = Container::new(
                    Text::new("mirdaki")
                        .width(Length::Fill)
                        .size(15)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )
                .width(Length::Fill)
                .into();

                let message_date_time: Element<_> = Container::new(
                    Text::new("3:22pm")
                        .width(Length::Fill)
                        .size(15)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )
                // .align_x(Align::End)
                .into();

                let message_text: Element<_> = Container::new(
                    Text::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce posuere risus eu lorem placerat, venenatis ornare eros scelerisque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Sed ut vulputate odio.")
                        .width(Length::Fill)
                        .size(20)
                        .horizontal_alignment(HorizontalAlignment::Left),
                )
                .into();

                let message_header: Row<_> = Row::new()
                    .spacing(10)
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(message_name)
                    .push(message_date_time)
                    .into();

                let message_body: Row<_> = Row::new()
                    .spacing(10)
                    .width(Length::Fill)
                    .align_items(Align::Center)
                    .push(message_text)
                    .into();

                let message: Column<_> = Column::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(message_header)
                    .push(message_body)
                    .into();

                Row::new()
                    .spacing(10)
                    .align_items(Align::Start)
                    .push(message_image)
                    .push(message)
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
