use std::sync::{Arc, Mutex};

use tokio::sync::{mpsc, oneshot};

mod errors;
mod scan;

/// mpmc channel sender to send event to api(graphql subscription).
type Notifier = async_channel::Sender<BackgroundEvent>;
/// mpsc channel receiver to receive command from api.
type Receiver = mpsc::Receiver<BackgroundCommand>;
/// oneshot channel sender to response command.
type Responder<T> = oneshot::Sender<T>;

#[derive(Debug)]
pub enum BackgroundCommand {
    StartScan {
        responder: Responder<StartScanResponse>,
        library_id: i32,
    },
}

#[derive(Debug, Clone)]
pub struct StartScanResponse {
    pub started: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum BackgroundEvent {
    UpdateScan {
        library_id: i32,
        scanning: bool,
        count: i32,
    },
}

#[derive(Debug, Clone)]
struct BackgroundState {
    scanning: bool,
}

pub struct BackgroundActor {
    receiver: Receiver,
    notifier: Notifier,
    state: Arc<Mutex<BackgroundState>>,
}

impl BackgroundActor {
    pub fn new(receiver: Receiver, notifier: Notifier) -> Self {
        BackgroundActor {
            receiver,
            state: Arc::new(Mutex::new(BackgroundState { scanning: false })),
            notifier,
        }
    }

    async fn handle_message(
        msg: BackgroundCommand,
        state: Arc<Mutex<BackgroundState>>,
        notifier: Notifier,
    ) {
        match msg {
            BackgroundCommand::StartScan {
                responder,
                library_id,
            } => {
                if state.lock().unwrap().scanning {
                    tracing::info!("Already scanning");
                    responder
                        .send(StartScanResponse {
                            started: false,
                            message: "Already scanning".to_string(),
                        })
                        .unwrap();
                } else {
                    state.lock().unwrap().scanning = true;
                    responder
                        .send(StartScanResponse {
                            started: true,
                            message: "Started scanning".to_string(),
                        })
                        .unwrap();

                    tracing::info!("Started scanning");
                }
            }
        }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            tracing::info!("Receiver: {:?}", &msg);
            let state = self.state.clone();
            let notifier = self.notifier.clone();
            tokio::spawn(async move {
                Self::handle_message(msg, state, notifier).await;
            });
        }
    }
}
