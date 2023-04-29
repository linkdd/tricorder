use crate::prelude::*;
use crate::tasks::module;

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
    let data_dir = matches.value_of("data_file_path").unwrap().to_owned();
    let _all = matches.value_of("all");
    let module_name = matches.value_of("module").unwrap().to_owned();
    let parallel = matches.is_present("parallel");

    let task = module::Task::new(data_dir, module_name);

    let res = hosts.run_task(&task, parallel)?;
    println!("{}", res);

    Ok(())
}