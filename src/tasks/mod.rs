//! Available **tricorder** tasks.
//!
//! A `Task` describe an action that can be done on a `Host`.

use crate::core::{Result, Host};
use serde_json::{json, Value};
use rayon::prelude::*;

/// Describe the result of a `Task` execution
pub type TaskResult = Result<Value>;

/// Generic Task trait
pub trait Task<Data: Send> : Send + Sync {
  /// Called to prepare contextual data for the task execution
  fn prepare(&self, host: Host) -> Result<Data>;

  /// Called to execute the task
  fn apply(&self, host: Host, data: Data) -> TaskResult;
}

/// TaskRunner trait to extend the `Vec<Host>` type.
pub trait TaskRunner {
  /// Helper function to run a task on multiple hosts either sequentially or
  /// concurrently.
  fn run_task<Data: Send>(&self, task: &dyn Task<Data>, parallel: bool) -> TaskResult;

  /// Run a task sequentially on multiple hosts.
  ///
  /// This function first calls the `prepare()` method for all hosts. All should
  /// succeed, or else the error is returned.
  ///
  /// Once the task is prepared for all hosts, this function calls the `apply()`
  /// method with the contextual data produce at the previous step.
  fn run_task_seq<Data: Send>(&self, task: &dyn Task<Data>) -> TaskResult;

  /// Run a task concurrently on multiple hosts.
  ///
  /// This function first calls the `prepare()` method for all hosts. All should
  /// succeed, or else the error is returned.
  ///
  /// Once the task is prepared for all hosts, this function calls the `apply()`
  /// method with the contextual data produce at the previous step.
  fn run_task_parallel<Data: Send>(&self, task: &dyn Task<Data>) -> TaskResult;
}

impl TaskRunner for Vec<Host> {
  fn run_task<Data: Send>(&self, task: &dyn Task<Data>, parallel: bool) -> TaskResult {
    if parallel {
      self.run_task_parallel(task)
    }
    else {
      self.run_task_seq(task)
    }
  }

  fn run_task_seq<Data: Send>(&self, task: &dyn Task<Data>) -> TaskResult {
    let results: Vec<Value> = self
      .into_iter()
      .map(|host| prepare_host(task, host))
      .collect::<Result<Vec<(&Host, Data)>>>()?
      .into_iter()
      .map(|(host, data)| apply_to_host(task, host, data))
      .collect();

    Ok(json!(results))
  }

  fn run_task_parallel<Data: Send>(&self, task: &dyn Task<Data>) -> TaskResult {
    let results: Vec<Value> = self
      .into_par_iter()
      .map(|host| prepare_host(task, host))
      .collect::<Result<Vec<(&Host, Data)>>>()?
      .into_par_iter()
      .map(|(host, data)| apply_to_host(task, host, data))
      .collect();

    Ok(json!(results))
  }
}

fn prepare_host<'host, Data: Send>(
  task: &dyn Task<Data>,
  host: &'host Host,
) -> Result<(&'host Host, Data)> {
  let data = task.prepare(host.clone())?;
  Ok((host, data))
}

fn apply_to_host<'host, Data: Send>(
  task: &dyn Task<Data>,
  host: &'host Host,
  data: Data,
) -> Value {
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
}

pub mod exec;
pub mod info;
pub mod upload;
pub mod download;
