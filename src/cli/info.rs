use crate::{Result, Host};

use clap::ArgMatches;
use serde_json::json;

pub fn run(hosts: Vec<Host>, _matches: &ArgMatches) -> Result<()> {
  let res = json!({"hosts": hosts});
  println!("{}", res);

  Ok(())
}
