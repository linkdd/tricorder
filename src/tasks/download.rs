//! Download a file from a remote host
//!
//! Example usage:
//!
//! ```rust
//! use tricorder::core::{Inventory, Host};
//! use tricorder::tasks::{TaskRunner, download};
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
//! let task = download::Task::new_template(
//!   "/path/to/remote/file.ext".to_string(),
//!   "file.ext".to_string(),
//! );
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
//!       "file_path": "<pwd>/example-0/file.ext",
//!       "file_size": 12345
//!     }
//!   },
//!   {
//!     "host": "example-1",
//!     "success": false,
//!     "error": "..."
//!   }
//! ]
//! ```

use crate::core::{Result, Error, Host};
use super::{Task as TaskTrait, TaskResult};

use serde_json::json;

use std::{
  io,
  path::Path,
  env,
  fs,
};

/// Describe a `download` task
pub struct Task {
  /// Path to file on remote
  remote_path: String,
  /// Relative path on local machine to download the file to.
  ///
  /// The full path will be `{pwd}/{host.id}/{local_path}`.
  local_path: String,
}

impl Task {
  pub fn new(remote_path: String, local_path: String) -> Self {
    Self { remote_path, local_path }
  }
}

impl TaskTrait<String> for Task {
  fn prepare(&self, host: Host) -> Result<String> {
    let local_path = Path::new(&self.local_path);

    if local_path.is_absolute() {
      return Err(Box::new(Error::IsAbsolute(
        "Local path should be a relative path, not absolute".to_string()
      )))
    }

    let cwd = env::current_dir()?;
    let fullpath = cwd.join(host.id).join(local_path);
    let fulldir = fullpath.parent().unwrap();
    fs::create_dir_all(fulldir)?;

    Ok(String::from(fullpath.to_string_lossy()))
  }

  fn apply(&self, host: Host, local_path: String) -> TaskResult {
    let sess = host.get_session()?;
    let (mut channel, _) = sess.scp_recv(Path::new(&self.remote_path))?;

    let mut remote_file_stream = channel.stream(1);
    let mut local_file = fs::File::create(&local_path)?;
    let size = io::copy(&mut remote_file_stream, &mut local_file)?;

    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;

    Ok(json!({
      "file_path": local_path,
      "file_size": size,
    }))
  }
}
