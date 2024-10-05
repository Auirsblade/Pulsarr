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
struct RatingDetail {
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

    async fn delete(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM rating_detail WHERE rating_id = $1")
            .bind(self.rating_detail_id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }
}

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_detail]
}

/// # Get a rating detail by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating_detail(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingDetail> {
    let group = sqlx::query_as::<_, RatingDetail>(
        "select * from rating_detail where rating_detail_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(group))
}
