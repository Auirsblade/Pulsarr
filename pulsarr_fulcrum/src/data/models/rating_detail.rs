use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::FromRow;
use sqlx::types::Decimal;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingDetail {
    rating_detail_id: i32,
    rating_id: i32,
    rating_system_parameter_id: i32,
    rating_value: Decimal,
}