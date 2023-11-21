use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{Json, Router};
use entity::region;
use nusaca::base::response::BaseResponse;
use nusaca::use_cases;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    keyword: Option<String>,
}

pub async fn get_regions(
    State(app): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let regions = use_cases::region::get_regions(&app.db_connection, params.keyword)
        .await
        .unwrap();
    let response = BaseResponse { data: regions };
    Json(response)
}

pub async fn get_region_by_code(State(app): State<AppState>, code: &str) -> impl IntoResponse {
    let regions = use_cases::region::get_region_by_code(&app.db_connection, code)
        .await
        .unwrap();

    let response = BaseResponse { data: regions };
    Json(response)
}
