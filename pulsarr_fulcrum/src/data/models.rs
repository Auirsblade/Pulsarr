use sqlx::{FromRow, Postgres};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;

pub mod pulsarr_user;
pub mod pulsarr_group;
pub mod rating_system;
pub mod rating_system_parameter;
pub mod rating;
pub mod rating_detail;


pub trait Model: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin {
    fn add<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments>;
    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments>;
    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments>;
    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments>;
    fn get_all<T: Model>() -> QueryAs<'static, Postgres, T, PgArguments>;
}
