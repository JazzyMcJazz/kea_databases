use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use migration::{
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use std::env;
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};
use tera::Tera;
use tracing::log;

use crate::{middleware, repo::{ddbms::DocuRepo, gdbms::GraphRepo}, routes, utils::traits::Terafy};

impl Terafy for Tera {}

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub ddbms_surreal: Surreal<Any>,
    pub gdbms_surreal: Surreal<Any>,
    pub tera: Tera,
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // establish connection to the relational database and apply migrations
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let conn = Database::connect(opt).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    // Establish connection to the SurrealDB document database
    let surreal_url = env::var("SURREAL_URL").expect("SURREAL_URL is not set in .env file");
    let ddbms_surreal = connect(&surreal_url).await.unwrap();
    ddbms_surreal
        .signin(Root {
            username: &env::var("SURREAL_USER").expect("SURREAL_USER is not set in .env file"),
            password: &env::var("SURREAL_PASS").expect("SURREAL_PASS is not set in .env file"),
        })
        .await
        .unwrap();
    ddbms_surreal
        .use_ns("documenia")
        .use_db("documenia")
        .await
        .unwrap();

    // Establish connection to the SurrealDB graph database
    let gdbms_surreal = connect(surreal_url).await.unwrap();
    gdbms_surreal
        .signin(Root {
            username: &env::var("SURREAL_USER").expect("SURREAL_USER is not set in .env file"),
            password: &env::var("SURREAL_PASS").expect("SURREAL_PASS is not set in .env file"),
        })
        .await
        .unwrap();
    gdbms_surreal
        .use_ns("graphia")
        .use_db("graphia")
        .await
        .unwrap();

    // ddbms_surreal.surreal_clear("documenia").await;
    // gdbms_surreal.graphia_clear("graphia").await;
    ddbms_surreal.documenia_init().await;
    gdbms_surreal.graphia_init().await;

    // Initialize Tera template engine
    let Ok(tera) = Tera::new("templates/**/*") else {
        panic!("Failed to initialize Tera template engine");
    };

    // Build app state
    let state = AppState {
        conn,
        ddbms_surreal,
        gdbms_surreal,
        tera,
    };

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
    cfg.route("/", web::get().to(routes::index::index));
    cfg.route("/404", web::get().to(routes::index::not_found));

    // Relania (PUBLIC)
    cfg.route(
        "/relania/login",
        web::get().to(routes::relania::auth::login_page),
    );
    cfg.route(
        "/relania/login",
        web::post().to(routes::relania::auth::login),
    );
    cfg.route(
        "/relania/register",
        web::get().to(routes::relania::auth::register_page),
    );
    cfg.route(
        "/relania/register",
        web::post().to(routes::relania::auth::register),
    );

    // Relania (PROTECTED)
    cfg.service(
        web::scope("/relania")
            .wrap(middleware::Authorization)
            .route("", web::get().to(routes::relania::index::index))
            .route("/logout", web::get().to(routes::relania::auth::logout))
            .route(
                "/c",
                web::get().to(routes::relania::character::create_character_page),
            )
            .route(
                "/c",
                web::post().to(routes::relania::character::create_character),
            )
            .route(
                "/c/{id}",
                web::get().to(routes::relania::character::character_detail),
            )
            .route(
                "/c/{id}",
                web::delete().to(routes::relania::character::character_detail),
            )
            .route(
                "/c/{id}/i",
                web::post().to(routes::relania::inventory::loot_list),
            )
            .route(
                "/c/{id}/i/{item_piece_id}",
                web::put().to(routes::relania::inventory::loot_details),
            )
            .route(
                "/c/{id}/i/{item_piece_id}",
                web::patch().to(routes::relania::inventory::loot_details),
            )
            .route(
                "/c/{id}/i/{item_piece_id}",
                web::delete().to(routes::relania::inventory::loot_details),
            ),
    );

    // Documenia (PUBLIC)
    cfg.route(
        "/documenia/login",
        web::get().to(routes::documenia::auth::login_page),
    );
    cfg.route(
        "/documenia/login",
        web::post().to(routes::documenia::auth::login),
    );
    cfg.route(
        "/documenia/register",
        web::get().to(routes::documenia::auth::register_page),
    );
    cfg.route(
        "/documenia/register",
        web::post().to(routes::documenia::auth::register),
    );

    // Documenia (PROTECTED)
    cfg.service(
        web::scope("/documenia")
            .wrap(middleware::Authorization)
            .route("", web::get().to(routes::documenia::index::index))
            .route("/logout", web::get().to(routes::documenia::auth::logout))
            .route(
                "/c",
                web::get().to(routes::documenia::character::create_character_page),
            )
            .route(
                "/c",
                web::post().to(routes::documenia::character::create_character),
            )
            .route(
                "/c/{id}",
                web::get().to(routes::documenia::character::character_detail),
            )
            .route(
                "/c/{id}",
                web::delete().to(routes::documenia::character::character_detail),
            )
    );

    // Graphia (PUBLIC)
    cfg.route(
        "/graphia/login",
        web::get().to(routes::graphia::auth::login_page),
    );
    cfg.route(
        "/graphia/login",
        web::post().to(routes::graphia::auth::login),
    );
    cfg.route(
        "/graphia/register",
        web::get().to(routes::graphia::auth::register_page),
    );
    cfg.route(
        "/graphia/register",
        web::post().to(routes::graphia::auth::register),
    );

    // Graphia (PROTECTED)
    cfg.service(
        web::scope("/graphia")
            .wrap(middleware::Authorization)
            .route("", web::get().to(routes::graphia::index::index))
            .route("/logout", web::get().to(routes::graphia::auth::logout))
            .route(
                "/c",
                web::get().to(routes::graphia::character::create_character_page),
            )
            .route(
                "/c",
                web::post().to(routes::graphia::character::create_character),
            )
            .route(
                "/c/{id}",
                web::get().to(routes::graphia::character::character_detail),
            )
            .route(
                "/c/{id}",
                web::delete().to(routes::graphia::character::character_detail),
            )
            .route(
                "/c/{id}/i",
                web::post().to(routes::graphia::inventory::loot_list),
            )
            // .route(
            //     "/c/{id}/i/{item_piece_id}",
            //     web::put().to(routes::graphia::inventory::loot_details),
            // )
            // .route(
            //     "/c/{id}/i/{item_piece_id}",
            //     web::patch().to(routes::graphia::inventory::loot_details),
            // )
            // .route(
            //     "/c/{id}/i/{item_piece_id}",
            //     web::delete().to(routes::graphia::inventory::loot_details),
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
