//! Command Line Interface to the `tricorder::tasks::info` task
//!
//! Example:
//!
//! ```shell
//! $ tricorder -i inventory info
//! ```

use crate::prelude::*;
use crate::tasks::info;

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
    let parallel = matches.is_present("parallel");

    let task = info::Task::new();
    let res = hosts.run_task(&task, parallel)?;
    println!("{}", res);

    Ok(())
}
