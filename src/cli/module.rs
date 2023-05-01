//! Upload a Modoule to remote host and call it with data.
//!
//! A module is an executable, that accepts structured
//! data as Input (json) via stdin and performs
//! actions based on the provided data.
//!
//! The Module optionally takes default data from <DATA_FILE_PATH>
//! these get overwritten with custom values in
//! host var host.vars.module_<module_name>
//! <DATA_FILE_PATH> should contain all information of the
//! expected data structure
//!
//! Usage:
//!
//! ```shell
//! $ tricorder -i inventory module --data <DATA_FILE_PATH> --module <MODULE_PATH>
//! $ tricorder -i inventory module --module <MODULE_PATH>
//! ```
//!

use crate::prelude::*;
use crate::tasks::module;

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_path(matches.value_of("data_file_path"));
    let module_path = get_path(matches.value_of("module"))?;
    let parallel = matches.is_present("parallel");

    let task = module::Task::new(data_path, module_path);

    let res = hosts.run_task(&task, parallel)?;
    println!("{}", res);

    Ok(())
}

fn get_path(arg: Option<&str>) -> Result<String> {
    if let Some(path) = arg {
        Ok(String::from(path))
    } else {
        Err(Box::new(Error::MissingInput(
            "No input file provided".to_string(),
        )))
    }
}

fn get_data_path(arg: Option<&str>) -> Option<String> {
    arg.map(String::from)
}
