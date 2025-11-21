use serde::{Deserialize, Serialize};

/// Execution mode for Lit Actions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Invocation {
    /// Synchronous execution - waits for result (default).
    #[default]
    Sync,
    /// Asynchronous execution - returns immediately.
    Async,
}
