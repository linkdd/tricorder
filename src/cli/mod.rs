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
//! | `-t, --host_tags <STR>` | Boolean tag expression to select the hosts (example: `foo & !(bar | baz)`) |
//!
//! > **NB:**
//! >   - If `-H` is provided, `-t` will be ignored.
//! >   - If `-i` is omitted, we assume an inventory with only `root@localhost:22`
//! >   - The host needs only one tag from the list to match in order to be selected (boolean OR)

pub mod download;
pub mod exec;
pub mod external;
pub mod info;
pub mod module;
pub mod upload;

use crate::prelude::{Host, HostId, Inventory, Result};

use clap::ArgMatches;

pub fn run(matches: ArgMatches) -> Result<()> {
    let inventory_arg = matches.value_of("inventory");
    let host_id_arg = matches.value_of("host_id");
    let host_tags_arg = matches.value_of("host_tags");

    match matches.subcommand() {
        Some(("info", sub_matches)) => {
            let inventory = get_inventory(inventory_arg);
            let hosts = get_host_list(inventory, host_id_arg, host_tags_arg)?;
            info::run(hosts, sub_matches)
        }
        Some(("do", sub_matches)) => {
            let inventory = get_inventory(inventory_arg);
            let hosts = get_host_list(inventory, host_id_arg, host_tags_arg)?;
            exec::run(hosts, sub_matches)
        }
        Some(("upload", sub_matches)) => {
            let inventory = get_inventory(inventory_arg);
            let hosts = get_host_list(inventory, host_id_arg, host_tags_arg)?;
            upload::run(hosts, sub_matches)
        }
        Some(("download", sub_matches)) => {
            let inventory = get_inventory(inventory_arg);
            let hosts = get_host_list(inventory, host_id_arg, host_tags_arg)?;
            download::run(hosts, sub_matches)
        }
        Some(("module", sub_matches)) => {
            let inventory = get_inventory(inventory_arg);
            let hosts = get_host_list(inventory, host_id_arg, host_tags_arg)?;
            module::run(hosts, sub_matches)
        }
        Some((cmd, sub_matches)) => {
            external::run(cmd, inventory_arg, host_id_arg, host_tags_arg, sub_matches)
        }
        _ => {
            unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
        }
    }
}

fn get_inventory(arg: Option<&str>) -> Inventory {
    match arg {
        Some(path) => match Inventory::from_file(path) {
            Ok(inventory) => inventory,
            Err(err) => {
                eprintln!("{}, ignoring...", err);
                Inventory::new()
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
    host_tags_arg: Option<&str>,
) -> Result<Vec<Host>> {
    if let Some(host_id) = host_id_arg {
        let hostlist = match inventory.get_host_by_id(HostId::new(host_id)?) {
            Some(host) => {
                vec![host]
            }
            None => {
                eprintln!("Host '{}' not found in inventory, ignoring...", host_id);
                vec![]
            }
        };
        return Ok(hostlist);
    }

    if let Some(host_tags) = host_tags_arg {
        return Ok(inventory.get_hosts_by_tags(host_tags.to_string())?);
    }

    return Ok(inventory.hosts.clone());
}
