use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use entity::region::Model;
use nusaca::base::response::BaseResponse;
use nusaca::use_cases;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct Params {
    keyword: Option<String>,
    #[serde(rename = "parentId")]
    parent_id: Option<i32>,
}

#[derive(Serialize, ToSchema)]
struct RegionRead {
    pub id: i32,
    pub code: String,
    pub name_en: String,
    pub name_id: String,
    pub parent_id: i32,
    pub longitude: f64,
    pub latitude: f64,
}

impl TryFrom<entity::region::Model> for RegionRead {
    type Error = &'static str;
    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(RegionRead {
            id: value.id,
            code: value.code,
            name_en: value.name_en,
            name_id: value.name_id,
            parent_id: value.parent_id.unwrap_or(0),
            longitude: value.longitude,
            latitude: value.latitude,
        })
    }
}

#[utoipa::path(
    get,
    path = "/api/regions",
    responses(
        (status = 200, body = BaseResponse<Vec<RegionRead>>)
    ),
    params(
        ("keyword" = String, Query, description = "Find region by name", nullable = true),
        ("parentId" = i32, Query, description = "Find region by parent", nullable = true),
    )
)]
pub async fn get_regions(
    State(app): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let regions =
        use_cases::region::get_regions(&app.db_connection, params.keyword, params.parent_id)
            .await
            .unwrap()
            .iter()
            .map(|v| RegionRead::try_from(v.clone()).unwrap())
            .collect::<Vec<RegionRead>>();
    let response = BaseResponse { data: regions };
    Json(response)
}

#[utoipa::path(
    get,
    path = "/api/regions/{regionCode}",
    responses(
        (status = 200, body = BaseResponse<Vec<RegionRead>>)
    ),
    params(
        ("regionId" = String, Query, description = "Find region by parent"),
    )
)]
pub async fn get_region_by_code(State(app): State<AppState>, code: &str) -> impl IntoResponse {
    let regions = use_cases::region::get_region_by_code(&app.db_connection, code)
        .await
        .unwrap();

    let response = BaseResponse { data: regions };
    Json(response)
}
