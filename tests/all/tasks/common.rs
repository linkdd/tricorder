use tricorder::prelude::*;
use std::env;

pub fn setup_context() -> Result<Inventory> {
  let cwd = env::current_dir()?;
  let test_dir = cwd.join("tests").join("tricorder");
  env::set_current_dir(test_dir)?;

  let inventory = Inventory::from_file("./inventory.toml")?;

  Ok(inventory)
}
