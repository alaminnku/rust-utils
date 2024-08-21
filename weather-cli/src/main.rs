use serde::Deserialize;
use std::io;

#[derive(Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let data = response.json()?;
    Ok(data)
}

fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅️"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1}m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    );
    println!("{}", weather_text);
}

fn main() {
    println!("{}", "Welcome to Weather Cli 😊");
    loop {
        println!("{}", "Please enter the name of the city:");
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input");
        let city = city.trim();

        println!(
            "{}",
            "Please enter the country code (e.g., US for United States:)"
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input");
        let country_code = country_code.trim();

        let api_key = "open_weather_map_api_key";
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (y/n):"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Thank you for using our software");
            break;
        }
    }
}
