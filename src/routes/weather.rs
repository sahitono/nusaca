use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use bmkg_opendata::model::StringOrNumber;
use chrono::{Local, NaiveDate};
use entity::{region, weather_prediction};
use nusaca::base::response::BaseResponse;
use nusaca::response::WeatherResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect, QueryTrait, RelationTrait};
use sea_query::Condition;
use sea_query::JoinType;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct Params {
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[serde(rename = "regionCode")]
    region_code: Option<String>,
    parameter: Option<String>,
    date: Option<NaiveDate>,
    unit: Option<String>,
}

#[utoipa::path(
get,
    path = "/api/weathers",
    responses(
        (status = 200, body = BaseResponse<Vec<RegionRead>>)
    ),
    params(
        ("regionCode" = String, Query, description = "Find weather by region code"),
        ("parameter" = String, Query, description = "Find weather by parameter", nullable = true),
        ("unit" = String, Query, description = "Filter weather by unit", nullable = true),
        ("date" = String, Query, description = "Filter weather by date, default to today", example = "2023-11-23", nullable = true),
    )
)]
pub async fn get_predictions(
    State(app): State<AppState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let date = params.date.unwrap_or(Local::now().date_naive());
    let timestamp = date.format("%Y%m%d");

    let query = weather_prediction::Entity::find()
        .join(JoinType::Join, weather_prediction::Relation::Region.def())
        .filter(
            Condition::all()
                .add(region::Column::Code.eq(params.region_code))
                .add(weather_prediction::Column::Timestamp.like(format!("{}%", timestamp))),
        )
        .apply_if(params.parameter, |q, v| {
            q.filter(weather_prediction::Column::ParameterId.eq(v))
        })
        .apply_if(params.unit, |q, v| {
            q.filter(weather_prediction::Column::Unit.eq(v))
        });

    let models: Vec<WeatherResponse> = query
        .all(&app.db_connection)
        .await
        .unwrap()
        .iter()
        .map(|x| WeatherResponse::try_from(x.clone()).unwrap())
        .collect();

    Json(BaseResponse { data: models })
}
