//! Command Line Interface to the `tricorder::tasks::info` task
//!
//! Example:
//!
//! ```shell
//! $ tricorder -i inventory info
//! ```

use crate::core::{Result, Host};
use crate::tasks::{TaskRunner, info};

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, _matches: &ArgMatches) -> Result<()> {
  let task = info::Task::new();
  let res = hosts.run_task_seq(&task)?;
  println!("{}", res);

  Ok(())
}
