use crate::prelude::{Host, Result};
use serde_json::Value;

/// Describe the result of a `Task` execution
pub type TaskResult = Result<Value>;

/// Generic Task trait
pub trait GenericTask<Data: Send>: Send + Sync {
    /// Called to prepare contextual data for the task execution
    fn prepare(&self, host: Host) -> Result<Data>;

    /// Called to execute the task
    fn apply(&self, host: Host, data: Data) -> TaskResult;
}
