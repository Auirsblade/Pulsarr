use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::{FromRow, PgPool};
use sqlx::types::Decimal;
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct RatingSystemParameter {
    rating_system_parameter_id: i32,
    rating_system_id: i32,
    parameter_rating_max: Decimal,
    name: String,
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system_parameter]
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

/// # Get a rating system parameter by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system_parameter(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystemParameter> {
    let rating_system = sqlx::query_as::<_, RatingSystemParameter>(
        "select * from rating_system_parameter where rating_system_parameter_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(rating_system))
}
