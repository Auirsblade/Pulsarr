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

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        todo!()
    }

    // async fn update<Rating>(self) -> QueryAs<'static, Postgres, Rating, PgArguments> {
    //     query_as(
    //         "INSERT INTO rating (rating_id, pulsarr_user_id, pulsarr_group_id, rating_system_id, comments, rating_value)\
    //         VALUES ($1, $2, $3, $4, $5, $6)",
    //     )
    //         .bind(self.rating_id)
    //         .bind(self.pulsarr_user_id)
    //         .bind(self.pulsarr_group_id)
    //         .bind(self.rating_system_id)
    //         .bind(self.comments)
    //         .bind(self.rating_value)
    // }
    //
    // async fn delete<Rating>(id: i32) -> QueryAs<'static, Postgres, Rating, PgArguments> {
    //     query_as("DELETE FROM rating WHERE rating_id = $1")
    //         .bind(id)
    // }
}