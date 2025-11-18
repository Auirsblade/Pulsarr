use crate::error::PulsarrError;
use sqlx::types::Uuid;
use sqlx::{query_as, FromRow, PgPool, Postgres};

#[derive(FromRow)]
pub struct UserSession {
    pub user_session_uid: Uuid,
    pub pulsarr_user_id: i32,
    pub hw_key: String,
}

pub async fn start_session(user_session_id: Uuid, user_id: &i32, hw_key: &str, pool: &PgPool) -> Result<UserSession, PulsarrError> {
    
    match query_as::<Postgres, UserSession>("DELETE FROM user_session WHERE pulsarr_user_id = $1 AND hw_key = $2")
        .bind(user_id)
        .bind(hw_key)
        .fetch_optional(pool).await 
    {
        Ok(_) => (),
        Err(error) => return Err(PulsarrError {
            err: "Error clearing old user session".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        }),   
    }
    
    match query_as::<Postgres, UserSession>("
            INSERT INTO user_session (user_session_uid, pulsarr_user_id, hw_key) \
                VALUES ($1, $2, $3) \
            RETURNING *"
    )
        .bind(user_session_id)
        .bind(user_id)
        .bind(hw_key)
        .fetch_one(pool).await 
    {
        Ok(result) => Ok(result),
        Err(error) => Err(PulsarrError {
            err: "Error creating user session".to_owned(),
            msg: Some(error.to_string()),
            http_status_code: 400,
        }),
    }
}