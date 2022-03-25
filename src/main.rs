use tricorder::{Result, cli};
use serde_json::{json, Value};

fn main() -> Result<()> {
  let (hosts, cmd) = cli::parse_args()?;
  let results: Vec<Value> = hosts
    .iter()
    .map(|host| {
      eprintln!("Executing command on {}...", host.id);
      host.exec(&cmd)
        .map_or_else(
          |err| {
            json!({
              "host": host.id,
              "success": false,
              "error": format!("{}", err),
            })
          },
          |(exit_code, output)| {
            json!({
              "host": host.id,
              "success": true,
              "info": {
                "exit_code": exit_code,
                "output": output,
              },
            })
          }
        )
    })
    .collect();

  let out = json!(results);
  println!("{}", out);

  Ok(())
}
