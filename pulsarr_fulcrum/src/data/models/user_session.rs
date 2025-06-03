use crate::error::PulsarrError;
use sqlx::types::Uuid;
use sqlx::{query_as, FromRow, PgPool, Postgres};

#[derive(FromRow)]
pub struct UserSession {
    pub user_session_uid: Uuid,
    pub pulsarr_user_id: i32,
}

// impl<'r> FromRow<'r, PgRow> for UserSession {
//     fn from_row(row: &'r PgRow) -> Result<Self, Error> {
//         let user_session_uid: String = row.try_get("session_token")?;
//         let pulsarr_user_id: String = row.try_get("pulsarr_user_id")?;
// 
//         Ok(UserSession { user_session_uid, pulsarr_user_id })
//     }
// }

pub async fn start_session(user_session_id: Uuid, user_id: &i32, pool: &PgPool) -> Result<UserSession, PulsarrError> {
    match query_as::<Postgres, UserSession>("
            INSERT INTO user_session (user_session_uid, pulsarr_user_id) \
                VALUES ($1, $2) \
            RETURNING *"
    )
        .bind(user_session_id)
        .bind(user_id)
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