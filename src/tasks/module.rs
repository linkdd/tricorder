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

use serde_json::json;
use ssh2::Channel;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

/// Describe an `module` task
pub struct Task {
    data: String,
    executable: String,
}

impl Task {
    /// Create a new `module` task
    pub fn new(data: String, executable: String) -> Self {
        Self { data, executable }
    }
}

impl GenericTask<String> for Task {
    fn prepare(&self, host: Host) -> Result<String> {

        let data = serde_json::to_string(
            host.vars.get("module_mod").unwrap_or(&json!({}))
        )?;
        Ok(data)
    }

    fn apply(&self, host: Host, data: String) -> TaskResult {

        let sess = host.get_session()?;

        let mut channel = sess.channel_session()?;
        channel.exec("echo $HOME")?;

        let mut home_path = String::new();
        channel.read_to_string(&mut home_path)?;
        channel.wait_close()?;

        self.upload_module(&host, home_path.trim())?;

        let mut channel = self.execute_module(&host, data)?;

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
impl Task {
    fn execute_module(&self, host: &Host, data: String) -> Result<Channel> {
        let sess = host.get_session()?;
        let mut channel = sess.channel_session()?;
        channel.exec("~/.local/tricorder/modules/mod")?; // !todo: execute binary with data

        channel.write_all(data.as_bytes())?;
        channel.send_eof()?;
        Ok(channel)
    }

    fn upload_module(&self, host: &Host, home_path: &str) -> Result<()>{

        let sess = host.get_session()?;

        // create folder
        let mut channel = sess.channel_session()?;
        channel.exec("mkdir -p ~/.local/tricorder/modules")?;

        let mut module_binary_file = File::open(format!("{}{}", self.data, self.executable))?;

        let mut module_binary: Vec<u8> = vec![];
        module_binary_file.read_to_end(&mut module_binary)?;
        // !todo(fix): create dir if not exists
        let mut remote_file = sess.scp_send(
            Path::new(&format!("{}/.local/tricorder/modules/mod", home_path)), 
            0o700, 
            module_binary.len() as u64, 
            None
        )?;
        
        remote_file.write(&module_binary)?;
        // Close the channel and wait for the whole content to be transferred
        remote_file.send_eof()?;
        remote_file.wait_eof()?;
        remote_file.close()?;
        remote_file.wait_close()?;

        Ok(())

        /*if rsync_output.status.success() {
            Ok(())
        } else {
            Err(Box::new(Error::UploadFailed(
                "failed to upload module".to_string(),
            )))
        }*/
    }
}