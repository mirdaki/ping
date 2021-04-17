use iced::{executor, Application, Clipboard, Column, Command, Element, Length, Settings, Text};

use matrix_sdk::{
    self, async_trait,
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        SyncMessageEvent,
    },
    reqwest::Url,
    Client, ClientConfig, EventEmitter, SyncRoom, SyncSettings,
};

mod ui;
use ui::{chat_view, message};

#[tokio::main]
async fn main() -> iced::Result {
    Ping::run(Settings::default())
}

enum Ping {
    Loading,
    Loaded {
        matrix_client: Client,
        chat_view: chat_view::ChatView,
    },
    Errored {
        error: matrix_sdk::Error,
    }
}

#[derive(Debug)]
enum PingMessage {
    MatrixLogin(Result<Client, matrix_sdk::Error>),
    LoadChatView(chat_view::ChatViewMessage),
}

impl Application for Ping {
    type Executor = executor::Default;
    type Message = PingMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Ping, Command<Self::Message>) {
        let username = "";
        let password = "";
        let homeserver_url = "https://matrix-client.matrix.org";
        (
            Ping::Loading,
            Command::perform(
                login(&homeserver_url, &username, &password),
                PingMessage::MatrixLogin,
            ),
        )
    }

    fn title(&self) -> String {
        match self {
            Ping::Loading => String::from("Loading..."),
            Ping::Loaded {
                matrix_client: _,
                chat_view: _,
            } => String::from("Ping"),
            Ping::Errored {
                error
            } => error.to_string(),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            PingMessage::MatrixLogin(Ok(matrix_client)) => {
                *self = Ping::Loaded {
                    matrix_client,
                    chat_view: chat_view::ChatView::new(vec![]),
                };
                Command::none()
            },
            PingMessage::MatrixLogin(Err(error)) => {
                *self = Ping::Errored {
                    error,
                };
                Command::none()
            },
            PingMessage::LoadChatView(_) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // let rt = tokio::runtime::Runtime::new().unwrap();
        // rt.spawn(matrix());
        match self {
            Ping::Loading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Loading...").size(40))
                .into(),
            Ping::Loaded {
                matrix_client: _,
                chat_view,
            } => chat_view.view().map(PingMessage::LoadChatView),
            Ping::Errored {
                error
            } => Column::new()
            .width(Length::Shrink)
            .push(Text::new(error.to_string()).size(40))
            .into(),
        }
    }
}

struct EventCallback;

#[async_trait]
impl EventEmitter for EventCallback {
    async fn on_room_message(&self, room: SyncRoom, event: &SyncMessageEvent<MessageEventContent>) {
        if let SyncRoom::Joined(room) = room {
            if let SyncMessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent { body: msg_body, .. }),
                sender,
                ..
            } = event
            {
                let name = {
                    // any reads should be held for the shortest time possible to
                    // avoid dead locks
                    let room = room.read().await;
                    let member = room.joined_members.get(&sender).unwrap();
                    member.name()
                };
                println!("{}: {}", name, msg_body);
            }
        }
    }
}

async fn login(
    homeserver_url: &str,
    username: &str,
    password: &str,
) -> Result<Client, matrix_sdk::Error> {
    let client_config = ClientConfig::new();
    // .proxy("http://localhost:8080")?
    // .disable_ssl_verification();
    let homeserver_url = Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");
    let client =
        Client::new_with_config(homeserver_url, client_config).expect("Could not load config");

    //client.add_event_emitter(Box::new(EventCallback)).await;

    client
        .login(username, password, None, Some("ping"))
        .await
        .expect("Could not login");
    //client.sync(SyncSettings::new()).await;

    Ok(client)
}
