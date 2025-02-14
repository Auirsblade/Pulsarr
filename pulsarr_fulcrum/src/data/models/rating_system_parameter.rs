use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::{FromRow, PgPool, Postgres, query_as, query};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::types::Decimal;
use crate::data::models::Model;

use crate::PostgresState;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystemParameter {
    rating_system_parameter_id: i32,
    rating_system_id: i32,
    parameter_rating_max: Decimal,
    name: String,
}

impl Model for RatingSystemParameter {
    fn add<RatingSystemParameter: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, RatingSystemParameter, PgArguments> {
        query_as(
            "INSERT INTO rating_system_parameter (rating_system_id, parameter_rating_max, name)\
            VALUES ($1, $2, $3)\
            RETURNING *",
        )
            .bind(self.rating_system_id)
            .bind(self.parameter_rating_max)
            .bind(self.name)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating_system_parameter\
            SET rating_system_id = $2, parameter_rating_max = $3, name = $4\
            WHERE rating_system_parameter_id = $1\
            RETURNING *"
        )
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_system_id)
            .bind(self.parameter_rating_max)
            .bind(self.name)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating_system_parameter WHERE rating_system_parameter_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_parameter WHERE rating_system_parameter_id = $1").bind(id)
    }

    fn get_all<T: Model>() -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_parameter")
    }
}