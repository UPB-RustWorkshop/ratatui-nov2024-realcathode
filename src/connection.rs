use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

struct CityInfo {
    city: String,
    description: String,
    temp: f32,
    feel_like: f32,
    humidity: f32,
    pressure: f32,
    wind: f32
}
const APIKEY: &str = "";

pub fn get_data(city: String) {
    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit={}&appid={}", city, 5, APIKEY);
    match reqwest::blocking::get(url) {
        Ok(response) => {

        },
        Err(error) => {
            
        }
    }
}