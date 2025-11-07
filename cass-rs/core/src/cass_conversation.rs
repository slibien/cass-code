use crate::codex::Cass;
use crate::error::Result as CassResult;
use crate::protocol::Event;
use crate::protocol::Op;
use crate::protocol::Submission;
use std::path::PathBuf;

pub struct CassConversation {
    codex: Cass,
    rollout_path: PathBuf,
}

/// Conduit for the bidirectional stream of messages that compose a conversation
/// in Cass.
impl CassConversation {
    pub(crate) fn new(codex: Cass, rollout_path: PathBuf) -> Self {
        Self {
            codex,
            rollout_path,
        }
    }

    pub async fn submit(&self, op: Op) -> CassResult<String> {
        self.cass.submit(op).await
    }

    /// Use sparingly: this is intended to be removed soon.
    pub async fn submit_with_id(&self, sub: Submission) -> CassResult<()> {
        self.cass.submit_with_id(sub).await
    }

    pub async fn next_event(&self) -> CassResult<Event> {
        self.cass.next_event().await
    }

    pub fn rollout_path(&self) -> PathBuf {
        self.rollout_path.clone()
    }
}
