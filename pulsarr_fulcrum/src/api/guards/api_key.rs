use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{Object, Parameter, ParameterValue};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use sqlx::{query_as, PgPool, Postgres};
use uuid::Uuid;
use crate::data::models::user_session::UserSession;
use crate::PostgresState;

pub struct ApiKey(pub i32);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid API key string.
        let state = request.rocket().state::<PostgresState>().unwrap();
        let pool = &state.pool;

        match request.headers().get_one("pulsarr-api-key") {
            None => Outcome::Error((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) => {
                match get_signed_in_user_id(key, pool).await {
                    Ok(user_id) => Outcome::Success(ApiKey(user_id)),
                    Err(e) => Outcome::Error((Status::Unauthorized, e))
                }
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> OpenApiFromRequest<'r> for ApiKey {
    fn from_request_input(gen: &mut OpenApiGenerator, name: String, required: bool) -> rocket_okapi::Result<RequestHeaderInput> {
        let schema = gen.json_schema::<String>();
        Ok(RequestHeaderInput::Parameter(Parameter {
            name: "pulsarr-api-key".to_owned(),
            location: "header".to_owned(),
            description: Option::from("Pulsarr Api Key (user session key)".to_owned()),
            required,
            deprecated: false,
            allow_empty_value: false,
            value: ParameterValue::Schema {
                style: None,
                explode: None,
                allow_reserved: false,
                schema,
                example: None,
                examples: None,
            },
            extensions: Object::default(),
        }))
    }
}

async fn get_signed_in_user_id(key: &str, pool: &PgPool) -> Result<i32, ApiKeyError> {
    let key_uid = match Uuid::parse_str(key) {
        Ok(uuid) => uuid,
        Err(_) => return Err(ApiKeyError::Invalid)
    };

    let user_session_result = query_as::<Postgres, UserSession>("SELECT * FROM user_session WHERE user_session_uid = $1")
        .bind(key_uid).fetch_one(pool).await;

    match user_session_result {
        Ok(user_session) => {
            let now = chrono::Utc::now().naive_utc();
            let expires_at = user_session.created_at + chrono::Duration::days(7);
            if now > expires_at {
                let _ = sqlx::query("DELETE FROM user_session WHERE user_session_uid = $1")
                    .bind(key_uid)
                    .execute(pool)
                    .await;
                return Err(ApiKeyError::Invalid);
            }
            Ok(user_session.pulsarr_user_id)
        },
        Err(_) => Err(ApiKeyError::Invalid)
    }
}
