//! Available **tricorder** tasks.
//!
//! A `Task` describe an action that can be done on a `Host`.

use crate::core::{Result, Host};
use serde_json::{json, Value};

/// Describe the result of a `Task` execution
pub type TaskResult = Result<Value>;

/// Generic Task trait
pub trait Task<Data> {
  /// Called to prepare contextual data for the task execution
  fn prepare(&self, host: Host) -> Result<Data>;

  /// Called to execute the task
  fn apply(&self, host: Host, data: Data) -> TaskResult;
}

/// TaskRunner trait to extend the `Vec<Host>` type.
pub trait TaskRunner {
  /// Run a task sequentially on multiple hosts.
  ///
  /// This function first calls the `prepare()` method for all hosts. All should
  /// succeed, or else the error is returned.
  ///
  /// Once the task is prepared for all hosts, this function calls the `apply()`
  /// method with the contextual data produce at the previous step.
  fn run_task_seq<Data>(&self, task: &dyn Task<Data>) -> TaskResult;
}

impl TaskRunner for Vec<Host> {
  fn run_task_seq<Data>(&self, task: &dyn Task<Data>) -> TaskResult {
    let results: Vec<Value> = self
      .into_iter()
      .map(|host| {
        let data = task.prepare(host.clone())?;
        Ok((host, data))
      })
      .collect::<Result<Vec<(&Host, Data)>>>()?
      .into_iter()
      .map(|(host, data)| {
        task.apply(host.clone(), data)
          .map_or_else(
            |err| json!({
              "host": host.id,
              "success": false,
              "error": format!("{}", err),
            }),
            |info| json!({
              "host": host.id,
              "success": true,
              "info": info,
            })
          )
      })
      .collect();

    Ok(json!(results))
  }
}

pub mod exec;
pub mod info;
pub mod upload;
pub mod download;