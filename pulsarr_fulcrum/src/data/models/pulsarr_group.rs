use crate::data::models::Model;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{query_as, FromRow, Postgres};

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub struct PulsarrGroup {
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub name: String,
    pub privacy_type: String,
}

pub const PRIVACY_TYPE: [&str; 2] = ["Public", "Private"];

impl Model for PulsarrGroup {
    fn add<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as(
            "INSERT INTO pulsarr_group (rating_system_id, name, privacy_type)\
                VALUES ($1, $2, $3)\
                RETURNING *"
        )
            .bind(self.rating_system_id)
            .bind(self.name)
            .bind(self.privacy_type)
    }

    fn update<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as(
            "UPDATE pulsarr_group \
                SET rating_system_id = $2, name = $3, privacy_type = $4 \
                WHERE pulsarr_group_id = $1 \
                RETURNING *",
        )
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.name)
            .bind(self.privacy_type)
    }

    fn delete<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as("DELETE FROM pulsarr_group WHERE pulsarr_group_id = $1")
            .bind(id)
    }

    fn get_by_id<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as("SELECT * FROM pulsarr_group WHERE pulsarr_group_id = $1")
            .bind(id)
    }
    
    fn get_all<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>() -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        query_as("SELECT * FROM pulsarr_group")
    }
}