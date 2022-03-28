//! Execute a command on multiple hosts.
//!
//! Usage:
//!
//! ```shell
//! $ tricorder -i inventory do -- echo "run on all hosts"
//! $ tricorder -i inventory -H foo do -- echo "run only on host 'foo'"
//! $ tricorder -i inventory -t myapp do -- echo "run only on hosts tagged with 'myapp'"
//! ```
//!
//! The result is a JSON document with the following structure:
//!
//! ```json
//! [
//!   {
//!     "host": "example-0",
//!     "success": true,
//!     "info": {
//!       "exit_code": 0,
//!       "output": "..."
//!     }
//!   },
//!   {
//!     "host": "example-1",
//!     "success": false,
//!     "error": "..."
//!   }
//! ]
//! ```
//!
//! Commands can be templated with data from the host as defined in the
//! inventory:
//!
//! ```shell
//! $ tricorder -i inventory do -- echo "{host.id} says {host.vars.msg}"
//! ```

use crate::{Result, Host};

use clap::ArgMatches;
use serde_json::{json, Value};
use tinytemplate::{TinyTemplate, format_unescaped};

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
  let cmd_tmpl = get_command(matches.values_of("cmd"));

  let results: Vec<Value> = hosts
    .iter()
    .map(|host| {
      let cmd = render_command(host, cmd_tmpl.clone())?;
      Ok((host, cmd))
    })
    .collect::<Result<Vec<(&Host, String)>>>()?
    .into_iter()
    .map(|(host, cmd)| {
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

fn render_command(host: &Host, cmd_tmpl: String) -> Result<String> {
  let mut tt = TinyTemplate::new();
  tt.set_default_formatter(&format_unescaped);
  tt.add_template("cmd", cmd_tmpl.as_str())?;

  let ctx = json!({ "host": host });
  let cmd = tt.render("cmd", &ctx)?;
  Ok(cmd)
}
