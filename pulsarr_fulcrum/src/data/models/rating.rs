use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::{FromRow, PgPool, Postgres, query_as, types::Decimal};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct Rating {
    rating_id: i32,
    pulsarr_user_id: i32,
    pulsarr_group_id: i32,
    rating_system_id: i32,
    comments: String,
    rating_value: Decimal,
}

impl Model for Rating {
    fn add<Rating: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, Rating, PgArguments> {
        query_as(
            "INSERT INTO rating (pulsarr_user_id, pulsarr_group_id, rating_system_id, comments, rating_value)\
            VALUES ($1, $2, $3, $4, $5)\
            RETURNING *",
        )
            .bind(self.pulsarr_user_id)
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.comments)
            .bind(self.rating_value)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating\
            SET pulsarr_user_id = $2, pulsarr_group_id = $3, rating_system_id = $4, comments = $5, rating_value = $6\
            WHERE rating_id = $1\
            RETURNING *"
        )
            .bind(self.rating_id)
            .bind(self.pulsarr_user_id)
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.comments)
            .bind(self.rating_value)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating WHERE rating_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating WHERE rating_id = $1").bind(id)
    }

    fn get_all<T: Model>() -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating")
    }
}