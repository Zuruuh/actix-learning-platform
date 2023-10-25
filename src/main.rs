use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("No database url provided");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(greet)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind((
        "127.0.0.1",
        std::env::var("PORT")
            .map(|port| port.parse::<u16>().expect("Invalid port provided!"))
            .unwrap_or(3000),
    ))?
    .run()
    .await
}
