mod external;
mod info;
mod exec;
mod upload;

use crate::{Result, Inventory, Host};

use clap::ArgMatches;
use is_executable::IsExecutable;
use std::{
  path::Path,
  process::Command,
  fs
};

pub fn run(matches: ArgMatches) -> Result<()> {
  let inventory_arg = matches.value_of("inventory");
  let host_id_arg = matches.value_of("host_id");
  let host_tags_arg = matches.value_of("host_tags");

  match matches.subcommand() {
    Some(("info", sub_matches)) => {
      let inventory = get_inventory(inventory_arg)?;
      let hosts = get_host_list(inventory, host_id_arg, host_tags_arg);
      info::run(hosts, sub_matches)
    },
    Some(("do", sub_matches)) => {
      let inventory = get_inventory(inventory_arg)?;
      let hosts = get_host_list(inventory, host_id_arg, host_tags_arg);
      exec::run(hosts, sub_matches)
    },
    Some(("upload", sub_matches)) => {
      let inventory = get_inventory(inventory_arg)?;
      let hosts = get_host_list(inventory, host_id_arg, host_tags_arg);
      upload::run(hosts, sub_matches)
    },
    Some((cmd, sub_matches)) => {
      external::run(cmd, inventory_arg, host_id_arg, host_tags_arg, sub_matches)
    },
    _ => {
      unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
    }
  }
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

  return inventory.hosts.clone();
}
