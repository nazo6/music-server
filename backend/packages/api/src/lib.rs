use axum::Router;
use server_background::{BackgroundCommand, BackgroundEvent};
use tokio::sync::mpsc;

mod auth;
mod auth_extractor;
mod graphql;
mod setup;

type BgReceiver = async_channel::Receiver<BackgroundEvent>;
type BgSender = mpsc::Sender<BackgroundCommand>;

#[derive(Clone, Debug)]
pub struct AppState {
    background_event_receiver: BgReceiver,
    background_command_sender: BgSender,
}

impl AppState {
    pub fn new(
        background_event_receiver: async_channel::Receiver<BackgroundEvent>,
        background_command_sender: mpsc::Sender<BackgroundCommand>,
    ) -> Self {
        AppState {
            background_event_receiver,
            background_command_sender,
        }
    }
}

pub fn init<S>(bg_event_receiver: BgReceiver, bg_event_sender: BgSender) -> Router<S> {
    let app_state = AppState::new(bg_event_receiver, bg_event_sender);
    let gql = Router::new().nest("/graphql", graphql::init());

    let rest = Router::new()
        .nest("/auth", auth::init())
        .nest("/setup", setup::init());

    Router::new().merge(gql).merge(rest).with_state(app_state)
}
