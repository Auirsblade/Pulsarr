use crate::data::data_wrangler;
use crate::error::PulsarrError;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};
use sqlx;
use sqlx::{FromRow, PgPool};
use crate::data::models::Model;
use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct PulsarrUser {
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
}