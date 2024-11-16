use rocket::serde::json::Json;
use sqlx::PgPool;
use crate::data::models::Model;
use crate::error::PulsarrError;

pub async fn add<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<T> {
    match object.add::<T>().fetch_one(pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

pub async fn update<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<T>{
    match object.update::<T>().fetch_one(pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

pub async fn delete<T: Model>(id: i32, pool: &PgPool) -> crate::PulsarrResult<bool> {
    match T::delete::<T>(id).fetch_optional(pool).await {
        Ok(_) => Ok(Json(true)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

pub async fn get_by_id<T: Model>(id: i32, pool: &PgPool) -> crate::PulsarrResult<T> {
    match T::get_by_id::<T>(id).fetch_one(pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

pub async fn get_all<T: Model>(pool: &PgPool) -> crate::PulsarrResult<Vec<T>> {
    match T::get_all::<T>().fetch_all(pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}