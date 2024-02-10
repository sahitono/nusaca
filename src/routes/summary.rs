use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use nusaca::base::response::BaseResponse;
use nusaca::infrastructure::state::AppState;
use nusaca::use_cases;

#[utoipa::path(
    get,
    path = "/api/summary/daily",
    responses(
        (status = 200, body = BaseResponse<Vec<DailySummary>>)
    )
)]
pub async fn get_daily_summary(State(app): State<AppState>) -> impl IntoResponse {
    let summary = use_cases::summary::get_daily_summary(&app.db_connection)
        .await
        .unwrap();

    Json(BaseResponse { data: summary })
}

#[utoipa::path(
    get,
    path = "/api/summary/region",
    responses(
        (status = 200, body = BaseResponse<Vec<DailySummary>>)
    )
)]
pub async fn get_region_summary(State(app): State<AppState>) -> impl IntoResponse {
    let summary = use_cases::summary::get_region_summary(&app.db_connection)
        .await
        .unwrap();

    Json(BaseResponse { data: summary })
}

#[utoipa::path(
    get,
    path = "/api/summary/available-dates",
    responses(
        (status = 200, body = BaseResponse<Vec<DailySummary>>)
    )
)]
pub async fn get_available_dates(State(app): State<AppState>) -> impl IntoResponse {
    let summary = use_cases::summary::get_available_date(&app.db_connection)
        .await
        .unwrap();

    Json(BaseResponse { data: summary })
}
