use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::types::Decimal;
use sqlx::{FromRow, PgPool};
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingDetail {
    rating_detail_id: i32,
    rating_id: i32,
    rating_system_parameter_id: i32,
    rating_value: Decimal,
}

impl Model for RatingDetail {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating_detail (rating_id, rating_system_parameter_id, rating_value)\
            VALUES ($1, $2, $3)",
        )
            .bind(self.rating_id)
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_value)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn update(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating_detail (rating_detail_id, rating_id, rating_system_parameter_id, rating_value)\
            VALUES ($1, $2, $3, $4)",
        )
            .bind(self.rating_detail_id)
            .bind(self.rating_id)
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_value)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(id: i32, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM rating_detail WHERE rating_id = $1")
            .bind(id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }
}