use crate::data::models::Model;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{query_as, FromRow, Postgres};
use sqlx::types::chrono::NaiveDateTime;

#[derive(FromRow)]
pub struct PulsarrGroup {
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub name: String,
    pub privacy_type: String,
    pub creation_date: NaiveDateTime,
    pub created_by_user_id: Option<i32>
}

pub const PUBLIC_PRIVACY_TYPE: &str = "Public";
pub const PRIVATE_PRIVACY_TYPE: &str = "Private";
pub const PERSONAL_PRIVACY_TYPE: &str = "Personal";
pub const PRIVACY_TYPE: [&str; 3] = [PUBLIC_PRIVACY_TYPE, PRIVATE_PRIVACY_TYPE, PERSONAL_PRIVACY_TYPE];

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
    
    fn get_all<PulsarrGroup: for<'r> sqlx::FromRow<'r, PgRow>>(take_size: Option<i32>) -> QueryAs<'static, Postgres, PulsarrGroup, PgArguments> {
        match take_size {
            Some(size) => query_as("SELECT * FROM pulsarr_group LIMIT $1").bind(size),
            None => query_as("SELECT * FROM pulsarr_group")
        }
    }
}