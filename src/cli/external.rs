use crate::Result;

use clap::ArgMatches;
use std::process::{Command, exit};


pub fn run(
  command: &str,
  inventory_arg: Option<&str>,
  host_id_arg: Option<&str>,
  host_tags_arg: Option<&str>,
  matches: &ArgMatches,
) -> Result<()> {
  let bin = format!("tricorder-{}", command);
  let args = matches
    .values_of_os("")
    .unwrap_or_default()
    .collect::<Vec<_>>();

  let status = Command::new(bin)
    .args(args)
    .env("TRICORDER_INVENTORY", inventory_arg.unwrap_or(""))
    .env("TRICORDER_HOST_ID", host_id_arg.unwrap_or(""))
    .env("TRICORDER_HOST_TAGS", host_tags_arg.unwrap_or(""))
    .status()?;

  match status.code() {
    Some(code) => {
      exit(code);
    },
    None => {
      eprintln!("Subcommand was terminated by a signal.");
      exit(127);
    }
  }
}
