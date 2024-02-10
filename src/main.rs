use crate::routes::parameter as weather_parameter;
use crate::routes::region;
use crate::routes::summary;
use crate::routes::weather as weather_prediction;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use nusaca::infrastructure::database::DatabaseSettings;
use nusaca::infrastructure::state::AppState;
use nusaca::scraper::scrape_weathers;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use thiserror::Error;
use tokio::task::JoinHandle;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::{log, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod routes;

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

    let task = spawn_task().await;

    let port: u16 = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .unwrap();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());

    let state = AppState {
        db_connection: create_db_conn().await.unwrap(),
    };

    #[derive(OpenApi)]
    #[openapi(
    info(title = "NUSACA API", description = "Weather prediction from BMKG open data"),
    paths(
    region::get_regions,
    weather_parameter::get_parameters,
    weather_prediction::get_predictions,
    summary::get_daily_summary,
    summary::get_region_summary,
    summary::get_available_dates,
    ),
    servers(
    (url = "http://localhost:3000", description = "Local server"),
    (url = "https://nusaca.sahitono.space", description = "Cloud server")
    ),
    components(
    schemas()
    ),
    tags(
    (name = "region", description = "Region API")
    )
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(
            SwaggerUi::new("/api/docs/swagger-ui").url("/api/docs/openapi.json", ApiDoc::openapi()),
        )
        .merge(RapiDoc::new("/api/docs/openapi.json").path("/api/docs"))
        .route("/", get(hello_world))
        .route("/api/regions", get(region::get_regions))
        .route(
            "/api/weather-parameters",
            get(weather_parameter::get_parameters),
        )
        .route("/api/weathers", get(weather_prediction::get_predictions))
        .route(
            "/api/summary/daily",
            get(routes::summary::get_daily_summary),
        )
        .route(
            "/api/summary/region",
            get(routes::summary::get_region_summary),
        )
        .route(
            "/api/summary/available-dates",
            get(routes::summary::get_available_dates),
        )
        .fallback(handler_404)
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
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

async fn spawn_task() -> JoinHandle<()> {
    tokio::spawn(async move {
        // sleep(std::time::Duration::from_secs(60)).await;

        let duration = std::time::Duration::from_secs(24 * 60 * 60);
        let db_conn = create_db_conn().await.unwrap();

        loop {
            tracing::info!("begin scraping at {:?}", chrono::Utc::now());
            scrape_weathers(&db_conn).await.expect("Failed to scrape");
            tracing::info!("finished scraping at {:?}", chrono::Utc::now());
            tokio::time::sleep(duration).await;
        }
    })
}

async fn hello_world() -> &'static str {
    "Hello world"
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "you might be lost, that's life")
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("hyper server error")]
    Hyper(#[from] hyper::Error),
    #[error("error connecting to the database")]
    DBConnection(#[from] sea_orm::DbErr),
    #[error("error parsing xml")]
    ParseXML(#[from] serde_xml_rs::Error),
}
