//! Run an external command found in `$PATH`.
//!
//! External subcommands are called with the following environment variables:
//!
//! | Variable | Description |
//! | --- | --- |
//! | `TRICORDER_INVENTORY` | Value of the `-i, --inventory` flag |
//! | `TRICORDER_HOST_ID` | Value of the `-H, --host_id` flag |
//! | `TRICORDER_HOST_TAGS` | Value of the `-t, --host_tags` flag |
//!
//! Internally, calling `tricorder [global-options...] SUBCOMMAND [options...]`
//! would be similar to:
//!
//! ```shell
//! $ export TRICORDER_INVENTORY="..."
//! $ export TRICORDER_HOST_ID="..."
//! $ export TRICORDER_HOST_TAGS="..."
//! $ tricorder-SUBCOMMAND [options...]
//! ```

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
