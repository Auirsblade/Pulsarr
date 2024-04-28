use crate::{schemars, PostgresState};
use rocket::futures::future::err;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, State};
use rocket::async_stream::stream;
use rocket_okapi::{
    okapi::{openapi3::OpenApi, schemars::schema_for},
    openapi, openapi_get_routes_spec,
    settings::OpenApiSettings,
    JsonSchema,
};
use rocket_okapi::okapi::schemars::schema_for_value;
use serde::Serializer;
use sqlx;
use sqlx::{Database, Decode, Error, FromRow, PgPool, Pool, Postgres};
use sqlx::database::HasValueRef;
use sqlx::types::Decimal;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: get_rating_system]
}

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
struct RatingSystem {
    rating_system_id: i32,
    master_rating_type: MasterRatingType,
    rating_max: Decimal,
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, sqlx::Type)]
enum MasterRatingType {
    Absolute,
    Cumulative,
    Average,
}

/// # Get a rating system by id
#[openapi(tag = "Rating System")]
#[get("/<id>")]
async fn get_rating_system(
    state: &State<PostgresState>,
    id: i32,
) -> crate::PulsarrResult<RatingSystem> {
    let rating_system =
        sqlx::query_as::<_, RatingSystem>("select * from rating_system where rating_system_id = $1")
            .bind(&id)
            .fetch_one(&state.pool)
            .await
            .unwrap();

    Ok(Json(rating_system))
}
