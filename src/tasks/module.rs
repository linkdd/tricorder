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
    data_path: Option<String>, // todo! implement with &str and livetime
    module_path: String,
    module_name: String,
}

impl Task {
    /// Create a new `module` task

    pub fn new(data_path: Option<String>, module_path: String) -> Self {
        let module_name = module_path.split("/").last().unwrap().to_owned(); // todo better error handling
        Self { data_path, module_path, module_name }
    }
}

impl GenericTask<String> for Task {
    fn prepare(&self, host: Host) -> Result<String> {

        if let Some(datapath) = self.data_path.clone() {

        }
        // todo! merge data from data-file and host-vars
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
        channel.exec(&format!("~/.local/tricorder/modules/{}",self.module_name))?; 

        channel.write_all(data.as_bytes())?;
        channel.send_eof()?;
        Ok(channel)
    }

    fn upload_module(&self, host: &Host, home_path: &str) -> Result<()>{

        let sess = host.get_session()?;

        // create folder
        let mut channel = sess.channel_session()?;
        channel.exec("mkdir -p ~/.local/tricorder/modules")?;

        let mut module_binary_file = File::open(format!("{}", self.module_path))?;


        let mut module_binary: Vec<u8> = vec![];
        module_binary_file.read_to_end(&mut module_binary)?;
        // !todo(fix): create dir if not exists
        let mut remote_file = sess.scp_send(
            Path::new(&format!("{}/.local/tricorder/modules/{}", home_path, self.module_name)), 
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
    }
}