use rocket::serde::json::Json;
use sqlx::PgPool;
use crate::data::models::Model;
use crate::error::PulsarrError;

pub async fn add<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<bool> {
    match object.add(pool).await {
        (true, _) => Ok(Json(true)),
        (false, error_message) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: error_message,
            http_status_code: 400,
        }),
    }
}