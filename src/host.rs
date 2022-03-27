//! **tricorder** applies commands on multiple hosts found in the inventory.
//!
//! It relies on SSH and `ssh-agent` to connect to a remote host.

use crate::Result;

use serde_json::Value;
use serde_derive::{Serialize, Deserialize};
use ssh2::Session;

use std::{
  collections::HashMap,
  io::{
    prelude::*,
    BufRead,
    BufReader,
  },
  net::TcpStream,
  fs::File,
  path::Path,
};

/// Abstraction of a host found in the inventory
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Host {
  /// Host identifier
  pub id: String,
  /// SSH host address in the form of `hostname:port`
  pub address: String,
  /// SSH user to authenticate with
  #[serde(default = "Host::default_user")]
  pub user: String,
  /// Tags used to apply commands on a subset of hosts from the inventory.
  #[serde(default = "Host::default_tags")]
  pub tags: Vec<String>,
  /// Variables specific to this host, used by templates.
  #[serde(default = "Host::default_vars")]
  pub vars: HashMap<String, Value>,
}

impl Host {
  /// Execute a command on this host.
  pub fn exec(&self, command: &str) -> Result<(i32, String)> {
    let sess = self.get_session()?;
    let mut channel = sess.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;

    let exit_code = channel.exit_status()?;

    Ok((exit_code, output))
  }

  /// Write data to a file on this host.
  pub fn send_data(
    &self,
    remote_path: String,
    file_mode: i32,
    file_size: u64,
    data: String,
  ) -> Result<()> {
    let sess = self.get_session()?;
    let mut channel = sess.scp_send(
      Path::new(&remote_path),
      file_mode,
      file_size,
      None
    )?;

    channel.write(data.as_bytes())?;
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;

    Ok(())
  }

  /// Copy a file from this machine to this remote host.
  pub fn send_file(
    &self,
    local_path: String,
    remote_path: String,
    file_mode: i32,
    file_size: u64,
  ) -> Result<()> {
    let sess = self.get_session()?;
    let mut channel = sess.scp_send(
      Path::new(&remote_path),
      file_mode,
      file_size,
      None
    )?;

    let file = File::open(&local_path)?;
    let block_size = 4 * 1024 * 1024; // 4 megabytes
    let mut reader = BufReader::with_capacity(block_size, file);

    loop {
      let buffer = reader.fill_buf()?;
      let length = buffer.len();

      if length > 0 {
        channel.write(buffer)?;
      }
      else {
        channel.send_eof()?;
        channel.wait_eof()?;
        channel.close()?;
        channel.wait_close()?;
        break;
      }

      reader.consume(length);
    }

    Ok(())
  }

  fn get_session(&self) -> Result<Session> {
    let sock = TcpStream::connect(self.address.clone())?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(sock);
    sess.handshake()?;
    sess.userauth_agent(&self.user)?;

    Ok(sess)
  }

  /// If not provided, `user` defaults to `root`.
  pub fn default_user() -> String {
    String::from("root")
  }

  /// If not provided, `tags` defaults to an empty list.
  pub fn default_tags() -> Vec<String> {
    vec![]
  }

  /// If not provided, `vars` defaults to an empty hashmap.
  pub fn default_vars() -> HashMap<String, Value> {
    HashMap::new()
  }
}
