//! Download a file from multiple remote hosts.
//!
//! Usage:
//!
//! ```shell
//! $ tricorder -i inventory download REMOTE_PATH LOCAL_PATH
//! ```
//!
//! The files will be downloaded to: `{pwd}/{host.id}/{local_path}`

use crate::core::{Result, Error, Host};
use crate::tasks::{TaskRunner, download};

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
  let remote_path = get_path(matches.value_of("remote_path"))?;
  let local_path = get_path(matches.value_of("local_path"))?;

  let task = download::Task::new(remote_path, local_path);
  let res = hosts.run_task_seq(&task)?;
  println!("{}", res);

  Ok(())
}

fn get_path(arg: Option<&str>) -> Result<String> {
  if let Some(path) = arg {
    Ok(String::from(path))
  }
  else {
    Err(Box::new(Error::MissingInput(
      "No input file provided".to_string(),
    )))
  }
}
