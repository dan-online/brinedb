use neon::{context::Context, result::NeonResult};
use once_cell::sync::OnceCell;
use async_once_cell::OnceCell as AsyncOnceCell;
use sea_orm::{Database, DatabaseConnection};
use tokio::runtime::Runtime;

pub fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

pub async fn connection<'a>(
    connection_uri: String,
) -> Result<&'static DatabaseConnection, sea_orm::DbErr> {
    static RUNTIME: AsyncOnceCell<DatabaseConnection> = AsyncOnceCell::new();

    RUNTIME
        .get_or_try_init(async { Database::connect(connection_uri).await })
        .await
}