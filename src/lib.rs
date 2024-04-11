use brinedb_entity::doc::ActiveModel as ActiveDocumentModel;
use brinedb_entity::doc::Column as DocumentColumn;
use brinedb_entity::doc::Entity as Document;
use brinedb_entity::sea_orm;
use brinedb_entity::sea_orm::ColumnTrait;
use brinedb_entity::sea_orm::ConnectOptions;
use brinedb_entity::sea_orm::ConnectionTrait;
use brinedb_entity::sea_orm::Database;
use brinedb_entity::sea_orm::DatabaseConnection;
use brinedb_entity::sea_orm::EntityTrait;
use brinedb_entity::sea_orm::PaginatorTrait;
use brinedb_entity::sea_orm::QueryFilter;
use brinedb_entity::sea_orm::QuerySelect;
use brinedb_entity::sea_orm::Set;
use migration::sea_orm::FromQueryResult;
use migration::Migrator;
use migration::MigratorTrait;
use migration::OnConflict;
use napi::bindgen_prelude::*;
use std::collections::HashMap;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct BrineDB {
  pub connection_uri: String,
  connection: Option<DatabaseConnection>,
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
  pub async unsafe fn connect(&mut self) -> Result<bool> {
    let opt = ConnectOptions::new(&self.connection_uri);

    let connection = Database::connect(opt)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    if self.connection_uri.starts_with("sqlite") {
      let pragmas = vec!["PRAGMA journal_mode = wal;", "PRAGMA synchronous = 1;"];

      for pragma in pragmas {
        connection
          .execute_unprepared(pragma)
          .await
          .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;
      }
    }

    self.connection = Some(connection);

    Ok(true)
  }

  #[napi]
  pub async fn migrate(&self) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    Migrator::up(connection, None)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn get(&self, key: String) -> Result<Option<String>> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    #[derive(FromQueryResult)]
    struct GetDoc {
      value: String,
    }

    let res = Document::find_by_id(&key)
      .into_model::<GetDoc>()
      .one(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    if let Some(doc) = res {
      Ok(Some(doc.value))
    } else {
      Ok(None)
    }
  }

  #[napi]
  pub async fn set(&self, key: String, value: String) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let model = ActiveDocumentModel {
      key: Set(key),
      value: Set(value),
    };

    Document::insert(model.clone())
      .on_conflict(
        OnConflict::column(DocumentColumn::Key)
          .update_column(DocumentColumn::Value)
          .to_owned(),
      )
      .exec(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn set_many(&self, data: Vec<(String, String)>) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let docs = data
      .into_iter()
      .map(|(key, value)| ActiveDocumentModel {
        key: Set(key),
        value: Set(value),
      })
      .collect::<Vec<_>>();

    Document::insert_many(docs)
      .on_conflict(
        OnConflict::column(DocumentColumn::Key)
          .update_column(DocumentColumn::Value)
          .to_owned(),
      )
      .exec(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn get_many(&self, keys: Vec<String>) -> Result<HashMap<String, Option<String>>> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let res = Document::find()
      .filter(DocumentColumn::Key.is_in(&keys))
      .all(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    let mut res = res
      .into_iter()
      .map(|doc| (doc.key, Some(doc.value)))
      .collect::<HashMap<_, _>>();

    for key in keys {
      if !res.contains_key(&key) {
        res.insert(key, None);
      }
    }

    Ok(res)
  }

  #[napi]
  pub async fn clear(&self) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    Document::delete_many()
      .exec(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn delete(&self, key: String) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let model = ActiveDocumentModel {
      key: Set(key.clone()),
      ..Default::default()
    };

    Document::delete(model)
      .exec(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn delete_many(&self, keys: Vec<String>) -> Result<()> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    Document::delete_many()
      .filter(DocumentColumn::Key.is_in(keys))
      .exec(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(())
  }

  #[napi]
  pub async fn keys(&self) -> Result<Vec<String>> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    #[derive(FromQueryResult)]
    struct KeysOnlyDoc {
      key: String,
    }

    let res = Document::find()
      .select_only()
      .column(DocumentColumn::Key)
      .into_model::<KeysOnlyDoc>()
      .all(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(res.into_iter().map(|doc| doc.key).collect())
  }

  #[napi]
  pub async fn values(&self) -> Result<Vec<String>> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    #[derive(FromQueryResult)]
    struct ValuesOnlyDoc {
      value: String,
    }

    let res = Document::find()
      .select_only()
      .column(DocumentColumn::Value)
      .into_model::<ValuesOnlyDoc>()
      .all(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(res.into_iter().map(|doc| doc.value).collect())
  }

  #[napi]
  pub async fn count(&self) -> Result<i64> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let count = Document::find()
      .count(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(count as i64)
  }

  #[napi]
  pub async fn has(&self, key: String) -> Result<bool> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let count = Document::find()
      .filter(DocumentColumn::Key.eq(key))
      .count(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(count > 0)
  }

  #[napi]
  pub async unsafe fn close(&mut self) -> Result<()> {
    self.connection = None;

    Ok(())
  }
}
