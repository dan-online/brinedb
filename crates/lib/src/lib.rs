use brinedb_entity::doc::ActiveModel as ActiveDocumentModel;
use brinedb_entity::doc::Column as DocumentColumn;
use brinedb_entity::doc::Entity as Document;
use migration::Migrator;
use migration::MigratorTrait;
use neon::prelude::*;
use sea_orm::prelude::*;
use sea_orm::Set;

mod utils;

use utils::*;

fn migrate(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri)
            .await
            .map_err(|err| err.to_string());

        if let Err(err) = connection {
            deferred.settle_with(&channel, move |mut cx| {
                cx.throw_error::<String, Handle<'_, JsString>>(err)
            });
            return;
        }

        let connection = connection.unwrap();

        let res = Migrator::up(connection, None)
            .await
            .map_err(|err| err.to_string());

        deferred.settle_with(&channel, move |mut cx| match res {
            Ok(_) => Ok(cx.undefined()),
            Err(err) => cx.throw_error(err),
        });
    });

    Ok(promise)
}

fn get(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);
    let key = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        let model = Document::find_by_id(&key)
            .one(connection)
            .await
            .expect("Unable to find key");

        deferred.settle_with(&channel, move |mut cx| match model {
            Some(doc) => Ok(cx.string(doc.value)),
            None => cx.throw_error("Key not found"),
        });
    });

    Ok(promise)
}

fn set(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);
    let key = cx.argument::<JsString>(1)?.value(&mut cx);
    let value = cx.argument::<JsString>(2)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        let model = ActiveDocumentModel {
            key: Set(key.clone()),
            value: Set(value.clone()),
        };

        Document::insert(model)
            .exec(connection)
            .await
            .expect("Unable to insert key");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn clear(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        Document::delete_many()
            .exec(connection)
            .await
            .expect("Unable to clear all keys");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn del(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);
    let key = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        let model = ActiveDocumentModel {
            key: Set(key.clone()),
            ..Default::default()
        };

        Document::delete(model)
            .exec(connection)
            .await
            .expect("Unable to delete key");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn del_many(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);
    let keys = cx.argument::<JsArray>(1)?;

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    let mut keys_vec = vec![];

    for i in 0..keys.len(&mut cx) {
        let key = keys
            .get::<JsString, _, u32>(&mut cx, i)
            .expect("Invalid array entry")
            .value(&mut cx);

        keys_vec.push(key);
    }

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        Document::delete_many()
            .filter(DocumentColumn::Key.is_in(keys_vec))
            .exec(connection)
            .await
            .expect("Unable to delete keys");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn count(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        let count = Document::find()
            .count(connection)
            .await
            .expect("Unable to count keys");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.number(count as f64)));
    });

    Ok(promise)
}

fn has(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let connection_uri = cx.argument::<JsString>(0)?.value(&mut cx);
    let key = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let connection = connection(connection_uri).await.expect("Unable to connect");

        let has = Document::find()
            .filter(DocumentColumn::Key.eq(key))
            .count(connection)
            .await
            .expect("Unable to count keys");

        deferred.settle_with(&channel, move |mut cx| Ok(cx.boolean(has > 0)));
    });

    Ok(promise)
}

fn close(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        unsafe {
            let db = DATABASE.take();

            if let Some(db) = db {
                db.close().await.expect("Unable to close connection");
            }
        }

        deferred.settle_with(&channel, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("migrate", migrate)?;
    cx.export_function("get", get)?;
    cx.export_function("set", set)?;
    cx.export_function("clear", clear)?;
    cx.export_function("del", del)?;
    cx.export_function("delMany", del_many)?;
    cx.export_function("count", count)?;
    cx.export_function("has", has)?;
    cx.export_function("close", close)?;
    Ok(())
}
