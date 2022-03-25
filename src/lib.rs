extern crate toml;
extern crate serde_json;
extern crate serde_derive;

extern crate clap;
extern crate ssh2;
extern crate is_executable;
extern crate shell_words;

pub mod inventory;
pub mod host;
pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use self::{
  inventory::Inventory,
  host::Host,
};
