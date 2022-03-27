pub mod inventory;
pub mod host;
pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub use self::{
  inventory::Inventory,
  host::Host,
};
