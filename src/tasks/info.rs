//! Gather information on hosts
//!
//! Example usage:
//!
//! ```rust
//! use tricorder::core::{Inventory, Host};
//! use tricorder::tasks::{TaskRunner, info};
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new("localhost".to_string(), "localhost:22".to_string())
//!       .set_user("root".to_string())
//!       .add_tag("local".to_string())
//!       .set_var("msg", json!("hello"))
//!   );
//!
//! let task = info::Task::new();
//! let result = inventory.hosts.run_task_seq(&task).unwrap();
//! ```
//!
//! The result is a JSON document with the following structure:
//!
//! ```json
//! [
//!   {
//!     "host": "localhost",
//!     "success": true,
//!     "info": {
//!       "id": "localhost",
//!       "address": "localhost:22",
//!       "user": "root",
//!       "tags": ["local"],
//!       "vars": {"msg": "hello"}
//!     }
//!   }
//! ]
//! ```
//!
//! > **NB:** In the future, will be gathered facts like:
//! >   - network interfaces
//! >   - hostname and FQDN
//! >   - OS / Kernel version
//! >   - partitions / mount points
//! >   - ...

use crate::core::{Result, Host};
use super::{Task as TaskTrait, TaskResult};

use serde_json::json;

/// Describe an `info` task
pub struct Task;

impl Task {
  /// Create a new `info` task
  pub fn new() -> Self {
    Self {}
  }
}

impl TaskTrait<()> for Task {
  fn prepare(&self, _host: Host) -> Result<()> {
    Ok(())
  }

  fn apply(&self, host: Host, _data: ()) -> TaskResult {
    Ok(json!(host))
  }
}
