use iced::{Align, Element, Length, Scrollable, scrollable};

use crate::message::{Message, MessageMessage};

#[derive(Debug)]
pub enum ChatView {
    // Loading,
    Loaded(ChatViewState),
}

#[derive(Debug)]
pub struct ChatViewState {
    messages: Vec<Message>,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum ChatViewMessage {
    HandleMessageMessage(MessageMessage),
}

impl ChatView {
    pub fn new(messages: Vec<Message>) -> Self {
        ChatView::Loaded(
            ChatViewState {
                messages,
                scroll: scrollable::State::new(),
            }
        )
    }

    pub fn update(&mut self, message: ChatViewMessage) {
        todo!()
    }

    pub fn view(&mut self) -> Element<ChatViewMessage> {
        match self {
            ChatView::Loaded(state) => {
                let message_list: Scrollable<_> = state.messages.iter_mut().fold(
                    Scrollable::new(&mut state.scroll)
                        .spacing(40)
                        .align_items(Align::Center)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    |scrollable, message| {
                        scrollable.push(message
                            .view()
                            .map(ChatViewMessage::HandleMessageMessage))
                    }
                )
                .padding(20);

                message_list.into()

                // Container::new(&message_list)
                //     .width(Length::Fill)
                //     .height(Length::Fill)
                //     .into()
            }
        }
    }
}
