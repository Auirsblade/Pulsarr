use rocket::{get, State};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_okapi::{JsonSchema, openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;
use sqlx::{FromRow, PgPool, types::Decimal};
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct Rating {
    rating_id: i32,
    pulsarr_user_id: i32,
    pulsarr_group_id: i32,
    rating_system_id: i32,
    comments: String,
    rating_value: Decimal,
}

impl Model for Rating {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query(
            "INSERT INTO rating (pulsarr_user_id, pulsarr_group_id, rating_system_id, comments, rating_value)\
            VALUES ($1, $2, $3, $4, $5)",
        )
            .bind(self.pulsarr_user_id)
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.comments)
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
            "INSERT INTO rating (rating_id, pulsarr_user_id, pulsarr_group_id, rating_system_id, comments, rating_value)\
            VALUES ($1, $2, $3, $4, $5, $6)",
        )
            .bind(self.rating_id)
            .bind(self.pulsarr_user_id)
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.comments)
            .bind(self.rating_value)
            .execute(pool)
            .await;

        match result {
            Ok(_) => (true, None),
            Err(err) => (false, Some(err.to_string()))
        }
    }

    async fn delete(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM rating WHERE rating_id = $1")
            .bind(self.rating_id)
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
    openapi_get_routes_spec![settings: get_rating]
}

/// # Get a rating by id
#[openapi(tag = "Rating")]
#[get("/<id>")]
async fn get_rating(state: &State<PostgresState>, id: i32) -> crate::PulsarrResult<Rating> {
    let group = sqlx::query_as::<_, Rating>("select * from rating where rating_id = $1")
        .bind(&id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Ok(Json(group))
}
