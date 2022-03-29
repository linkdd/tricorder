//! Command Line Interface to **tricorder** capabilities.
//!
//! **tricorder** is distributed as a static command line tool. It is agent-less
//! and connects to remote hosts via SSH (authentication is done via `ssh-agent`
//! on the local host).
//!
//! It requires an [[Inventory]] and a selection of hosts to perform a task:
//!
//! | Global flag | Description |
//! | --- | --- |
//! | `-i, --inventory <PATH>` | Path to a TOML inventory file or an executable producing a JSON inventory |
//! | `-H, --host_id <STR>` | Specific host on which to perform the task |
//! | `-t, --host_tags <STR>` | Comma-separated list of tags to select the hosts |
//!
//! > **NB:**
//! >   - If `-H` is provided, `-t` will be ignored.
//! >   - If `-i` is omitted, we assume an inventory with only `root@localhost:22`
//! >   - The host needs only one tag from the list to match in order to be selected (boolean OR)

pub mod external;
pub mod info;
pub mod exec;
pub mod upload;

use crate::core::{Result, Inventory, Host};

use clap::ArgMatches;

pub fn run(matches: ArgMatches) -> Result<()> {
  let inventory_arg = matches.value_of("inventory");
  let host_id_arg = matches.value_of("host_id");
  let host_tags_arg = matches.value_of("host_tags");

  match matches.subcommand() {
    Some(("info", sub_matches)) => {
      let inventory = get_inventory(inventory_arg);
      let hosts = get_host_list(inventory, host_id_arg, host_tags_arg);
      info::run(hosts, sub_matches)
    },
    Some(("do", sub_matches)) => {
      let inventory = get_inventory(inventory_arg);
      let hosts = get_host_list(inventory, host_id_arg, host_tags_arg);
      exec::run(hosts, sub_matches)
    },
    Some(("upload", sub_matches)) => {
      let inventory = get_inventory(inventory_arg);
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

fn get_inventory(arg: Option<&str>) -> Inventory {
  match arg {
    Some(path) => {
      match Inventory::from_file(path) {
        Ok(inventory) => {
          inventory
        },
        Err(err) => {
          eprintln!("{}, ignoring...", err);
          Inventory::new()
        }
      }
    },
    None => {
      eprintln!("No inventory provided, using empty inventory...");
      Inventory::new()
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
