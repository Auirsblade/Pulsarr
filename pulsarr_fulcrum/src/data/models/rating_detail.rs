use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::types::Decimal;
use sqlx::{FromRow, PgPool, Postgres, query_as};
use sqlx::postgres::{PgAdvisoryLockKey, PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingDetail {
    rating_detail_id: i32,
    rating_id: i32,
    rating_system_parameter_id: i32,
    rating_value: Decimal,
}

impl Model for RatingDetail {
    fn add<RatingDetail: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, RatingDetail, PgArguments> {
        query_as(
            "INSERT INTO rating_detail (rating_id, rating_system_parameter_id, rating_value)\
            VALUES ($1, $2, $3)\
            RETURNING *",
        )
            .bind(self.rating_id)
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_value)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating_detail\
            SET rating_id = $2, rating_system_parameter_id = $3, rating_value = $4\
            WHERE rating_detail_id = $1\
            RETURNING *"
        )
            .bind(self.rating_detail_id)
            .bind(self.rating_id)
            .bind(self.rating_system_parameter_id)
            .bind(self.rating_value)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating_detail WHERE rating_detail_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_detail WHERE rating_detail_id = $1").bind(id)
    }

    fn get_all<T: Model>() -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_detail")
    }
}