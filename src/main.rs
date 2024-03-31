use std::io;

use serde::Deserialize;

use colored::*;


// All of the required structs

// struct to deserialize the OpenWeatherMap API 
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>, // Weather information
    main: Main, // Contains main weather info
    wind: Wind, // contains all of the wind info
    name: String, // name of the required location.
}

// represents weather description
#[derive(Deserialize, Debug)]
struct Weather {
    name: String, // weather description
}

// temperature info
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64, // in C
    humidity: f64, // in %
    pressure: f64, // hPA
}

// Wind info
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64, // in m/s
}


fn get_weather_info (city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> 
{
    // The url for API response
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}, {}&unit=metric&appid={}",
        city, country_code, api_key
    );

    // GET request to api endpoint
    let response = reqwest::blocking::get(&url)?;
    // Paring the response data to <WeatherResponse> struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info () {
    
}

fn get_temp_emoji() {

}


fn main() {
}
