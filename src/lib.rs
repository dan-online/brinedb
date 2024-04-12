use brinedb_orm::DieselBrine;
use napi::bindgen_prelude::*;
use std::collections::HashMap;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct BrineDB {
  pub connection_uri: String,
  connection: Option<DieselBrine>,
}

#[napi]
impl BrineDB {
  #[napi(constructor)]
  pub fn new(connection_uri: String) -> Self {
    BrineDB {
      connection_uri,
      connection: None,
    }
  }

  #[napi]
  pub fn connect(&mut self) -> Result<bool> {
    let mut connection = DieselBrine::new();

    connection
      .connect(&self.connection_uri)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    connection
      .run_migrations()
      .map_err(|err| Error::new(Status::GenericFailure, err))?;

    self.connection = Some(connection);

    Ok(true)
  }

  #[napi]
  pub fn migrate(&mut self) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .run_migrations()
      .map_err(|err| Error::new(Status::GenericFailure, err))?;

    Ok(())
  }

  #[napi]
  pub fn get(&mut self, key: String) -> Result<Option<String>> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    return connection
      .get(key)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()));
  }

  #[napi]
  pub fn set(&mut self, key: String, value: String) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .set(key, value)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn set_many(&mut self, data: Vec<(String, String)>) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .set_many(data)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn get_many(&mut self, keys: Vec<String>) -> Result<HashMap<String, Option<String>>> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let res = connection
      .get_many(keys.clone())
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    let mut res = res
      .into_iter()
      .map(|(key, val)| (key, Some(val)))
      .collect::<HashMap<_, _>>();

    for key in keys {
      if !res.contains_key(&key) {
        res.insert(key, None);
      }
    }

    Ok(res)
  }

  #[napi]
  pub fn clear(&mut self) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .clear()
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn delete(&mut self, key: String) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .delete(key)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn delete_many(&mut self, keys: Vec<String>) -> Result<()> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .delete_many(keys)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn keys(&mut self) -> Result<Vec<String>> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .keys()
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn values(&mut self) -> Result<Vec<String>> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .values()
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn count(&mut self) -> Result<i64> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .count()
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn has(&mut self, key: String) -> Result<bool> {
    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection
      .has(key)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))
  }

  #[napi]
  pub fn close(&mut self) -> Result<()> {
    if self.connection.is_none() {
      return Ok(());
    }

    let connection = self
      .connection
      .as_mut()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    connection.close().unwrap();

    self.connection = None;

    Ok(())
  }
}
