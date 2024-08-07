use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("TalentDev Backend With Rust")
}

async fn connect_to_db() -> Client {
    dotenv().ok();
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env file");

    let mut client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    client_options.app_name = Some("TalentDevRust".to_string());
    Client::with_options(client_options).expect("Failed to initialize MongoDB client")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = connect_to_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
