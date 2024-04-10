use brinedb_entity::doc::ActiveModel as ActiveDocumentModel;
use brinedb_entity::doc::Column as DocumentColumn;
use brinedb_entity::doc::Entity as Document;
use migration::Migrator;
use migration::MigratorTrait;
use migration::OnConflict;
use napi::bindgen_prelude::*;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::Set;

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
    let connection = sea_orm::Database::connect(&self.connection_uri)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

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

    let res = Document::find_by_id(&key)
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

    let doc = ActiveDocumentModel {
      key: Set(key),
      value: Set(value),
    };

    Document::insert(doc)
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
  pub async fn get_many(&self, keys: Vec<String>) -> Result<Vec<Vec<String>>> {
    let connection = self
      .connection
      .as_ref()
      .ok_or_else(|| Error::new(Status::GenericFailure, "No connection found"))?;

    let res = Document::find()
      .filter(DocumentColumn::Key.is_in(keys))
      .all(connection)
      .await
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(
      res
        .into_iter()
        .map(|doc| vec![doc.key, doc.value])
        .collect(),
    )
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

    let res = Document::find()
      .select_only()
      .column(DocumentColumn::Key)
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

    let res = Document::find()
      .select_only()
      .column(DocumentColumn::Value)
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
