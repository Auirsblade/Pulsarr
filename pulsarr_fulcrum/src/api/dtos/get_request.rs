use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct GetRequest {
    pub take_size: Option<i32>
}