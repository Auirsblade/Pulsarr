use crate::data::models::Model;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{query_as, FromRow, Postgres};

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingReaction {
    pub rating_reaction_id: i32,
    pub rating_id: i32,
    pub pulsarr_user_id: i32,
    pub emoji: String,
    pub created_at: NaiveDateTime,
}

impl Model for RatingReaction {
    fn add<RatingReaction: for<'r> sqlx::FromRow<'r, PgRow>>(
        self,
    ) -> QueryAs<'static, Postgres, RatingReaction, PgArguments> {
        query_as(
            "INSERT INTO rating_reaction (rating_id, pulsarr_user_id, emoji) \
            VALUES ($1, $2, $3) \
            RETURNING *",
        )
            .bind(self.rating_id)
            .bind(self.pulsarr_user_id)
            .bind(self.emoji)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        // Reactions are immutable — update is not meaningful. Provide a no-op
        // self-returning query so the Model trait is satisfied for callers
        // that only use add/delete/get.
        query_as("SELECT * FROM rating_reaction WHERE rating_reaction_id = $1")
            .bind(self.rating_reaction_id)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating_reaction WHERE rating_reaction_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_reaction WHERE rating_reaction_id = $1").bind(id)
    }

    fn get_all<T: Model>(_take_size: Option<i32>, _offset: Option<i32>) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_reaction ORDER BY created_at DESC")
    }
}
