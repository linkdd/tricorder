use tricorder::core::{Inventory, Host, HostId, HostTag};

#[test]
fn new_should_create_an_empty_inventory() {
  let inventory = Inventory::new();

  assert_eq!(inventory.hosts.len(), 0);
}

#[test]
fn add_host_should_work() {
  let inventory = Inventory::new()
    .add_host(
      Host::new(HostId::new("example-0").unwrap(), "127.0.1.1:22".to_string())
    )
    .to_owned();

  assert_eq!(inventory.hosts.len(), 1);

  let host = inventory.get_host_by_id(HostId::new("example-0").unwrap())
    .expect("host example-0 should exist");

  assert_eq!(host.address, String::from("127.0.1.1:22"));
}

#[test]
fn remove_host_should_work() {
  let inventory = Inventory::new()
    .add_host(
      Host::new(HostId::new("example-0").unwrap(), "127.0.1.1:22".to_string())
    )
    .add_host(
      Host::new(HostId::new("example-1").unwrap(), "127.0.1.2:22".to_string())
    )
    .remove_host(HostId::new("example-1").unwrap())
    .to_owned();

  assert_eq!(inventory.hosts.len(), 1);

  let host = inventory.get_host_by_id(HostId::new("example-0").unwrap())
    .expect("host example-0 should exist");

  assert_eq!(host.address, String::from("127.0.1.1:22"));
}

#[test]
fn from_toml_should_return_an_inventory() {
  let content = r#"
  [[hosts]]

  id = "example-0"
  address = "127.0.1.1:22"
  "#;

  match Inventory::from_toml(content) {
    Ok(inventory) => {
      assert_eq!(inventory.hosts.len(), 1);

      let host = inventory.get_host_by_id(HostId::new("example-0").unwrap())
        .expect("host example-0 should exist");

      assert_eq!(host.address, String::from("127.0.1.1:22"));
    },
    Err(err) => {
      assert!(false, "error while parsing TOML: {}", err);
    }
  }
}

#[test]
fn from_toml_should_fail_on_invalid_content() {
  let content = "{this is not valid toml}";

  match Inventory::from_toml(content) {
    Ok(_) => assert!(false, "invalid TOML should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn from_toml_should_fail_on_invalid_hostid() {
  let content = r#"
  [[hosts]]

  id = "example-0$"
  address = "127.0.1.1:22"
  "#;

  match Inventory::from_toml(content) {
    Ok(_) => assert!(false, "invalid id should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn from_toml_should_fail_on_invalid_hosttag() {
  let content = r#"
  [[hosts]]

  id = "example-0"
  address = "127.0.1.1:22"
  tags = ["&foo"]
  "#;

  match Inventory::from_toml(content) {
    Ok(_) => assert!(false, "invalid tag should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn from_json_should_return_an_inventory() {
  let content = r#"
  {"hosts": [
    {
      "id": "example-0",
      "address": "127.0.1.1:22"
    }
  ]}
  "#;

  match Inventory::from_json(content) {
    Ok(inventory) => {
      assert_eq!(inventory.hosts.len(), 1);

      let host = inventory.get_host_by_id(HostId::new("example-0").unwrap())
        .expect("host example-0 should exist");

      assert_eq!(host.address, String::from("127.0.1.1:22"));
    },
    Err(err) => {
      assert!(false, "error while parsing JSON: {}", err);
    }
  }
}

#[test]
fn from_json_should_fail_on_invalid_content() {
  let content = "{this is not valid toml}";

  match Inventory::from_json(content) {
    Ok(_) => assert!(false, "invalid JSON should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn from_json_should_fail_on_invalid_hostid() {
  let content = r#"
  {"hosts": [
    {
      "id": "example-0$",
      "address": "127.0.1.1:22"
    }
  ]}
  "#;

  match Inventory::from_json(content) {
    Ok(_) => assert!(false, "invalid id should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn from_json_should_fail_on_invalid_hosttag() {
  let content = r#"
  {"hosts": [
    {
      "id": "example-0",
      "address": "127.0.1.1:22",
      "tags": ["&foo"]
    }
  ]}
  "#;

  match Inventory::from_json(content) {
    Ok(_) => assert!(false, "invalid tag should not be parsed"),
    Err(_) => assert!(true)
  };
}

#[test]
fn get_host_by_tags_should_work() {
  let inventory = Inventory::new()
    .add_host(
      Host::new(HostId::new("example-0").unwrap(), "127.0.1.1:22".to_string())
        .add_tag(HostTag::new("foo").unwrap())
        .to_owned()
    )
    .add_host(
      Host::new(HostId::new("example-1").unwrap(), "127.0.1.2:22".to_string())
      .add_tag(HostTag::new("bar").unwrap())
      .to_owned()
    )
    .to_owned();

  assert_eq!(inventory.hosts.len(), 2);

  let foo_hosts = inventory.get_hosts_by_tags("foo".to_string()).unwrap();
  let bar_hosts = inventory.get_hosts_by_tags("bar".to_string()).unwrap();

  assert_eq!(foo_hosts.len(), 1);
  match foo_hosts.get(0) {
    Some(host) => {
      assert_eq!(host.id, HostId::new("example-0").unwrap());
    },
    None => {
      assert!(false, "Expected host example-0 not found");
    }
  }

  assert_eq!(bar_hosts.len(), 1);
  match bar_hosts.get(0) {
    Some(host) => {
      assert_eq!(host.id, HostId::new("example-1").unwrap());
    },
    None => {
      assert!(false, "Expected host example-1 not found");
    }
  }
}
