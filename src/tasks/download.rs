//! Download a file from a remote host
//!
//! Example usage:
//!
//! ```no_run
//! use tricorder::prelude::*;
//! use tricorder::tasks::download;
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
//! let task = download::Task::new(
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

use crate::prelude::*;

use serde_json::json;

use std::{env, fs, io, path::Path};

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
        Self {
            remote_path,
            local_path,
        }
    }
}

impl GenericTask<String> for Task {
    fn prepare(&self, host: Host) -> Result<String> {
        let local_path = Path::new(&self.local_path);

        if local_path.is_absolute() {
            return Err(Box::new(Error::IsAbsolute(
                "Local path should be a relative path, not absolute".to_string(),
            )));
        }

        let cwd = env::current_dir()?;
        let fullpath = cwd.join(host.id.to_string()).join(local_path);
        let fulldir = fullpath.parent().unwrap();
        fs::create_dir_all(fulldir)?;

        Ok(String::from(fullpath.to_string_lossy()))
    }

    fn apply(&self, host: Host, local_path: String) -> TaskResult {
        let sess = host.get_session()?;
        let sftp = sess.sftp()?;

        let mut remote_file = sftp.open(Path::new(&self.remote_path))?;
        let mut local_file = fs::File::create(&local_path)?;
        let size = io::copy(&mut remote_file, &mut local_file)?;

        Ok(json!({
          "file_path": local_path,
          "file_size": size,
        }))
    }
}
