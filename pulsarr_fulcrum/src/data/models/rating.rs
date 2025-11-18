use crate::data::models::Model;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{query_as, types::Decimal, FromRow, Postgres};

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct Rating {
    pub rating_id: i32,
    pub pulsarr_user_id: i32,
    pub pulsarr_group_id: i32,
    pub rating_system_id: i32,
    pub comments: String,
    pub rating_value: Decimal,
    pub media_type: String,
    pub media_title: String,
    pub musicbrainz_id: String,
    pub artist_name: String,
    pub rating_date: NaiveDateTime,
    pub release_date: NaiveDateTime,
}

impl Model for Rating {
    fn add<Rating: for<'r> sqlx::FromRow<'r, PgRow>>(
        self,
    ) -> QueryAs<'static, Postgres, Rating, PgArguments> {
        query_as(
            "INSERT INTO rating (pulsarr_user_id, pulsarr_group_id, rating_system_id, comments, rating_value, media_type, media_title, musicbrainz_id, artist_name, rating_date, release_date)\
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)\
            RETURNING *",
        )
            .bind(self.pulsarr_user_id)
            .bind(self.pulsarr_group_id)
            .bind(self.rating_system_id)
            .bind(self.comments)
            .bind(self.rating_value)
            .bind(self.media_type)
            .bind(self.media_title)
            .bind(self.musicbrainz_id)
            .bind(self.artist_name)
            .bind(self.rating_date)
            .bind(self.release_date)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating \
            SET pulsarr_user_id = $2, pulsarr_group_id = $3, rating_system_id = $4, comments = $5, rating_value = $6 \
            WHERE rating_id = $1 \
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

    fn get_all<T: Model>(take_size: Option<i32>) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating LIMIT $1").bind(take_size)
    }
}
