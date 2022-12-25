use std::sync::{Arc, Mutex};

use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum BackgroundCommand {
    StartScan {
        responder: oneshot::Sender<StartScanResponse>,
    },
}

#[derive(Debug, Clone)]
pub struct StartScanResponse {
    pub started: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum BackgroundEvent {
    UpdateScan { scanning: bool, count: i32 },
}

#[derive(Debug, Clone)]
struct BackgroundState {
    scanning: bool,
    count: i32,
}

pub struct BackgroundActor {
    receiver: mpsc::Receiver<BackgroundCommand>,
    notifier: async_channel::Sender<BackgroundEvent>,
    state: Arc<Mutex<BackgroundState>>,
}

impl BackgroundActor {
    pub fn new(
        receiver: mpsc::Receiver<BackgroundCommand>,
        notifier: async_channel::Sender<BackgroundEvent>,
    ) -> Self {
        BackgroundActor {
            receiver,
            state: Arc::new(Mutex::new(BackgroundState {
                scanning: false,
                count: 0,
            })),
            notifier,
        }
    }

    async fn handle_message(
        msg: BackgroundCommand,
        state: Arc<Mutex<BackgroundState>>,
        notifier: async_channel::Sender<BackgroundEvent>,
    ) {
        match msg {
            BackgroundCommand::StartScan { responder } => {
                if state.lock().unwrap().scanning {
                    state.lock().unwrap().count += 1;
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

                let count = state.lock().unwrap().count;

                notifier
                    .send(BackgroundEvent::UpdateScan {
                        scanning: true,
                        count,
                    })
                    .await
                    .unwrap();
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
