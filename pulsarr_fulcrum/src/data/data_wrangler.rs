use sqlx::PgPool;
use crate::data::models::Model;
use crate::error::PulsarrError;

pub async fn add<T: Model>(object: T, pool: &PgPool) -> Result<T, PulsarrError> {
    match object.add::<T>().fetch_one(pool).await {
        Ok(result) => Ok(result),
        Err(error) => Err(PulsarrError::validation_error(error))
    }
}

pub async fn update<T: Model>(object: T, pool: &PgPool) -> Result<T, PulsarrError> {
    match object.update::<T>().fetch_one(pool).await {
        Ok(result) => Ok(result),
        Err(error) => Err(PulsarrError::validation_error(error))
    }
}

pub async fn delete<T: Model>(id: i32, pool: &PgPool) -> Result<bool, PulsarrError> {
    match T::delete::<T>(id).fetch_optional(pool).await {
        Ok(_) => Ok(true),
        Err(error) => Err(PulsarrError::validation_error(error))
    }
}

pub async fn get_by_id<T: Model>(id: i32, pool: &PgPool) -> Result<T, PulsarrError> {
    match T::get_by_id::<T>(id).fetch_one(pool).await {
        Ok(result) => Ok(result),
        Err(error) => Err(PulsarrError::validation_error(error))
    }
}

pub async fn get_all<T: Model>(pool: &PgPool, take_size: Option<i32>, offset: Option<i32>) -> Result<Vec<T>, PulsarrError> {
    match T::get_all::<T>(take_size, offset).fetch_all(pool).await {
        Ok(result) => Ok(result),
        Err(error) => Err(PulsarrError::validation_error(error))
    }
}