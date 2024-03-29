//! Execute a command on a remote host
//!
//! Example usage:
//!
//! ```no_run
//! use tricorder::prelude::*;
//! use tricorder::tasks::exec;
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new(Host::id("localhost").unwrap(), "localhost:22".to_string())
//!       .set_user("root".to_string())
//!       .add_tag(Host::tag("local").unwrap())
//!       .set_var("msg".to_string(), json!("hello"))
//!       .to_owned()
//!   )
//!   .to_owned();
//!
//! let task = exec::Task::new("echo \"{host.id} says {host.vars.msg}\"".to_string());
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
//!       "stdout": "...",
//!       "stderr": "..."
//!     }
//!   },
//!   {
//!     "host": "example-1",
//!     "success": false,
//!     "error": "..."
//!   }
//! ]
//! ```

use crate::prelude::*;

use serde_json::json;
use tinytemplate::{format_unescaped, TinyTemplate};

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

impl GenericTask<String> for Task {
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

        let mut stdout = String::new();
        channel.read_to_string(&mut stdout)?;
        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr)?;

        channel.wait_close()?;

        let exit_code = channel.exit_status()?;

        Ok(json!({
          "exit_code": exit_code,
          "stdout": stdout,
          "stderr": stderr,
        }))
    }
}
