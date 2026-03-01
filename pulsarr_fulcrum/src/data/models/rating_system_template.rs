use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::types::Decimal;
use sqlx::{FromRow, Postgres, query_as};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystemTemplate {
    pub template_id: i32,
    pub name: String,
    pub master_rating_type: String,
    pub rating_max: Decimal,
}

impl Model for RatingSystemTemplate {
    fn add<RatingSystemTemplate: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, RatingSystemTemplate, PgArguments> {
        query_as(
            "INSERT INTO rating_system_template (name, master_rating_type, rating_max)\
            VALUES ($1, $2, $3)\
            RETURNING *",
        )
            .bind(self.name)
            .bind(self.master_rating_type)
            .bind(self.rating_max)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating_system_template \
            SET name = $2, master_rating_type = $3, rating_max = $4 \
            WHERE template_id = $1 \
            RETURNING *"
        )
            .bind(self.template_id)
            .bind(self.name)
            .bind(self.master_rating_type)
            .bind(self.rating_max)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating_system_template WHERE template_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_template WHERE template_id = $1").bind(id)
    }

    fn get_all<T: Model>(_take_size: Option<i32>, _offset: Option<i32>) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_template")
    }
}
