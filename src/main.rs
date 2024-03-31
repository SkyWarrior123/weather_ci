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
    description: String, // weather description
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
        "https://api.openweathermap.org/data/2.5/weather?q={}, {}&units=metric&appid={}",
        city, country_code, api_key
    );

    // GET request to api endpoint
    let response = reqwest::blocking::get(&url)?;
    // Paring the response data to <WeatherResponse> struct
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info (response: &WeatherResponse) {
    // Extracting weather infor from the res
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // foramtting all of the weather info into a string
    let weather_text = format!(
        "Weather in {}: {} {} 
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1}hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Allocating colors to the weather text based on weather conditions.
    let text_color = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", text_color);

}

fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature  < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature  < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }

}


fn main() {
    println!("{}", "Welcome to your CLI news reporter".bright_yellow());

    loop {
        println!("{}", "Please enter the name of your desired city weather info".bright_green());

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();


        println!("{}", "Please enter the country code (e.g., US for United States, FR for France):".bright_green());

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        let api_key = "51b54cd0bc4448007a48f3798d9a5827";

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response); // Displaying weather information
            } 
            // error message on error fetching data
            Err(err) => {
                eprintln!("Error, {}", err); 
            }
        }

        println!("{}" , "Do you want to search for weather info in anyother city ? (y/n)".bright_green());

        let mut input = String::new();
        io::stdin().read_line(& mut input).expect("Fail to read");
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("{}", "Thanks for using the CLI weather app! Powered by RUST ❤️".bright_red());
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
