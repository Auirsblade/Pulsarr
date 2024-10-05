use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket_okapi::{
    okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec, settings::OpenApiSettings,
    JsonSchema,
};
use sqlx;
use sqlx::types::Decimal;
use sqlx::{FromRow, PgPool};
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: String,
    rating_max: Decimal,
    name: String,
}

const RATING_TYPE: [&str; 3] = ["Absolute", "Cumulative", "Average"];

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

/// Api Logic
pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system, get_rating_types]
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystem> {
    let rating_system = sqlx::query_as::<_, RatingSystem>(
        "select * from rating_system where rating_system_id = $1",
    )
    .bind(&id)
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Ok(Json(rating_system))
}

/// # Get the master rating types
#[openapi(tag = "Rating System")]
#[get("/ratingTypes")]
async fn get_rating_types() -> crate::PulsarrResult<Vec<String>> {
    let mut rating_types = vec![];

    for typ in RATING_TYPE {
        rating_types.push(typ.to_owned());
    }

    Ok(Json(rating_types))
}
