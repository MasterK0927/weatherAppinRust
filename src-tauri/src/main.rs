// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env,fs};
use dotenv::dotenv;
use reqwest::Client;
use crate::model::{Geo, Response, WeatherInfo};

mod model;

#[tauri::command]
fn get_geo_info() -> Vec<Geo> {
  let data = fs::read_to_string("./data/geo.json")
      .expect("Unable to read from file");
  
  let mut get_info : Vec<Geo> = serde_json::from_str(&data).unwrap();

  geo_info.sort_by(|a,b| a.country.cmp(&b.country));

  geo_info
}

#[tauri::command]
async fn get_weather(last: f64, long: f64) -> WeatherInfo {
  dotenv().ok();

  let key = env::var("KEY").expect("Error reading key");
  let url = format!("https://api.weatherbit.io/v2.0/history/subhourly?lat={}&lon={}&start_date=2024-12-03&end_date=2024-12-07&key={}",lat. long, key);
  let response = Client::new()
      .get(url)
      .send()
      .await
      .unwrap()
      .json::<Response>()
      .await
      .unwrap();
  let data = response.to_info();
  println!("{:?}",data);
  data
}

fn main() {
  app_lib::run();
}
