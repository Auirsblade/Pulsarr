mod error;
mod api;
mod data;
mod musicbrainz_client;
mod constants;
mod email;

use crate::api::{group, rating, rating_system, rating_system_template, user, auth, musicbrainz, profile};
use crate::musicbrainz_client::MusicBrainzClient;
use crate::api::musicbrainz::MusicBrainzState;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, http::Method};
use rocket_okapi::{mount_endpoints_and_merged_docs, swagger_ui::*};
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::postgres::PgPool;
use sqlx::Error;
use dotenv::dotenv;

pub type PulsarrResult<T> = Result<Json<T>, error::PulsarrError>;

pub struct PostgresState {
    pub pool: PgPool
}

pub struct EmailState {
    pub sender: email::EmailSender,
}

#[rocket::main]
async fn main() {
    eprintln!("app starting");
    dotenv().ok();
    let postgres_pool = get_db_pool().await.unwrap();

    eprintln!("running migrations");
    sqlx::migrate!("db/migrations").run(&postgres_pool).await.unwrap();
    eprintln!("migrations complete");

    let email_sender = email::EmailSender::from_env();

    let launch_result = create_server()
        .manage(PostgresState { pool: postgres_pool })
        .manage(EmailState { sender: email_sender })
        .launch().await;

    match launch_result {
        Ok(_) => eprintln!("Rocket shut down gracefully."),
        Err(err) => eprintln!("Rocket had an error: {}", err)
    }
}

async fn get_db_pool() -> Result<PgPool, Error> {
    let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db_url = "postgresql://".to_owned() + &postgres_url;
    let pool = PgPool::connect(&db_url).await;
    pool
}

fn create_server() -> Rocket<Build> {

    let figment = rocket::Config::figment()
        .merge(("port", 4004u16))
        .merge(("address", "0.0.0.0"));

    let allowed_origins_str = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3003".to_string());
    let origins: Vec<&str> = allowed_origins_str.split(',').map(|s| s.trim()).collect();
    let allowed_origins = AllowedOrigins::some_exact(&origins);

    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete].into_iter().map(From::from).collect()
        )
        .allow_credentials(true);

    let mb_client = MusicBrainzClient::new();

    let mut building_rocket = rocket::custom(figment)
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(MusicBrainzState { client: mb_client })
        .attach(cors.to_cors().unwrap());

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/auth" => auth::get_routes_and_docs(&openapi_settings),
        "/user" => user::get_routes_and_docs(&openapi_settings),
        "/group" => group::get_routes_and_docs(&openapi_settings),
        "/rating-system" => rating_system::get_routes_and_docs(&openapi_settings),
        "/rating-system-template" => rating_system_template::get_routes_and_docs(&openapi_settings),
        "/rating" => rating::get_routes_and_docs(&openapi_settings),
        "/musicbrainz" => musicbrainz::get_routes_and_docs(&openapi_settings),
        "/profile" => profile::get_routes_and_docs(&openapi_settings),
    }

    building_rocket
}
