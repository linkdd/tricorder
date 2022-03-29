//! Command Line Interface to the `tricorder::tasks::exec` task.
//!
//! Usage:
//!
//! ```shell
//! $ tricorder -i inventory do -- echo "run on all hosts"
//! $ tricorder -i inventory -H foo do -- echo "run only on host 'foo'"
//! $ tricorder -i inventory -t myapp do -- echo "run only on hosts tagged with 'myapp'"
//! ```
//!
//! Commands can be templated with data from the host as defined in the
//! inventory:
//!
//! ```shell
//! $ tricorder -i inventory do -- echo "{host.id} says {host.vars.msg}"
//! ```

use crate::core::{Result, Host};
use crate::tasks::{TaskRunner, exec};

use clap::ArgMatches;

pub fn run(hosts: Vec<Host>, matches: &ArgMatches) -> Result<()> {
  let cmd_tmpl = get_command(matches.values_of("cmd"));
  let task = exec::Task::new(cmd_tmpl);
  let res = hosts.run_task_seq(&task)?;
  println!("{}", res);

  Ok(())
}

fn get_command(arg: Option<clap::Values<'_>>) -> String {
  arg
    .map(|vals| vals.collect::<Vec<_>>())
    .map(|argv| shell_words::join(argv))
    .unwrap()
}
