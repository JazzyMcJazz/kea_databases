use actix_web::{HttpServer, App, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;
use tera::Tera;
use std::env;
use tracing::log;
use migration::{
    sea_orm::{Database, ConnectOptions, DatabaseConnection},
    Migrator, MigratorTrait
};

use crate::website;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub tera: Tera,
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // establish connection to database and apply migrations
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let conn = Database::connect(opt).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    // Initialize Tera template engine
    let Ok(tera) = Tera::new("templates/**/*") else {
        panic!("Failed to initialize Tera template engine");
    };

    // Build app state
    let state = AppState { conn, tera };

    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // Start the HTTP server
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(init)
    });

    server = server.bind("0.0.0.0:3000")?;
    server.run().await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {

    // Website routes
    cfg.service(website::index::index);
    cfg.service(website::index::not_found);
    
    cfg.default_service(web::route().to(|| async { 
        actix_web::HttpResponse::Found()
            .append_header(("Location", "/404"))
            .finish()
     }));

    // cfg.service(
    //     // This scope is protected by the Authorization middleware
    //     web::scope("")
    //         .wrap(middleware::Authorization)
    //         .service(auth_controller::refresh);
    // );
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}