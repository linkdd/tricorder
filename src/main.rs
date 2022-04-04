use tricorder::{prelude::Result, cli};
use clap::{command, arg, Command};

fn main() -> Result<()> {
  let matches = command!()
    .propagate_version(true)
    .subcommand_required(true)
    .allow_external_subcommands(true)
    .allow_invalid_utf8_for_external_subcommands(true)
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
        .arg(
          arg!(parallel: -p --parallel "If set, the task will be executed concurrently")
        )
    )
    .subcommand(
      Command::new("do")
        .about("Execute a command on multiple hosts")
        .arg(
          arg!(parallel: -p --parallel "If set, the task will be executed concurrently")
        )
        .arg(
          arg!(cmd: [COMMAND] "Command to run on each host")
          .last(true)
          .required(true)
        )
    )
    .subcommand(
      Command::new("upload")
        .about("Upload a file to multiple hosts")
        .arg(
          arg!(parallel: -p --parallel "If set, the task will be executed concurrently")
        )
        .arg(
          arg!(template: -T --template "If set, the file is a template with the current host as context data")
        )
        .arg(
          arg!(local_path: [LOCAL_PATH] "Path on local host to the file to be uploaded")
          .required(true)
        )
        .arg(
          arg!(remote_path: [REMOTE_PATH] "Path on remote host to upload the file")
          .required(true)
        )
        .arg(
          arg!(file_mode: [MODE] "UNIX file mode to set on the uploaded file (default: 0644)")
        )
    )
    .subcommand(
      Command::new("download")
        .about("Download a file from multiple hosts")
        .arg(
          arg!(parallel: -p --parallel "If set, the task will be executed concurrently")
        )
        .arg(
          arg!(remote_path: [REMOTE_PATH] "Path to the file on the remote host")
          .required(true)
        )
        .arg(
          arg!(local_path: [LOCAL_PATH] "Path to the destination on local machine")
          .required(true)
        )
    )
    .get_matches();

  cli::run(matches)
}
