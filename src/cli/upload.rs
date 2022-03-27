//! Upload a file to multiple remote hosts.
//!
//! Usage:
//!
//! ```
//! $ tricorder -i inventory upload LOCAL_PATH REMOTE_PATH [FILE_MODE]
//! $ tricorder -i inventory upload -T LOCAL_PATH REMOTE_PATH [FILE_MODE]
//! ```
//!
//! If not provided, `FILE_MODE` defaults to `0644`.
//!
//! The following options are available:
//!
//! | Flag | Description |
//! | --- | --- |
//! | `-T, --template` | If set, treats `LOCAL_PATH` as a template with the current host as input data. |

use crate::{Result, Host};

use clap::ArgMatches;
use file_mode::Mode;
use tinytemplate::{TinyTemplate, format_unescaped};
use serde_json::{json, Value};

use std::{
  convert::TryFrom,
  io::{Error, ErrorKind},
  path::Path,
  fs,
};

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
  let local_path = get_local_path(matches.value_of("local_path"))?;
  let remote_path = get_remote_path(matches.value_of("remote_path"))?;
  let file_mode = get_file_mode(matches.value_of("file_mode"))?;

  if matches.is_present("template") {
    render_template(hosts, local_path, remote_path, file_mode)
  }
  else {
    send_file(hosts, local_path, remote_path, file_mode)
  }
}

fn render_template(
  hosts: Vec<Host>,
  local_path: String,
  remote_path: String,
  file_mode: i32
) -> Result<()> {
  let template = fs::read_to_string(local_path)?;

  let mut tt = TinyTemplate::new();
  tt.set_default_formatter(&format_unescaped);
  tt.add_template("file", template.as_str())?;

  let results: Vec<Value> = hosts
    .iter()
    .map(|host| {
      let ctx = json!({"host": host});
      let content = tt.render("file", &ctx)?;
      let size = u64::try_from(content.len())?;
      Ok((host, content, size))
    })
    .collect::<Result<Vec<(&Host, String, u64)>>>()?
    .into_iter()
    .map(|(host, content, file_size)| {
      host.send_data(remote_path.clone(), file_mode, file_size, content)
        .map_or_else(
          |err| {
            json!({
              "host": host.id,
              "success": false,
              "error":format!("{}", err),
            })
          },
          |()| {
            json!({
              "host": host.id,
              "success": true,
            })
          }
        )
    })
    .collect();

  let out = json!(results);
  println!("{}", out);

  Ok(())
}

fn send_file(
  hosts: Vec<Host>,
  local_path: String,
  remote_path: String,
  file_mode: i32
) -> Result<()> {
  let results: Vec<Value> = hosts
    .iter()
    .map(|host| {
      let file_size = fs::metadata(&local_path)?.len();
      Ok((host, file_size))
    })
    .collect::<Result<Vec<(&Host, u64)>>>()?
    .into_iter()
    .map(|(host, file_size)| {
      host.send_file(local_path.clone(), remote_path.clone(), file_mode, file_size)
        .map_or_else(
          |err| {
            json!({
              "host": host.id,
              "success": false,
              "error":format!("{}", err),
            })
          },
          |()| {
            json!({
              "host": host.id,
              "success": true,
            })
          }
        )
    })
    .collect();

  let out = json!(results);
  println!("{}", out);

  Ok(())
}

fn get_local_path(arg: Option<&str>) -> Result<String> {
  if let Some(path) = arg {
    let local_path = Path::new(path);

    if !local_path.exists() {
      Err(Box::new(Error::new(
        ErrorKind::NotFound,
        format!("No such file: {}", path),
      )))
    }
    else if local_path.is_dir() {
      Err(Box::new(Error::new(
        ErrorKind::InvalidInput,
        format!("Path is a directory, not a file: {}", path),
      )))
    }
    else {
      Ok(String::from(path))
    }
  }
  else {
    Err(Box::new(Error::new(
      ErrorKind::Other,
      "No input file provided",
    )))
  }
}

fn get_remote_path(arg: Option<&str>) -> Result<String> {
  if let Some(path) = arg {
    Ok(String::from(path))
  }
  else {
    Err(Box::new(Error::new(
      ErrorKind::Other,
      "No input file provided",
    )))
  }
}

fn get_file_mode(arg: Option<&str>) -> Result<i32> {
  let mut mode = Mode::from(0o644);

  if let Some(mode_str) = arg {
    mode.set_str(mode_str)?;
  }

  let file_mode = i32::try_from(mode.mode())?;
  Ok(file_mode)
}
