use crate::model::ForecastData;
use serde_xml_rs::from_str;

const DOMAIN: &str = "https://data.bmkg.go.id/DataMKG/MEWS/DigitalForecast/DigitalForecast";

pub async fn scrape_weather(
    province_name: &str,
) -> Result<ForecastData, Box<dyn std::error::Error>> {
    let res = reqwest::get(format!("{}-{}.xml", DOMAIN, province_name))
        .await?
        .text()
        .await?;
    let forecast_data: ForecastData = from_str(&res).unwrap();
    Ok(forecast_data)
}
