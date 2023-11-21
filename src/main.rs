use crate::routes::parameter::get_parameters;
use crate::routes::region::get_regions;
use crate::routes::weather::get_predictions;
use axum::extract::FromRef;
use axum::routing::get;
use axum::{Router, ServiceExt};
use dotenvy::dotenv;
use nusaca::database::DatabaseSettings;
use nusaca::scraper::scrape_weathers;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use thiserror::Error;
use tokio_cron_scheduler::JobScheduler;
use tower_http::trace::{self, TraceLayer};
use tracing::{log, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod routes;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
}

async fn create_db_conn() -> Result<DatabaseConnection, Error> {
    let db_uri = env::var("DATABASE_URL").unwrap();
    let mut connection_options = DatabaseSettings::get_connection_options(db_uri.as_str());
    connection_options
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    let db_conn = Database::connect(connection_options)
        .await
        .expect("Failed to connect to database");

    Ok(db_conn)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nusaca=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenv().ok();

    let scheduler = JobScheduler::new().await.unwrap();
    println!("begin scraping at {:?}", chrono::Utc::now());

    let task = tokio::spawn(async move {
        let duration = std::time::Duration::from_secs(24 * 60 * 60);
        let db_conn = create_db_conn().await.unwrap();

        loop {
            tracing::info!("begin scraping at {:?}", chrono::Utc::now());
            // scrape_weathers(&db_conn).await.expect("Failed to scrape");
            tokio::time::sleep(duration).await;
        }
    });

    let port: u16 = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());

    let state = AppState {
        db_connection: create_db_conn().await.unwrap(),
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/regions", get(get_regions))
        .route("/api/weather-parameters", get(get_parameters))
        .route("/api/weather", get(get_predictions))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    let ip = IpAddr::from_str(&host).unwrap();
    let socket = SocketAddr::new(ip, port);
    tracing::info!("listening on {}:{}", socket.ip(), socket.port());
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::info!("Aborting task");
    task.abort();
    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello world"
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("hyper server error")]
    Hyper(#[from] hyper::Error),
    #[error("error connecting to the database")]
    DBConnection(#[from] sea_orm::DbErr),
}
