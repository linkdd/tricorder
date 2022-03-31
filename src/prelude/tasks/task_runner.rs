use crate::prelude::{Result, Host};
use super::task::{GenericTask, TaskResult};

use serde_json::{json, Value};
use rayon::prelude::*;

/// TaskRunner trait to extend the `Vec<Host>` type.
pub trait TaskRunner {
  /// Helper function to run a task on multiple hosts either sequentially or
  /// concurrently.
  fn run_task<Data: Send>(&self, task: &dyn GenericTask<Data>, parallel: bool) -> TaskResult;

  /// Run a task sequentially on multiple hosts.
  ///
  /// This function first calls the `prepare()` method for all hosts. All should
  /// succeed, or else the error is returned.
  ///
  /// Once the task is prepared for all hosts, this function calls the `apply()`
  /// method with the contextual data produce at the previous step.
  fn run_task_seq<Data: Send>(&self, task: &dyn GenericTask<Data>) -> TaskResult;

  /// Run a task concurrently on multiple hosts.
  ///
  /// This function first calls the `prepare()` method for all hosts. All should
  /// succeed, or else the error is returned.
  ///
  /// Once the task is prepared for all hosts, this function calls the `apply()`
  /// method with the contextual data produce at the previous step.
  fn run_task_parallel<Data: Send>(&self, task: &dyn GenericTask<Data>) -> TaskResult;
}

impl TaskRunner for Vec<Host> {
  fn run_task<Data: Send>(&self, task: &dyn GenericTask<Data>, parallel: bool) -> TaskResult {
    if parallel {
      self.run_task_parallel(task)
    }
    else {
      self.run_task_seq(task)
    }
  }

  fn run_task_seq<Data: Send>(&self, task: &dyn GenericTask<Data>) -> TaskResult {
    let results: Vec<Value> = self
      .into_iter()
      .map(|host| prepare_host(task, host))
      .collect::<Result<Vec<(&Host, Data)>>>()?
      .into_iter()
      .map(|(host, data)| apply_to_host(task, host, data))
      .collect();

    Ok(json!(results))
  }

  fn run_task_parallel<Data: Send>(&self, task: &dyn GenericTask<Data>) -> TaskResult {
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
  task: &dyn GenericTask<Data>,
  host: &'host Host,
) -> Result<(&'host Host, Data)> {
  let data = task.prepare(host.clone())?;
  Ok((host, data))
}

fn apply_to_host<'host, Data: Send>(
  task: &dyn GenericTask<Data>,
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
