//! Execute a command on a remote host
//!
//! Example usage:
//!
//! ```rust
//! use tricorder::core::{Inventory, Host};
//! use tricorder::tasks::{TaskRunner, exec};
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new("localhost", "localhost:22")
//!       .set_user("root")
//!       .add_tag("local")
//!       .set_var("msg", json!("hello"))
//!   );
//!
//! let task = exec::Task::new("echo \"{host.id} says {host.vars.msg}\"");
//! let result = inventory.hosts.run_task_seq(&task).unwrap();
//! ```
//!
//! The result is a JSON document with the following structure:
//!
//! ```json
//! [
//!   {
//!     "host": "example-0",
//!     "success": true,
//!     "info": {
//!       "exit_code": 0,
//!       "output": "..."
//!     }
//!   },
//!   {
//!     "host": "example-1",
//!     "success": false,
//!     "error": "..."
//!   }
//! ]
//! ```

use crate::core::{Result, Host};
use super::{Task as TaskTrait, TaskResult};

use tinytemplate::{TinyTemplate, format_unescaped};
use serde_json::json;

use std::io::prelude::*;

/// Describe an `exec` task
pub struct Task {
  /// Command template to execute on the remote host.
  ///
  /// Example: `"echo \"{host.id} says {host.vars.msg}\""`
  command_template: String,
}

impl Task {
  /// Create a new `exec` task
  pub fn new(command_template: String) -> Self {
    Self { command_template }
  }
}

impl TaskTrait<String> for Task {
  fn prepare(&self, host: Host) -> Result<String> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("cmd", self.command_template.as_str())?;

    let ctx = json!({ "host": host });
    let cmd = tt.render("cmd", &ctx)?;
    Ok(cmd)
  }

  fn apply(&self, host: Host, command: String) -> TaskResult {
    let sess = host.get_session()?;
    let mut channel = sess.channel_session()?;
    channel.exec(&command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;

    let exit_code = channel.exit_status()?;

    Ok(json!({
      "exit_code": exit_code,
      "output": output,
    }))
  }
}
