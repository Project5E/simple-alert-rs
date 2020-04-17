use std::sync::Arc;

pub mod server;
mod client;

#[derive(Debug, Clone)]
pub(crate) struct Webhook {
    pub(crate) wx: Arc<String>
}