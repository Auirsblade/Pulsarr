use sqlx::PgPool;
use crate::PulsarrResult;

pub mod pulsarr_user;
pub mod pulsarr_group;
pub mod rating_system;
pub mod rating_system_parameter;
pub mod rating;
pub mod rating_detail;


pub  trait Model {
    async fn add(self, pool: &PgPool) -> (bool, Option<String>);
    async fn update(self, pool: &PgPool) -> (bool, Option<String>);
}
