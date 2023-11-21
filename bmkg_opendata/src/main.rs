use bmkg_opendata::model::ForecastData;
use bmkg_opendata::scraper::scrape_weather;
use serde_xml_rs::from_str;
use std::{env, fs};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let xml_path: &str = match args.len() <= 1 {
        true => "D:\\Documents\\00-self-project\\nusaca\\DigitalForecast-Aceh.xml",
        false => args.last().expect("should provide path"),
    };

    let forecast_data: ForecastData = if xml_path.contains("url") {
        let province_name = xml_path.split('=').last().expect("missing province name");
        scrape_weather(province_name)
            .await
            .expect("Failed to fetch xml")
    } else {
        let contents =
            fs::read_to_string(xml_path).expect("Should have been able to read the file");
        from_str(&contents).unwrap()
    };

    println!("Forecast at: {}", forecast_data.forecast.issue.timestamp);
    println!("There is area: {}", forecast_data.forecast.area.len());
    println!("Checking missing parameter");
    let mut missing_cities: Vec<String> = Vec::new();
    for area in forecast_data.forecast.area {
        let area_name = &area.name.first().expect("missing area name").value;
        match area.parameter {
            None => {
                println!("{} missing", area_name);
                missing_cities.push(area_name.to_owned())
            }
            _ => continue,
        }
    }

    if missing_cities.is_empty() {
        println!("all citiies are ok");
    } else {
        println!("{} cities missing", missing_cities.len());
    }
}
