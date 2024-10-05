use rocket::serde::json::Json;
use sqlx::PgPool;
use crate::data::models::Model;
use crate::error::PulsarrError;

pub async fn add<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<T> {
    match object.add::<T>().fetch_one(pool).await
    {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

// pub async fn update<T: Model>(object: T, pool: &PgPool) -> crate::PulsarrResult<bool>{
//     match object.update(pool).await {
//         (true, _) => Ok(Json(true)),
//         (false, error_message) => Err(PulsarrError {
//             err: "validation error".to_owned(),
//             msg: error_message,
//             http_status_code: 400,
//         })
//     }
// }

// pub async fn delete<T: Model>(id: i32, pool: &PgPool) -> crate::PulsarrResult<bool> {
//     match T::delete(id, pool).await { 
//         (true, _) => Ok(Json(true)),
//         (false, error_message) => Err(PulsarrError {
//             err: "validation error".to_owned(),
//             msg: error_message,
//             http_status_code: 400,
//         })
//     } 
// }

pub async fn get_by_id<T: Model>(id: i32, pool: &PgPool) -> crate::PulsarrResult<T> {
    match T::get_by_id(id).fetch_one(pool).await {
        Ok(result) => Ok(Json(result)),
        Err(error) => Err(PulsarrError {
            err: "validation error".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        })
    }
}

// pub async fn get_all<T: Model>(pool: &PgPool) -> crate::PulsarrResult<[PulsarrGroup]> {
//     match T::get_all(pool).await {
//         (true, _) => Ok(Json(true)),
//         (false, error_message) => Err(PulsarrError {
//             err: "validation error".to_owned(),
//             msg: error_message,
//             http_status_code: 400,
//         })
//     }
// }