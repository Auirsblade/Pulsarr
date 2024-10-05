use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec, JsonSchema};
use sqlx::{FromRow, PgPool};
use crate::data::models::Model;

use crate::error::PulsarrError;
use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct PulsarrGroup {
    pulsarr_group_id: i32,
    rating_system_id: i32,
    name: String,
    privacy_type: String,
}

pub(crate) const PRIVACY_TYPE: [&str; 2] = ["Public", "Private"];

impl Model for PulsarrGroup {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO pulsarr_group (rating_system_id, name, privacy_type)\
        VALUES ($1, $2, $3)",
        )
            .bind(self.rating_system_id)
            .bind(self.name)
            .bind(self.privacy_type)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn update(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO pulsarr_group (pulsarr_group_id, rating_system_id, name, privacy_type)\
        VALUES ($1, $2, $3, $4)",
        )
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.name)
            .bind(self.privacy_type)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(id: String, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM pulsarr_group WHERE pulsarr_group_id = $1")
            .bind(id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn get_by_id(id: i32, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("SELECT FROM pulsarr_group WHERE pulsarr_group_id = $1")
            .bind(id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn get_all(pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("SELECT FROM pulsarr_group")
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }
}