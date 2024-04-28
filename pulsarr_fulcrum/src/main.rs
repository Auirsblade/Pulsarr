mod error;
mod models {
    pub mod rating_system;
}

use rocket::form::FromForm;
use rocket::{Build, Rocket};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{mount_endpoints_and_merged_docs, swagger_ui::*};
use sqlx::postgres::PgPool;
use std::env;
use crate::models::rating_system;

pub type Result<T> = std::result::Result<rocket::serde::json::Json<T>, error::Error>;

#[rocket::main]
async fn main() {
    // setup db connection and run any necessary migrations
    println!("app starting");
    let db_url = env!("DATABASE_URL");
    println!("connecting to db: {db_url}");
    let pool = PgPool::connect(&db_url).await.unwrap();
    println!("running migrations");
    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();
    println!("migrations complete");
    
    let launch_result = create_server().launch().await;

    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    }
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
        "/rating-system" => rating_system::get_routes_and_docs(&openapi_settings)
    }
    
    building_rocket
}
