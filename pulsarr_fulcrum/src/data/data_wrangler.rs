use rocket::serde::json::Json;
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgRow;
use crate::data::models::Model;
use crate::error::PulsarrError;

pub async fn add<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<T> {
    match object.add::<T>()
        .fetch_one(pool)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}