mod error;
mod models {
    pub mod pulsarr_user;
    pub mod pulsarr_group;
    pub mod rating_system;
    pub mod rating_system_parameter;
    pub mod rating;
    pub mod rating_detail;
}

use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, swagger_ui::*};
use sqlx::postgres::PgPool;
use std::{env};
use rocket::serde::json::Json;
use sqlx::{Error};
use crate::models::pulsarr_user;
use crate::models::pulsarr_group;
use crate::models::rating_system;
use crate::models::rating_system_parameter;
use crate::models::rating;
use crate::models::rating_detail;

pub type PulsarrResult<T> = Result<Json<T>, error::PulsarrError>;

struct PostgresState {
    pool: PgPool
}

#[rocket::main]
async fn main() {
    // setup db connection and run any necessary migrations
    println!("app starting");
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
    let db_url = env!("DATABASE_URL");
    println!("connecting to db: {db_url}");
    let pool = PgPool::connect(&db_url).await;
    pool
}

fn create_server() -> Rocket<Build> {

    let figment = rocket::Config::figment()
        .merge(("port", 3003))
        .merge(("address", "0.0.0.0"));

    let mut building_rocket = rocket::custom(figment)
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        );
    
    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/user" => pulsarr_user::get_routes_and_docs(&openapi_settings),
        "/group" => pulsarr_group::get_routes_and_docs(&openapi_settings),
        "/rating-system" => rating_system::get_routes_and_docs(&openapi_settings),
        "/rating-system_parameter" => rating_system_parameter::get_routes_and_docs(&openapi_settings),
        "/rating" => rating::get_routes_and_docs(&openapi_settings),
        "/rating-detail" => rating_detail::get_routes_and_docs(&openapi_settings),
    }
    
    building_rocket
}
