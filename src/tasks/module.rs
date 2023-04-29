//! A module is an executable, that accepts structured
//! data as Input (yaml, toml or json) and performs
//! actions based on the provided Data
//!
//! Task takes default data from <datadir>/default.toml
//! default-values get overwritten with custom values in
//! host var host.module_<module_name>
//! default.toml should contain all information of the
//! expected data structure

use crate::prelude::*;

use tinytemplate::{TinyTemplate, format_unescaped};
use serde_json::json;

use std::io::prelude::*;
use std::process::Command;
use std::process::Output;

/// Describe an `module` task
pub struct Task {
    /// Command template to execute on the remote host.
    ///
    /// Example: `"echo \"{host.id} says {host.vars.msg}\""`
    data_dir: String,
    module_name: String,
}

impl Task {
    /// Create a new `exec` task
    pub fn new(data_dir: String, module_name: String) -> Self {
        Self { data_dir, module_name }
    }
}

impl GenericTask<String> for Task {
    fn prepare(&self, host: Host) -> Result<String> {
        // merge the default data with data in host var
        // todo!

        // upload module to host
        // todo! this should not rely on rsync

        Ok("testing".to_owned())
    }

    fn apply(&self, host: Host, command: String) -> TaskResult {

        let ip = host.vars.get("ip").unwrap().to_string().trim_matches('"').to_string();

        println!("{}", ip);

        let output = Command::new("rsync")
            .arg("-avzq")
            .arg( format!("{}{}", &self.data_dir,&self.module_name) )
            .arg( format!("{}@{}:~/tricoder/", &host.user, &ip) )
            .output()
            .expect("fileupload failed");

        println!("status: {} {} {}", output.status, String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());

        let sess = host.get_session()?;
        let mut channel = sess.channel_session()?;
        channel.exec(&format!("chmod u+x ~/tricoder/{}", &self.module_name))?;

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
