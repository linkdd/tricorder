//! Upload a file to multiple remote hosts.
//!
//! Usage:
//!
//! ```shell
//! $ tricorder -i inventory upload LOCAL_PATH REMOTE_PATH [FILE_MODE]
//! $ tricorder -i inventory upload -T LOCAL_PATH REMOTE_PATH [FILE_MODE]
//! ```
//!
//! If not provided, `FILE_MODE` defaults to `0644`.
//!
//! The following options are available:
//!
//! | Flag | Description |
//! | --- | --- |
//! | `-T, --template` | If set, treats `LOCAL_PATH` as a template with the current host as input data. |

use crate::prelude::*;
use crate::tasks::upload;

use clap::ArgMatches;
use file_mode::Mode;
use std::convert::TryFrom;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
    let local_path = get_path(matches.value_of("local_path"))?;
    let remote_path = get_path(matches.value_of("remote_path"))?;
    let file_mode = get_file_mode(matches.value_of("file_mode"))?;
    let parallel = matches.is_present("parallel");

    let task = if matches.is_present("template") {
        upload::Task::new_template(local_path, remote_path, file_mode)
    } else {
        upload::Task::new_file(local_path, remote_path, file_mode)
    };

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

fn get_file_mode(arg: Option<&str>) -> Result<i32> {
    let mut mode = Mode::from(0o644);

    if let Some(mode_str) = arg {
        mode.set_str(mode_str)?;
    }

    let file_mode = i32::try_from(mode.mode())?;
    Ok(file_mode)
}
