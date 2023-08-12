use crate::message::{Message, RaftResponse};

use async_trait::async_trait;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;

#[async_trait]
pub trait Store {
    // async fn apply(&mut self, message: &[u8]) -> Result<Vec<u8>>;
    // async fn snapshot(&self) -> Result<Vec<u8>>;
    // async fn restore(&mut self, snapshot: &[u8]) -> Result<()>;

    async fn apply(&mut self, message: &[u8]) -> Vec<u8>;
    async fn snapshot(&self) -> Vec<u8>;
    async fn restore(&mut self, snapshot: &[u8]) -> ();
}

#[derive(Clone)]
pub struct MailBox(mpsc::Sender<Message>);

impl MailBox {
    pub async fn _send(&self, message: Vec<u8>) -> Result<Vec<u8>, String> {
        let (tx, rx) = oneshot::channel();
        let proposal = Message::Propose {
            proposal: message,
            chan: tx,
        };
        let sender = self.0.clone();
        // TODO make timeout duration a variable
        match sender.send(proposal).await {
            Ok(_) => match timeout(Duration::from_secs(2), rx).await {
                Ok(Ok(RaftResponse::Response { data })) => Ok(data),
                _ => Err("123".to_string()),
            },
            _ => Err("123".to_string()),
        }
    }
}
