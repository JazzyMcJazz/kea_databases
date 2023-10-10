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

use crate::{routes, middleware};

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
            .wrap(middleware::Authentication)
            .configure(init)
    });

    server = server.bind("0.0.0.0:3000")?;
    server.run().await?;

    Ok(())
}

fn init(cfg: &mut web::ServiceConfig) {
    
    cfg.route("/",    web::get().to(routes::index::index));
    cfg.route("/404", web::get().to(routes::index::not_found));

    // Relania (PUBLIC)
    cfg.route("/relania/login",    web::get().to(routes::relania::auth::login_page));
    cfg.route("/relania/login",    web::post().to(routes::relania::auth::login));
    cfg.route("/relania/register", web::get().to(routes::relania::auth::register_page));
    cfg.route("/relania/register", web::post().to(routes::relania::auth::register));

    // Relania (PROTECTED)
    cfg.service(web::scope("/relania")
        .wrap(middleware::Authorization)
        .route("", web::get().to(routes::relania::index::index))
        .route("/logout", web::get().to(routes::relania::auth::logout))
        .route("/c", web::get().to(routes::relania::character::create_character_view))
        .route("/c", web::post().to(routes::relania::character::create_character))
        .route("/c/{id}", web::get().to(routes::relania::character::character_detail))
        .route("/c/{id}", web::delete().to(routes::relania::character::character_detail))
        .route("/c/{id}/i", web::post().to(routes::relania::inventory::loot_list))
        .route("/c/{id}/i/{item_piece_id}", web::put().to(routes::relania::inventory::loot_details))
        .route("/c/{id}/i/{item_piece_id}", web::patch().to(routes::relania::inventory::loot_details))
        .route("/c/{id}/i/{item_piece_id}", web::delete().to(routes::relania::inventory::loot_details))
    );
    
    // Default 404
    cfg.default_service(web::route().to(|| async { 
        actix_web::HttpResponse::Found()
            .append_header(("Location", "/404"))
            .finish()
     }));
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}