use std::{env, panic};
use tricorder::prelude::*;

pub fn within_context<T>(test_fn: T) -> ()
where
    T: FnOnce(Inventory) -> () + panic::UnwindSafe,
{
    let cwd = env::current_dir().unwrap();
    let test_dir = cwd.join("tests").join("tricorder");
    env::set_current_dir(test_dir).unwrap();

    let inventory = Inventory::from_file("./inventory.toml").unwrap();

    let result = panic::catch_unwind(|| test_fn(inventory));

    env::set_current_dir(cwd).unwrap();

    assert!(result.is_ok());
}
