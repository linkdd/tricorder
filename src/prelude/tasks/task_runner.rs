use super::task::{GenericTask, TaskResult};
use crate::prelude::{Host, Result};

use rayon::prelude::*;
use serde_json::{json, Value};

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
        } else {
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
    task.apply(host.clone(), data).map_or_else(
        |err| {
            json!({
              "host": host.id,
              "success": false,
              "error": format!("{}", err),
            })
        },
        |info| {
            json!({
              "host": host.id,
              "success": true,
              "info": info,
            })
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Error;

    pub struct DummyTask;

    impl DummyTask {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl GenericTask<i32> for DummyTask {
        fn prepare(&self, host: Host) -> Result<i32> {
            if host.id.to_string() == String::from("success") {
                Ok(1)
            } else {
                Err(Box::new(Error::Other(String::from("failure"))))
            }
        }

        fn apply(&self, host: Host, data: i32) -> TaskResult {
            if host.id.to_string() == String::from("success") {
                Ok(json!(data + 1))
            } else {
                Err(Box::new(Error::Other(String::from("failure"))))
            }
        }
    }

    fn setup_success_host() -> Host {
        Host::new(Host::id("success").unwrap(), "success:22".to_string())
    }

    fn setup_failure_host() -> Host {
        Host::new(Host::id("failure").unwrap(), "failure:22".to_string())
    }

    #[test]
    fn prepare_host_should_work() {
        let success_host = setup_success_host();
        let failure_host = setup_failure_host();
        let task = DummyTask::new();

        match prepare_host(&task, &success_host) {
            Ok((host, 1)) => {
                assert_eq!(host, &success_host);
                assert!(true);
            }
            Err(_) => {
                assert!(false, "Unexpected error for successful host");
            }
            _ => {
                assert!(false, "Unexpected result for successful host");
            }
        }

        match prepare_host(&task, &failure_host) {
            Err(_) => {
                assert!(true);
            }
            _ => {
                assert!(false, "Unexpected result for failed host");
            }
        }
    }

    #[test]
    fn apply_to_host_should_work() {
        let success_host = setup_success_host();
        let failure_host = setup_failure_host();
        let task = DummyTask::new();

        assert_eq!(
            apply_to_host(&task, &success_host, 1),
            json!({
              "host": "success",
              "success": true,
              "info": 2 as i32
            })
        );

        assert_eq!(
            apply_to_host(&task, &failure_host, 1),
            json!({
              "host": "failure",
              "success": false,
              "error": "Other(\"failure\")"
            })
        );
    }
}
