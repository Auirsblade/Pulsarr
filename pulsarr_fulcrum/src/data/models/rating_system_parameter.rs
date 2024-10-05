use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::{JsonSchema};
use sqlx::types::Decimal;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystemParameter {
    rating_system_parameter_id: i32,
    rating_system_id: i32,
    parameter_rating_max: Decimal,
    name: String,
}
