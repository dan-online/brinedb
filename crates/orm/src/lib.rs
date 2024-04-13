use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::error::Error;

pub mod models;
pub mod schema;

use models::*;
use schema::brine::dsl::*;

#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
  Postgresql(diesel::PgConnection),
  Mysql(diesel::MysqlConnection),
  Sqlite(diesel::SqliteConnection),
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DieselBrine {
  connection: Option<AnyConnection>,
}

impl DieselBrine {
  pub fn new() -> Self {
    DieselBrine { connection: None }
  }

  pub fn connect(&mut self, uri: &str) -> Result<(), Box<dyn Error>> {
    let connection = match uri {
      uri if uri.starts_with("postgres") => {
        let connection = diesel::PgConnection::establish(uri)?;
        AnyConnection::Postgresql(connection)
      }
      uri if uri.starts_with("mysql") => {
        let connection = diesel::MysqlConnection::establish(uri)?;
        AnyConnection::Mysql(connection)
      }
      uri if uri.starts_with("sqlite") => {
        let connection = diesel::SqliteConnection::establish(uri)?;
        AnyConnection::Sqlite(connection)
      }
      _ => return Err("Unsupported database".into()),
    };

    self.connection = Some(connection);

    Ok(())
  }

  pub fn run_migrations(&mut self) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    match connection {
      AnyConnection::Postgresql(connection) => {
        connection
          .run_pending_migrations(MIGRATIONS)
          .map_err(|err| err.to_string())?;
      }
      AnyConnection::Mysql(connection) => {
        connection
          .run_pending_migrations(MIGRATIONS)
          .map_err(|err| err.to_string())?;
      }
      AnyConnection::Sqlite(connection) => {
        connection
          .run_pending_migrations(MIGRATIONS)
          .map_err(|err| err.to_string())?;

        connection
          .batch_execute("PRAGMA journal_mode=WAL; PRAGMA synchronous=normal;")
          .expect("Error setting journal mode");
      }
    }

    Ok(())
  }

  pub fn get(&mut self, get_key: String) -> Result<Option<String>, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let result = brine
      .filter(key.eq(get_key))
      .select(val)
      .first::<String>(connection);

    if let Ok(doc_value) = result {
      Ok(Some(doc_value))
    } else {
      Ok(None)
    }
  }

  pub fn set(&mut self, set_key: String, set_value: String) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let new_brine = NewBrine {
      key: &set_key,
      val: &set_value,
    };

    match connection {
      AnyConnection::Postgresql(connection) => {
        diesel::insert_into(brine)
          .values(&new_brine)
          .on_conflict(key)
          .do_update()
          .set(val.eq(excluded(val)))
          .execute(connection)
          .expect("Error saving new document");
      }
      AnyConnection::Sqlite(connection) => {
        diesel::insert_into(brine)
          .values(&new_brine)
          .on_conflict(key)
          .do_update()
          .set(val.eq(excluded(val)))
          .execute(connection)
          .expect("Error saving new document");
      }
      AnyConnection::Mysql(connection) => {
        diesel::insert_into(brine)
          .values(&new_brine)
          .on_conflict(diesel::dsl::DuplicatedKeys)
          .do_update()
          .set(val.eq(&set_value))
          .execute(connection)
          .expect("Error saving new document");
      }
    }

    Ok(())
  }

  pub fn set_many(&mut self, set_values: Vec<(String, String)>) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let set_values = set_values
      .iter()
      .map(|(doc_key, doc_value)| NewBrine {
        key: doc_key,
        val: doc_value,
      })
      .collect::<Vec<_>>();

    match connection {
      AnyConnection::Postgresql(connection) => {
        diesel::insert_into(brine)
          .values(&set_values)
          .on_conflict(key)
          .do_update()
          .set(val.eq(excluded(val)))
          .execute(connection)
          .expect("Error saving new documents");
      }
      AnyConnection::Sqlite(connection) => {
        for new_brine in set_values {
          diesel::insert_into(brine)
            .values(&new_brine)
            .on_conflict(key)
            .do_update()
            .set(val.eq(excluded(val)))
            .execute(connection)
            .expect("Error saving new documents");
        }
      }
      AnyConnection::Mysql(connection) => {
        for new_brine in set_values {
          diesel::insert_into(brine)
            .values(&new_brine)
            .on_conflict(diesel::dsl::DuplicatedKeys)
            .do_update()
            .set(val.eq(&new_brine.val))
            .execute(connection)
            .expect("Error saving new documents");
        }
      }
    }

    Ok(())
  }

  pub fn get_many(&mut self, get_keys: Vec<String>) -> Result<Vec<(String, String)>, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let result = brine
      .filter(key.eq_any(get_keys))
      .select((key, val))
      .load::<(String, String)>(connection);

    match result {
      Ok(docs) => Ok(docs),
      Err(_) => Err("Error fetching documents".to_string()),
    }
  }

  pub fn clear(&mut self) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    diesel::delete(brine)
      .execute(connection)
      .expect("Error deleting documents");

    Ok(())
  }

  pub fn delete(&mut self, delete_key: String) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    diesel::delete(brine.filter(key.eq(delete_key)))
      .execute(connection)
      .expect("Error deleting document");

    Ok(())
  }

  pub fn delete_many(&mut self, delete_keys: Vec<String>) -> Result<(), String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    diesel::delete(brine.filter(key.eq_any(delete_keys)))
      .execute(connection)
      .expect("Error deleting documents");

    Ok(())
  }

  pub fn keys(&mut self) -> Result<Vec<String>, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let result = brine.select(key).load::<String>(connection);

    match result {
      Ok(keys) => Ok(keys),
      Err(_) => Err("Error fetching keys".to_string()),
    }
  }

  pub fn values(&mut self) -> Result<Vec<String>, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let result = brine.select(val).load::<String>(connection);

    match result {
      Ok(values) => Ok(values),
      Err(_) => Err("Error fetching values".to_string()),
    }
  }

  pub fn count(&mut self) -> Result<i64, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;

    let result = brine.count().get_result(connection);

    match result {
      Ok(count) => Ok(count),
      Err(_) => Err("Error counting documents".to_string()),
    }
  }

  pub fn has(&mut self, has_key: String) -> Result<bool, String> {
    let connection = self.connection.as_mut().ok_or("Not connected")?;
    let sub_query = brine.filter(key.eq(has_key));

    let exists = diesel::select(diesel::dsl::exists(sub_query)).get_result::<bool>(connection);

    match exists {
      Ok(exists) => Ok(exists),
      Err(_) => Ok(false),
    }
  }

  pub fn close(&mut self) -> Result<(), String> {
    if self.connection.is_none() {
      return Ok(());
    }

    let connection = self.connection.as_mut().ok_or("Not connected")?;

    match connection {
      AnyConnection::Postgresql(connection) => {
        diesel::sql_query("VACUUM;")
          .execute(connection)
          .expect("Error vacuuming database");
      }
      AnyConnection::Mysql(connection) => {
        diesel::sql_query("OPTIMIZE TABLE brine;")
          .execute(connection)
          .expect("Error optimizing table");
      }
      AnyConnection::Sqlite(connection) => {
        diesel::sql_query("PRAGMA optimize;")
          .execute(connection)
          .expect("Error optimizing database");
      }
    }

    self.connection = None;

    Ok(())
  }
}

#[cfg(test)]
pub mod tests;
