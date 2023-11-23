use bmkg_opendata::model::StringOrNumber;
use entity::weather_prediction::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherResponse {
    pub region_id: i32,
    pub parameter_id: String,
    pub unit: String,
    pub value: StringOrNumber,
    pub timestamp: String,
}

impl TryFrom<entity::weather_prediction::Model> for WeatherResponse {
    type Error = ();

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(WeatherResponse {
            region_id: value.region_id,
            parameter_id: value.parameter_id,
            timestamp: value.timestamp.clone(),
            value: match value.value.clone().parse::<f32>() {
                Ok(v) => StringOrNumber::Number(v),
                Err(_) => StringOrNumber::String(value.value),
            },
            unit: value.unit,
        })
    }
}
