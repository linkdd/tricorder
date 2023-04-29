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

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

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
    fn prepare(&self, _host: Host) -> Result<String> {
        // merge the default data with data in host var
        // todo!

        // upload module to host
        // todo! this should not rely on rsync

        Ok("testing".to_owned())
    }

    fn apply(&self, host: Host, _data: String) -> TaskResult {


        let sess = host.get_session()?;

        upload_module(&self, &host)?;


        let mut channel = sess.channel_session()?;

        channel.exec("ls")?; // !todo: execute binary with data

        let mut stdout = String::new();
        channel.read_to_string(&mut stdout)?;
        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr)?;

        channel.wait_close()?;
        println!("closed channel");

        let exit_code = channel.exit_status()?;

        Ok(json!({
          "exit_code": exit_code,
          "stdout": stdout,
          "stderr": stderr,
        }))
    }
}

fn upload_module(task: &Task, host: &Host) -> Result<()>{

    let sess = host.get_session()?;

    let mut module_binary_file = File::open(format!("{}{}", task.data_dir, task.module_name))?;

    let mut module_binary: Vec<u8> = vec![];
    module_binary_file.read_to_end(&mut module_binary)?;

    let mut remote_file = sess.scp_send(
        Path::new("/home/spiegie/tricoder/mod"), 
        0o760, 
        module_binary.len() as u64, 
        None)?;

    println!("{}",module_binary.len());
    
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