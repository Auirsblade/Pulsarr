use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket_okapi::JsonSchema;
use sqlx;
use sqlx::types::Decimal;
use sqlx::{FromRow, PgPool};
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: String,
    rating_max: Decimal,
    name: String,
}

pub(crate) const RATING_TYPE: [&str; 3] = ["Absolute", "Cumulative", "Average"];

impl Model for RatingSystem {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating_system (master_rating_type, rating_max, name)\
            VALUES ($1,$2,$3)",
        )
            .bind(self.master_rating_type)
            .bind(self.rating_max)
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
            "INSERT INTO rating_system (rating_system_id, master_rating_type, rating_max, name)\
            VALUES ($1,$2,$3, $4)",
        )
            .bind(self.rating_system_id)
            .bind(self.master_rating_type)
            .bind(self.rating_max)
            .bind(self.name)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM rating_system WHERE rating_id = $1")
            .bind(self.rating_system_id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }
}