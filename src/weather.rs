use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub current_condition: Vec<CurrentCondition>,
}

#[derive(Deserialize, Debug)]

pub struct CurrentCondition {
    #[serde(rename = "temp_C")]
    pub temp_c: String,
    pub humidity: String,
    #[serde(rename = "weatherDesc")]
    pub weather_desc: Vec<WeatherDesc>,
}

#[derive(Deserialize, Debug)]
pub struct WeatherDesc {
    pub value: String,
}

pub fn fetch(city: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!("https://wttr.in/{city}?format=j1");
    let response = reqwest::blocking::get(url)?.json::<WeatherResponse>()?;

    Ok(response)
}
