use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::{FromRow, types::Decimal};

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct Rating {
    rating_id: i32,
    pulsarr_user_id: i32,
    pulsarr_group_id: i32,
    rating_system_id: i32,
    comments: String,
    rating_value: Decimal,
}
