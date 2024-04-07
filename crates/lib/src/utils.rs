use async_once_cell::OnceCell as AsyncOnceCell;
use neon::{context::Context, result::NeonResult};
use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};
use tokio::runtime::Runtime;

static RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

pub static mut DATABASE: AsyncOnceCell<DatabaseConnection> = AsyncOnceCell::new();
pub static mut DATABASE_URI: OnceCell<String> = OnceCell::new();

pub async fn connection<'a>(
    connection_uri: String,
) -> Result<&'static DatabaseConnection, sea_orm::DbErr> {
    unsafe {
        let stored_uri = DATABASE_URI.get_or_init(|| connection_uri.clone());

        if stored_uri != &connection_uri {
            let old_connection = DATABASE.take();

            if let Some(old_connection) = old_connection {
                old_connection.close().await?;
            }

            DATABASE_URI.take();
            DATABASE_URI.set(connection_uri.clone()).unwrap();
        }

        DATABASE
            .get_or_try_init(async { Database::connect(connection_uri).await })
            .await
    }
}
