mod error;
mod api;
mod data;
mod musicbrainz_client;

use crate::api::{group, rating, rating_system, user, auth, musicbrainz};
use crate::musicbrainz_client::MusicBrainzClient;
use crate::api::musicbrainz::MusicBrainzState;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, http::Method};
use rocket_okapi::{mount_endpoints_and_merged_docs, swagger_ui::*};
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::postgres::PgPool;
use sqlx::Error;
use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

pub type PulsarrResult<T> = Result<Json<T>, error::PulsarrError>;

struct PostgresState {
    pool: PgPool
}

#[rocket::main]
async fn main() {
    // setup db connection and run any necessary migrations
    println!("app starting");
    dotenv().ok();
    let postgres_pool = get_db_pool().await.unwrap();
    
    println!("running migrations");
    sqlx::migrate!("db/migrations").run(&postgres_pool).await.unwrap();
    println!("migrations complete");
    
    let launch_result = create_server().manage(PostgresState { pool: postgres_pool }).launch().await;

    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err)
    }
}

async fn get_db_pool() -> Result<PgPool, Error> {
    let db_url = "postgresql://".to_owned() + dotenv!("POSTGRES_URL");
    println!("connecting to db: {db_url}");
    let pool = PgPool::connect(&db_url).await;
    pool
}

fn create_server() -> Rocket<Build> {

    let port = dotenv!("RUST_PORT").parse::<u16>().unwrap();

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));
    
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect()
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
        "/rating" => rating::get_routes_and_docs(&openapi_settings),
        "/musicbrainz" => musicbrainz::get_routes_and_docs(&openapi_settings),
    }
    
    building_rocket
}
