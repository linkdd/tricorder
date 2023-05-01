use serde_json::json;
use tricorder::prelude::*;
use tricorder::tasks::exec;

use super::common::within_context;

#[test]
fn it_should_return_output() {
    within_context(|inventory| {
        let hosts = inventory
            .get_hosts_by_tags("test-success".to_string())
            .unwrap();
        let echo_task = exec::Task::new("echo '{host.id} says {host.vars.msg}'".to_string());
        let result = hosts.run_task_seq(&echo_task).unwrap();

        assert_eq!(
            result,
            json!([
              {
                "host": "localhost",
                "success": true,
                "info": {
                  "exit_code": 0 as i32,
                  "stdout": "localhost says hi\n",
                  "stderr": ""
                }
              }
            ])
        );
    });
}

#[test]
fn it_should_return_exit_code() {
    within_context(|inventory| {
        let hosts = inventory
            .get_hosts_by_tags("test-success".to_string())
            .unwrap();
        let fail_task =
            exec::Task::new("echo '{host.id} says {host.vars.msg}' >&2; exit 42".to_string());
        let result = hosts.run_task_seq(&fail_task).unwrap();

        assert_eq!(
            result,
            json!([
              {
                "host": "localhost",
                "success": true,
                "info": {
                  "exit_code": 42 as i32,
                  "stdout": "",
                  "stderr": "localhost says hi\n"
                }
              }
            ])
        );
    });
}

#[test]
fn it_should_return_an_error() {
    within_context(|inventory| {
        let hosts = inventory
            .get_hosts_by_tags("test-failure".to_string())
            .unwrap();
        let echo_task = exec::Task::new("echo '{host.id}' says {host.vars.msg}".to_string());
        let result = hosts.run_task_seq(&echo_task).unwrap();

        assert_eq!(
            result,
            json!([
              {
                "host": "localhost-fail",
                "success": false,
                "error": "failed to lookup address information: Name or service not known",
              }
            ])
        );
    });
}

#[test]
fn it_should_run_on_all_hosts_sequentially() {
    within_context(|inventory| {
        let echo_task = exec::Task::new("echo '{host.id}' says {host.vars.msg}".to_string());
        let result = inventory.hosts.run_task_seq(&echo_task).unwrap();

        assert_eq!(
            result,
            json!([
              {
                "host": "localhost",
                "success": true,
                "info": {
                  "exit_code": 0 as i32,
                  "stdout": "localhost says hi\n",
                  "stderr": ""
                }
              },
              {
                "host": "localhost-fail",
                "success": false,
                "error": "failed to lookup address information: Name or service not known",
              }
            ])
        );
    });
}

#[test]
fn it_should_run_on_all_hosts_concurrently() {
    within_context(|inventory| {
        let echo_task = exec::Task::new("echo '{host.id}' says {host.vars.msg}".to_string());
        let result = inventory.hosts.run_task_parallel(&echo_task).unwrap();

        assert_eq!(
            result,
            json!([
              {
                "host": "localhost",
                "success": true,
                "info": {
                  "exit_code": 0 as i32,
                  "stdout": "localhost says hi\n",
                  "stderr": ""
                }
              },
              {
                "host": "localhost-fail",
                "success": false,
                "error": "failed to lookup address information: Name or service not known",
              }
            ])
        );
    });
}

#[test]
fn it_should_fail_for_invalid_command_templates() {
    within_context(|inventory| {
        let echo_task = exec::Task::new("echo '{host.id' says {host.vars.msg}".to_string());
        let result = inventory.hosts.run_task_parallel(&echo_task);

        assert!(result.is_err());
    });
}
