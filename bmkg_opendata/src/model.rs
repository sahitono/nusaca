use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForecastData {
    pub forecast: Forecast,
    pub source: String,
    #[serde(rename = "productioncenter")]
    pub production_center: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Forecast {
    pub domain: String,
    pub issue: Issue,
    pub area: Vec<Area>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Issue {
    pub timestamp: i64,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Area {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub r#type: String,
    pub region: String,
    pub level: u32,
    pub tags: String,
    pub description: String,
    pub domain: String,

    pub name: Vec<AreaName>,
    pub parameter: Option<Vec<Parameter>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "name")]
pub struct AreaName {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "lang")]
    pub lang: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub id: String,
    pub description: String,
    pub r#type: String,
    #[serde(rename = "timerange")]
    pub time_range: Vec<TimeRange>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeRange {
    pub r#type: String,
    #[serde(default)]
    pub h: Option<i32>,
    pub datetime: String,
    pub value: Vec<ParameterValue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "value")]
pub struct ParameterValue {
    pub unit: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(f32),
}
