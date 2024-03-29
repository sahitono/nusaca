use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use entity::weather_parameter;
use nusaca::base::response::BaseResponse;
use sea_orm::EntityTrait;

#[utoipa::path(
    get,
    path = "/api/weather-parameters",
    responses(
        (status = 200, body = BaseResponse<Vec<weather_parameter::Model>>)
    )
)]
pub async fn get_parameters(State(app): State<AppState>) -> impl IntoResponse {
    let parameters = weather_parameter::Entity::find()
        .all(&app.db_connection)
        .await
        .unwrap();
    Json(BaseResponse { data: parameters })
}
