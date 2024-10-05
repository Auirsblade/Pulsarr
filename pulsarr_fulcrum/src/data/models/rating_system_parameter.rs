use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::{FromRow, PgPool};
use sqlx::types::Decimal;
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystemParameter {
    rating_system_parameter_id: i32,
    rating_system_id: i32,
    parameter_rating_max: Decimal,
    name: String,
}

impl Model for RatingSystemParameter {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating_system_parameter (rating_system_id, parameter_rating_max, parameter_rating_max, name)\
            VALUES ($1, $2, $3, $4)",
        )
            .bind(self.rating_system_id)
            .bind(self.parameter_rating_max)
            .bind(self.parameter_rating_max)
            .bind(self.name)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn update(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating_system_parameter (rating_system_parameter_id, rating_system_id, parameter_rating_max, parameter_rating_max, name)\
            VALUES ($1, $2, $3, $4, $5)",
        )
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_system_id)
            .bind(self.parameter_rating_max)
            .bind(self.parameter_rating_max)
            .bind(self.name)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM rating_system_parameter WHERE rating_id = $1")
            .bind(self.rating_system_parameter_id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }
}