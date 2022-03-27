use crate::{Result, Host};

use clap::ArgMatches;
use serde_json::{json, Value};

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
  let cmd = get_command(matches.values_of("cmd"));

  let results: Vec<Value> = hosts
    .iter()
    .map(|host| {
      eprintln!("Executing command on {}...", host.id);
      host.exec(&cmd)
        .map_or_else(
          |err| {
            json!({
              "host": host.id,
              "success": false,
              "error": format!("{}", err),
            })
          },
          |(exit_code, output)| {
            json!({
              "host": host.id,
              "success": true,
              "info": {
                "exit_code": exit_code,
                "output": output,
              },
            })
          }
        )
    })
    .collect();

  let out = json!(results);
  println!("{}", out);

  Ok(())
}

fn get_command(arg: Option<clap::Values<'_>>) -> String {
  arg
    .map(|vals| vals.collect::<Vec<_>>())
    .map(|argv| shell_words::join(argv))
    .unwrap()
}
