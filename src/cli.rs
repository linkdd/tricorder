use crate::{Result, Inventory, Host};

use clap::{command, arg};
use is_executable::IsExecutable;
use std::{
  path::Path,
  process::Command,
  fs
};

pub fn parse_args() -> Result<(Vec<Host>, String)> {
  let matches = command!()
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
    .arg(
      arg!(cmd: [COMMAND] "Command to run on each host")
      .last(true)
      .required(true)
    )
    .get_matches();

  let cmd = get_command(matches.values_of("cmd"));
  let inventory = get_inventory(matches.value_of("inventory"))?;
  let hosts = get_host_list(
    inventory,
    matches.value_of("host_id"),
    matches.value_of("host_tags"),
  );

  return Ok((hosts, cmd));
}

fn get_command(arg: Option<clap::Values<'_>>) -> String {
  arg
    .map(|vals| vals.collect::<Vec<_>>())
    .map(|argv| shell_words::join(argv))
    .unwrap()
}

fn get_inventory(arg: Option<&str>) -> Result<Inventory> {
  match arg {
    Some(path) => {
      let inventory_path = Path::new(path);

      if inventory_path.exists() {
        if inventory_path.is_executable() {
          let result = Command::new(path).output()?;

          if !result.status.success() {
            eprintln!("Failed to execute inventory {}: {}", path, result.status);
            eprintln!("Ignoring...");
            Ok(Inventory::new())
          }
          else {
            let content = String::from_utf8(result.stdout)?;
            Ok(Inventory::from_json(&content)?)
          }
        }
        else {
          let content = fs::read_to_string(path)?;
          Ok(Inventory::from_toml(&content)?)
        }
      }
      else {
        eprintln!("Inventory '{}' does not exist, ignoring...", path);
        Ok(Inventory::new())
      }
    },
    None => {
      eprintln!("No inventory provided, using localhost...");
      Ok(Inventory::new())
    }
  }
}

fn get_host_list(
  inventory: Inventory,
  host_id_arg: Option<&str>,
  host_tags_arg: Option<&str>
) -> Vec<Host> {
  if let Some(host_id) = host_id_arg {
    match inventory.get_host_by_id(host_id.to_string()) {
      Some(host) => {
        return vec![host];
      },
      None => {
        eprintln!("Host '{}' not found in inventory, ignoring...", host_id);
        return vec![];
      }
    }
  }

  if let Some(host_tags) = host_tags_arg {
    let tags: Vec<String> = host_tags.split(",")
      .map(|tag| String::from(tag.trim()))
      .collect();

    return inventory.get_hosts_by_tags(tags);
  }

  return inventory.hosts;
}
