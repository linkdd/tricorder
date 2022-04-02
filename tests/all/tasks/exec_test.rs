use tricorder::prelude::*;
use tricorder::tasks::exec;
use serde_json::json;

use super::common::setup_context;


#[test]
fn it_should_return_output() {
  let inventory = setup_context().unwrap();

  let echo_task = exec::Task::new("echo '{host.id} says {host.vars.msg}'".to_string());
  let result = inventory.hosts.run_task_seq(&echo_task).unwrap();

  assert_eq!(result, json!([
    {
      "host": "localhost",
      "success": true,
      "info": {
        "exit_code": 0 as i32,
        "output": "localhost says hi\n"
      }
    }
  ]))
}
