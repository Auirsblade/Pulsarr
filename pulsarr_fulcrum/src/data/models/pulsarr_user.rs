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
struct PulsarrUser {
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

    async fn delete(self, pool: &PgPool) -> (bool, Option<String>) {
        let result = sqlx::query("DELETE FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(self.pulsarr_user_id)
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
    openapi_get_routes_spec![settings: get_pulsarr_user, add_user]
}

/// # Get a user by id
#[openapi(tag = "User")]
#[get("/<id>")]
async fn get_pulsarr_user(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<PulsarrUser> {
    let rating_system =
        sqlx::query_as::<_, PulsarrUser>("select * from pulsarr_user where pulsarr_user_id = $1")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .unwrap();

    Ok(Json(rating_system))
}

/// # Add a user
#[openapi(tag = "User")]
#[post("/", format = "application/json", data = "<user>")]
async fn add_user(
    state: &State<PostgresState>,
    user: Json<PulsarrUser>,
) -> crate::PulsarrResult<bool> {
    data_wrangler::add(user.into_inner(), &state.pool).await
}
