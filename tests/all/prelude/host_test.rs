use tricorder::prelude::Host;
use serde_json::json;

#[test]
fn hostid_should_return_ok_with_valid_id() {
  let result = Host::id("valid");
  assert!(result.is_ok());
}

#[test]
fn hostid_should_return_err_with_invalid_id() {
  let result = Host::id("(invalid)");
  assert!(result.is_err());
}

#[test]
fn hosttag_should_return_ok_with_valid_tag() {
  let result = Host::tag("valid");
  assert!(result.is_ok());
}

#[test]
fn hosttag_should_return_err_with_invalid_tag() {
  let result = Host::tag("(invalid)");
  assert!(result.is_err());
}

#[test]
fn new_should_use_default_values() {
  let host = Host::new(
    Host::id("localhost").unwrap(),
    "localhost:8022".to_string(),
  );

  assert_eq!(host.id.to_string().as_str(), "localhost");
  assert_eq!(host.address.as_str(), "localhost:8022");
  assert_eq!(host.user.as_str(), "root");
  assert_eq!(host.tags.len(), 0);
  assert_eq!(host.vars.len(), 0);
}

#[test]
fn set_user_should_change_the_default_user() {
  let host = Host::new(Host::id("localhost").unwrap(), "localhost:8022".to_string())
    .set_user("admin".to_string())
    .to_owned();

  assert_eq!(host.user.as_str(), "admin");
}

#[test]
fn add_tag_should_add_a_tag() {
  let host = Host::new(Host::id("localhost").unwrap(), "localhost:8022".to_string())
    .add_tag(Host::tag("foo").unwrap())
    .to_owned();

  assert_eq!(host.tags.len(), 1);
  assert_eq!(host.tags.get(0), Some(&Host::tag("foo").unwrap()));
}

#[test]
fn remove_tag_should_remove_a_tag() {
  let host = Host::new(Host::id("localhost").unwrap(), "localhost:8022".to_string())
    .add_tag(Host::tag("foo").unwrap())
    .remove_tag(Host::tag("foo").unwrap())
    .to_owned();

  assert_eq!(host.tags.len(), 0);
}

#[test]
fn set_var_should_add_a_variable() {
  let host = Host::new(Host::id("localhost").unwrap(), "localhost:8022".to_string())
    .set_var("foo".to_string(), json!(42 as i32))
    .to_owned();

  assert_eq!(host.vars.get(&"foo".to_string()), Some(&json!(42 as i32)));
}

#[test]
fn remove_var_should_delete_a_variable() {
  let host = Host::new(Host::id("localhost").unwrap(), "localhost:8022".to_string())
    .set_var("foo".to_string(), json!(42 as i32))
    .remove_var("foo".to_string())
    .to_owned();

  assert_eq!(host.vars.get(&"foo".to_string()), None);
}
