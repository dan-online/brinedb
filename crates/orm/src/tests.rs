use super::*;

#[test]
fn test_brine_can_connect() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
}

#[test]
fn test_brine_can_run_migrations() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();
}

#[test]
fn test_brine_can_set_and_get() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  brinedb.set("key".to_string(), "value".to_string()).unwrap();
  let value = brinedb.get("key".to_string()).unwrap().unwrap();

  assert_eq!(value, "value");
}

#[test]
fn test_brine_can_set_many_and_get_many() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  let set_values = vec![
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
  ];

  brinedb.set_many(set_values).unwrap();
  let get_values = brinedb
    .get_many(vec!["key1".to_string(), "key2".to_string()])
    .unwrap();

  assert_eq!(
    get_values,
    vec![
      ("key1".to_string(), "value1".to_string()),
      ("key2".to_string(), "value2".to_string())
    ]
  );
}

#[test]
fn test_brine_can_clear() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  brinedb.set("key".to_string(), "value".to_string()).unwrap();
  brinedb.clear().unwrap();

  let count = brinedb.count().unwrap();

  assert_eq!(count, 0);
}

#[test]
fn test_brine_can_delete() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  brinedb.set("key".to_string(), "value".to_string()).unwrap();
  brinedb.delete("key".to_string()).unwrap();

  let count = brinedb.count().unwrap();

  assert_eq!(count, 0);
}

#[test]
fn test_brine_can_delete_many() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  let set_values = vec![
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
  ];

  brinedb.set_many(set_values).unwrap();
  brinedb
    .delete_many(vec!["key1".to_string(), "key2".to_string()])
    .unwrap();

  let count = brinedb.count().unwrap();

  assert_eq!(count, 0);
}

#[test]
fn test_brine_can_get_keys() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  let set_values = vec![
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
  ];

  brinedb.set_many(set_values).unwrap();
  let keys = brinedb.keys().unwrap();

  assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
}

#[test]
fn test_brine_can_get_values() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  let set_values = vec![
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
  ];

  brinedb.set_many(set_values).unwrap();
  let values = brinedb.values().unwrap();

  assert_eq!(values, vec!["value1".to_string(), "value2".to_string()]);
}

#[test]
fn test_brine_can_count() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  let set_values = vec![
    ("key1".to_string(), "value1".to_string()),
    ("key2".to_string(), "value2".to_string()),
  ];

  brinedb.set_many(set_values).unwrap();
  let count = brinedb.count().unwrap();

  assert_eq!(count, 2);
}

#[test]
fn test_brine_can_has_key() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();

  brinedb.set("key".to_string(), "value".to_string()).unwrap();
  let has_key = brinedb.has("key".to_string()).unwrap();

  assert_eq!(has_key, true);
}

#[test]
fn test_brine_can_close_connection() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();
  brinedb.close().unwrap();
}

#[test]
fn test_brine_can_close_connection_twice() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();
  brinedb.close().unwrap();
  brinedb.close().unwrap();
}

#[test]
fn test_brine_rejects_after_close() {
  let mut brinedb = DieselBrine::new();
  brinedb.connect("sqlite://:memory:").unwrap();
  brinedb.run_migrations().unwrap();
  brinedb.close().unwrap();

  let result = brinedb.get("key".to_string());

  assert!(result.is_err());
}
