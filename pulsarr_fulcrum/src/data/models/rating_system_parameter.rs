use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::{FromRow, PgPool, Postgres, query_as};
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
            "INSERT INTO rating_system_parameter (rating_system_id, parameter_rating_max, parameter_rating_max, name)\
            VALUES ($1, $2, $3, $4)",
        )
            .bind(self.rating_system_id)
            .bind(self.parameter_rating_max)
            .bind(self.parameter_rating_max)
            .bind(self.name)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        todo!()
    }

    // async fn update<RatingSystemParameter>(self) ->  QueryAs<'static, Postgres, RatingSystemParameter, PgArguments> {
    //     query_as(
    //         "INSERT INTO rating_system_parameter (rating_system_parameter_id, rating_system_id, parameter_rating_max, parameter_rating_max, name)\
    //         VALUES ($1, $2, $3, $4, $5)",
    //     )
    //         .bind(self.rating_system_parameter_id)
    //         .bind(self.rating_system_id)
    //         .bind(self.parameter_rating_max)
    //         .bind(self.parameter_rating_max)
    //         .bind(self.name)
    // }
    //
    // async fn delete<RatingSystemParameter>(id: i32) ->  QueryAs<'static, Postgres, RatingSystemParameter, PgArguments> {
    //     query_as("DELETE FROM rating_system_parameter WHERE rating_id = $1")
    //         .bind(id)
    // }
    
    // async fn get_all(pool: &PgPool) -> (bool, Option<String>) {
    //     todo!()
    // }
}