use rocket::futures::TryStreamExt;
use crate::data::models::Model;
use scrypt::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Params, Scrypt};
use sqlx;
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use sqlx::{query_as, FromRow, Postgres, QueryBuilder};

#[derive(FromRow)]
pub struct PulsarrUser {
    pub pulsarr_user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Model for PulsarrUser {
    fn add<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        let password_hash = hash_password(self.password);
        sqlx::query_as(
            "INSERT INTO pulsarr_user (name, email, password)\
            VALUES ($1, $2, $3)\
            RETURNING *"
        )
            .bind(self.name)
            .bind(self.email)
            .bind(password_hash)
    }

    fn update<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        sqlx::query_as(
            "UPDATE pulsarr_user \
             SET name = $2, email = $3 \
            WHERE pulsarr_user_id = $1 \
            RETURNING *"
        )
            .bind(self.pulsarr_user_id)
            .bind(self.name)
            .bind(self.email)
    }

    fn delete<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("DELETE FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(id)
    }

    fn get_by_id<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(id: i32) -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("SELECT * FROM pulsarr_user WHERE pulsarr_user_id = $1")
            .bind(id)
    }

    fn get_all<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>() -> QueryAs<'static, Postgres, PulsarrUser, PgArguments> {
        query_as("SELECT * FROM pulsarr_user")
    }
}

fn hash_password(password: String) -> String {
    Scrypt.hash_password_customized(
        password.as_ref(), 
        None, None, 
        Params::new(8, 8, 1, 32).expect("Invalid password hashing parameters"), 
        &SaltString::generate(&mut OsRng)
    ).expect("Failed to hash password").to_string()
}

pub fn get_by_email<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(email: &str) -> QueryAs<Postgres, PulsarrUser, PgArguments> {
    query_as("SELECT * FROM pulsarr_user WHERE email = $1")
        .bind(email)
}

pub fn get_password_hash<PulsarrUser: for<'r> sqlx::FromRow<'r, PgRow>>(email: &str) -> QueryAs<Postgres, PulsarrUser, PgArguments> {
    query_as("SELECT * FROM pulsarr_user WHERE email = $1")
        .bind(email)
}