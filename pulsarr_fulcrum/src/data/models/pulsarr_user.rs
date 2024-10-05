use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx;
use sqlx::{FromRow, PgPool, Postgres};
use sqlx::postgres::PgArguments;
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub struct PulsarrUser {
    pulsarr_user_id: i32,
    name: String,
}

impl Model for PulsarrUser {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO pulsarr_user (name)\
            VALUES ($1)",
        )
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
            "INSERT INTO pulsarr_user (pulsarr_user_id, name)\
            VALUES ($1, $2)",
        )
            .bind(self.pulsarr_user_id)
            .bind(self.name)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(id: i32, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    fn get_by_id(id: &i32) -> QueryAs<Postgres, Self, PgArguments>
    where
        Self: Sized
    {
        todo!()
    }

    async fn get_all(pool: &PgPool) -> (bool, Option<String>) {
        todo!()
    }
}