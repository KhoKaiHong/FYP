use crate::context::Context;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbModelController {
    const TABLE: &'static str;
}

pub async fn get<MC, E>(_context: &Context, model_manager: &ModelManager, id: i64) -> Result<E>
where
    MC: DbModelController,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = model_manager.db();

    let sql = format!("SELECT * from {} where id = $1", MC::TABLE);

    let entity: E = sqlx::query_as(&sql)
        .bind(id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

pub async fn list<MC, E>(_context: &Context, model_manager: &ModelManager) -> Result<Vec<E>>
where
    MC: DbModelController,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = model_manager.db();

    let sql = format!("SELECT * from {} ORDER BY id", MC::TABLE);

    let entity: Vec<E> = sqlx::query_as(&sql)
        .fetch_all(db)
        .await?;

    Ok(entity)
}

