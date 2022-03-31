//! Upload a file on a remote host
//!
//! Example usage:
//!
//! ```no_run
//! use tricorder::core::{Inventory, Host, HostId, HostTag};
//! use tricorder::tasks::{TaskRunner, upload};
//! use serde_json::json;
//!
//! let inventory = Inventory::new()
//!   .add_host(
//!     Host::new(HostId::new("localhost").unwrap(), "localhost:22".to_string())
//!       .set_user("root".to_string())
//!       .add_tag(HostTag::new("local").unwrap())
//!       .set_var("msg".to_string(), json!("hello"))
//!       .to_owned()
//!   )
//!   .to_owned();
//!
//! let task = upload::Task::new_template(
//!   "/path/to/local/file.ext".to_string(),
//!   "/path/to/remote/file.ext".to_string(),
//!   0o644
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

use tinytemplate::{TinyTemplate, format_unescaped};
use serde_json::json;
use std::{
  io::{
    prelude::*,
    BufRead,
    BufReader,
  },
  fs,
  path::Path,
};

/// Describe an `upload` task
pub struct Task {
  /// If true, `local_path` is treated as a template
  is_template: bool,
  /// Path to local file to upload
  local_path: String,
  /// Path to target file on remote host
  remote_path: String,
  /// UNIX file mode to set on the uploaded file
  file_mode: i32,
}

impl Task {
  /// Create a new `upload` task where `local_path` is a template
  pub fn new_template(local_path: String, remote_path: String, file_mode: i32) -> Self {
    Self {
      is_template: true,
      local_path,
      remote_path,
      file_mode,
    }
  }

  /// Create a new `upload` task where `local_path` is a static file
  pub fn new_file(local_path: String, remote_path: String, file_mode: i32) -> Self {
    Self {
      is_template: false,
      local_path,
      remote_path,
      file_mode,
    }
  }
}

pub enum TaskContext {
  Template { content: String, file_size: u64 },
  File { file_size: u64 },
}

impl TaskTrait<TaskContext> for Task {
  fn prepare(&self, host: Host) -> Result<TaskContext> {
    let local_path = Path::new(self.local_path.as_str());

    if !local_path.exists() {
      return Err(Box::new(Error::FileNotFound(
        format!("No such file: {}", self.local_path)
      )))
    }
    else if local_path.is_dir() {
      return Err(Box::new(Error::IsADirectory(
        format!("Path is a directory, not a file: {}", self.local_path)
      )))
    }

    if self.is_template {
      let template = fs::read_to_string(self.local_path.clone())?;

      let mut tt = TinyTemplate::new();
      tt.set_default_formatter(&format_unescaped);
      tt.add_template("file", template.as_str())?;

      let ctx = json!({"host": host});
      let content = tt.render("file", &ctx)?;
      let file_size = u64::try_from(content.len())?;

      Ok(TaskContext::Template { content, file_size })
    }
    else {
      let file_size = local_path.metadata()?.len();

      Ok(TaskContext::File { file_size })
    }
  }

  fn apply(&self, host: Host, context: TaskContext) -> TaskResult {
    let file_size = match context {
      TaskContext::Template { file_size: size, .. } => size,
      TaskContext::File { file_size: size } => size,
    };

    let sess = host.get_session()?;
    let mut channel = sess.scp_send(
      Path::new(&self.remote_path),
      self.file_mode,
      file_size,
      None
    )?;

    match context {
      TaskContext::Template { content, .. } => {
        channel.write(content.as_bytes())?;
      },
      TaskContext::File { .. } => {
        let file = fs::File::open(&self.local_path)?;
        let block_size = 4 * 1024 * 1024; // 4 megabytes
        let mut reader = BufReader::with_capacity(block_size, file);

        loop {
          let buffer = reader.fill_buf()?;
          let length = buffer.len();

          if length > 0 {
            channel.write(buffer)?;
          }
          else {
            break;
          }

          reader.consume(length);
        }
      }
    }

    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;

    Ok(json!({"file_size": file_size}))
  }
}
