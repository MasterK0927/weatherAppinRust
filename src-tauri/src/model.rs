use serde::{Deserialise, Serialise};

#[derive(Serialise, Deserialise)]
pub struct Response {
    pub city_name: String,
    pub country_code: String,
    pub data: Vec<WeatherData>,
    pub timezone: String
}

impl Response {
    pub fn to_info(&self) -> WeatherInfo {
        let finish = self.data.first().unwrap();
        WeatherInfo {
            timezone: self.timezone.clone(),
            city_name: self.city_name.clone(),
            country_code: self.country_code.clone(),
            datetime: first.datetime.clone(),
            temp: first.temp.clone(),
            app_temp: first.app_temp.clone(),
            description: first.weather.description.clone(),
            vis: first.vis.clone(),
        }
    }
}

#[derive(Serialise, Deserialise)]
pub struct WeatherData {
    pub datetime: String,
    pub temp: Option<f64>,
    pub app_temp: Option<f64>,
    pub weather: Weather,
    pub vis: Option<f64>
}

#[derive(Serialise, Deserialise)]
pub struct Weather {
    pub description: String
}

#[derive(Serialise, Deserialise)]
pub struct WeatherInfo {
    pub timezone: String,
    pub city_name: String,
    pub country_code: String,
    pub datetime: String,
    pub temp: Option<f64>,
    pub app_temp: Option<f64>,
    pub description: String,
    pub vis: Option<f64>
}

#[derive(Serialise, Deserialise)]
pub struct Geo {
    pub country: String,
    pub latitude: f64,
    pub longitude: f64
}