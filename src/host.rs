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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Host {
  pub id: String,
  pub address: String,
  #[serde(default = "Host::default_user")]
  pub user: String,
  #[serde(default = "Host::default_tags")]
  pub tags: Vec<String>,
  #[serde(default = "Host::default_vars")]
  pub vars: HashMap<String, Value>,
}

impl Host {
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

  pub fn default_user() -> String {
    String::from("root")
  }

  pub fn default_tags() -> Vec<String> {
    vec![]
  }

  pub fn default_vars() -> HashMap<String, Value> {
    HashMap::new()
  }
}
