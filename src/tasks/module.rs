//! Upload a Module to the remote host and call it with data
//!
//! Example usage:
//!
//! an example for a Moule could be a simple shell-script
//! which reads data from stdin and just echos it
//! the shell script is named /tmp/mod.sh in this example
//! ```shell
//! #!/bin/bash
//! read -r data
//! echo "$data"
//! ```
//!
//! ```no_run 
//! use serde_json::json;
//! use tricorder::prelude::*;
//! use tricorder::tasks::module;
//!
//! const MODULE_FILE: &str = r#"
//! #!/bin/bash
//! read -r data
//! echo echo output: "$data"
//! "#;
//! 
//! const DATA_FILE: &str = r#"
//! {
//!     "data": "data_from_file",
//!     "overwrittendata":"data from file shoul be overwritten by var modue_mod.sh"
//! }
//! "#;
//! 
//! const BINARY_PATH: &str = "/tmp/mod.sh";
//! const DATA_PATH: &str = "/tmp/data_file.json";
//! 
//! fn main() {
//!     //write the module file
//! 
//!     std::fs::write(BINARY_PATH, MODULE_FILE).unwrap();
//!     std::fs::write(DATA_PATH, DATA_FILE).unwrap();
//! 
//!     let inventory = Inventory::new()
//!         .add_host(
//!             Host::new(Host::id("localhost").unwrap(), "localhost:22".to_string())
//!                 .set_user("root".to_string())
//!                 .add_tag(Host::tag("local").unwrap())
//!                 .set_var("msg".to_string(), json!("hello"))
//!                 .set_var(
//!                     // you can define host variables overwriteing the values of the data file 
//!                     // these variables sould be named "module_<name of the binary>" in This case mod.sh (see BINARY_PATH)
//!                     "module_mod.sh".to_string(),
//!                     json!({"overwrittendata":"data_from_var1", "vardata":"data_from_var2"}),
//!                 )
//!                 .to_owned(),
//!         )
//!         .to_owned();
//! 
//!     let task = module::Task::new(Some(DATA_PATH.to_string()), BINARY_PATH.to_string());
//! 
//!     let result = inventory.hosts.run_task_seq(&task).unwrap();
//! 
//!     println!("{:#?}", result);
//! }
//! ```
//!
//! The result looks like this:
//! ```json
//! Array [
//!     Object {
//!         "host": String("localhost"),
//!         "info": Object {
//!             "exit_code": Number(0),
//!             "stderr": String(""),
//!             "stdout": String("echo output: {\"data\":\"data_from_file\",\"overwrittendata\":\"data_from_var1\",\"vardata\":\"data_from_var2\"}\n"),
//!         },
//!         "success": Bool(true),
//!     },
//! ]
//! ```
//!
//! you can see, that the variable "overwrittendata" gets
//! overwritten by the host-variable module_mod.sh

use crate::prelude::*;

use serde_json::{json, Value};
use ssh2::Channel;

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

/// Describe an `module` task
pub struct Task {
    data_path: Option<String>,
    module_path: String,
    module_name: String,
}

impl Task {
    /// Create a new `module` task

    pub fn new(data_path: Option<String>, module_path: String) -> Self {
        let module_name = module_path.split("/").last().unwrap().to_owned();
        Self {
            data_path,
            module_path,
            module_name,
        }
    }
}

impl GenericTask<Value> for Task {
    fn prepare(&self, host: Host) -> Result<Value> {
        let hostvars = host.vars.clone();

        let default = json!({});

        let host_var_data: &Value = hostvars
            .get(&format!("module_{}", self.module_name))
            .unwrap_or(&default);

        if let Some(datapath) = self.data_path.clone() {
            let mut data: Value = fs::read_to_string(datapath)?.parse()?;

            merge(&mut data, host_var_data);
            Ok(data)
        } else {
            Ok(host_var_data.clone())
        }
    }

    fn apply(&self, host: Host, data: Value) -> TaskResult {
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
    fn execute_module(&self, host: &Host, data: Value) -> Result<Channel> {
        let sess = host.get_session()?;
        let mut channel = sess.channel_session()?;
        channel.exec(&format!("~/.local/tricorder/modules/{}", self.module_name))?;

        channel.write_all(serde_json::to_string(&data)?.as_bytes())?;
        channel.send_eof()?;
        Ok(channel)
    }

    fn upload_module(&self, host: &Host, home_path: &str) -> Result<()> {
        let sess = host.get_session()?;

        // create folder
        let mut channel = sess.channel_session()?;
        channel.exec("mkdir -p ~/.local/tricorder/modules")?;

        let mut module_binary_file = File::open(format!("{}", self.module_path))?;

        let mut module_binary: Vec<u8> = vec![];
        module_binary_file.read_to_end(&mut module_binary)?;

        let mut remote_file = sess.scp_send(
            Path::new(&format!(
                "{}/.local/tricorder/modules/{}",
                home_path, self.module_name
            )),
            0o700,
            module_binary.len() as u64,
            None,
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

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}
