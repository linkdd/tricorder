use crate::prelude::*;
use crate::tasks::module;

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
    let data_path = get_data_path(matches.value_of("data_file_path")); // implement with &str and livetimes
    let module_path = get_path(matches.value_of("module"))?;
    let parallel = matches.is_present("parallel");

    let task = module::Task::new(data_path, module_path);

    let res = hosts.run_task(&task, parallel)?;
    println!("{}", res);

    Ok(())
}

fn get_path(arg: Option<&str>) -> Result<String> {
    if let Some(path) = arg {
        Ok(String::from(path))
    } else {
        Err(Box::new(Error::MissingInput(
            "No input file provided".to_string(),
        )))
    }
}

fn get_data_path(arg: Option<&str>) -> Option<String> {
    match arg {
        Some(arg) => {
            Some(String::from(arg))
        },
        None => {
            None
        }
    }
}
