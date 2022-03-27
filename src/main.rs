use tricorder::{Result, cli};
use clap::{command, arg, Command};

fn main() -> Result<()> {
  let matches = command!()
    .propagate_version(true)
    .subcommand_required(true)
    .arg(
      arg!(inventory: -i --inventory <FILE> "Path to TOML inventory file or program producing JSON inventory")
      .required(false)
    )
    .arg(
      arg!(host_id: -H --host_id <STR> "Identifier of the host to connect to")
      .required(false)
    )
    .arg(
      arg!(host_tags: -t --host_tags <STR> "Comma-separated list of tags identifying the hosts to connect to")
      .required(false)
    )
    .subcommand(
      Command::new("info")
        .about("Gather information about hosts in the inventory")
    )
    .subcommand(
      Command::new("do")
        .about("Execute a command on multiple hosts")
        .arg(
          arg!(cmd: [COMMAND] "Command to run on each host")
          .last(true)
          .required(true)
        )
    )
    .get_matches();

  cli::run(matches)
}
