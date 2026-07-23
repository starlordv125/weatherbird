// Weatherbird (C) Cameron Reynolds <cameron@starlordv125.net> 2026
// A simple CLI based weather program

use std::{env, io::Write};
use serde::Deserialize;
use chrono::{self, NaiveDateTime, Timelike};

mod print;
mod conf;

// Struct for all info collected from arguements
struct ArgInfo {
    num: usize,
    forecast: bool,
    hours: bool,
    set: bool,
}

// Top level JSON struct, needed to parse JSON response from openmeteo
#[derive(Deserialize)]
struct Obj {
    current: JsonInfo,
    daily: Daily,
    hourly: Hourly
}

// Current weather data
// TODO: Rename this struct to "Current"
#[derive(Deserialize)]
struct JsonInfo {
    weather_code: u8,
    temperature_2m: f32,
    is_day: u8
}

// Daily weather data
#[derive(Deserialize)]
struct Daily {
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    weather_code: Vec<u8>,
    time: Vec<String>
}

#[derive(Deserialize)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f32>,
    weather_code: Vec<u8>
}

// Change this when moving to a new version
const VERSION: &str = "v0.5.0-4";

// Tokio is needed for Reqwest, which is needed to interact with openmeteo API
#[tokio::main]
async fn main() {
    let info: ArgInfo = collect_args();
    // Checks if multiple arguements are passed
    if info.set == true && info.forecast == true {
        error("Multiple arguements cannot be used at the same time");
    }
    if info.set == true {
        let (lat, long) = set();
        conf::write_conf(lat, long);
        std::process::exit(0);
    }
    if info.hours == true {
        if info.num > 24 || info.num < 1 {
            error("Hour out of range");
        }
        let json: Obj = meteo_get(2).await;
        forecast_hourly(json.hourly, info.num);
        std::process::exit(0);
    }
    if info.forecast == true {
        if info.num > 7 || info.num < 1 {
            error("Day out of range");
        }
        let json: Obj = meteo_get(info.num.try_into().expect("Critical error")).await;
        forecast(json.daily, info.num);
        std::process::exit(0);
    }
    let json: Obj = meteo_get(info.num.try_into().expect("Critical error")).await;
    let code = json.current.weather_code;
    let is_day = json.current.is_day;
    print::print_weather(code, is_day);
    println!("Temperature: {}", json.current.temperature_2m);
    println!("Min: {}", json.daily.temperature_2m_min[0]);
    println!("Max: {}", json.daily.temperature_2m_max[0]);
}

// Collects arguements and does some basic error handling on its own
// num_next will make the program expect a number for the next arguement
fn collect_args() -> ArgInfo {
    let args: Vec<String> = env::args().collect();
    let mut num_next: bool = false;
    let mut info = ArgInfo {
        num: 1,
        set: false,
        forecast: false,
        hours: false,
    };
    for arg in &args[1..] {
        match num_next {
            true => {
                match arg.parse::<usize>() {
                    Ok(o) => {info.num = o;}
                    Err(_) => {error("Invalid number");}
                }
                num_next = false;
            }
            false => {
                match arg.as_str() {
                "set" => {info.set = true}
                "days" => {num_next = true;info.forecast = true}
                "hours" => {num_next = true;info.hours = true}
                "--version" | "-v" => {println!("{}", VERSION);std::process::exit(0)}
                "--help" | "-h" => {help()}
                _ => {error(&("Unrecognized arguement: \"".to_owned() + arg + "\""))}
                }
            }
        }
    }
    match num_next {
        true => {error("Number not specified");}
        false => {}
    }
    return info
}

// Help menu when "-h" or "--help" is passed
fn help() {
    println!("|--------------------------------------------------------------------------------------|");
    println!("|Weatherbird version {} Copyright (C) 2026 Cameron Reynolds                        |", VERSION); //offset from variable
    println!("|License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>        |");
    println!("|This program comes with ABSOLUTELY NO WARRANTY                                        |");
    println!("|This is free software, and you are welcome to redistribute it under certain conditions|");
    println!("|--------------------------------------------------------------------------------------|");
    println!("|Arguements                                                                            |");
    println!("|--help or -h -> Displays this menu                                                    |");
    println!("|--version or -v -> Shows version number                                               |");
    println!("|set -> Allows you to set coordinates, will overwrite previous configuration           |");
    println!("|days [1-7] -> Shows a forecast of up to seven days                                    |");
    println!("|hours [1-24] -> Shows a forecast of up to twenty-four hours                           |");
    println!("|--------------------------------------------------------------------------------------|");
    println!("|Repo: https://forgejo.starlordv125.net/starlordv125/weatherbird                       |");
    println!("|Maintainer email: <cameron@starlordv125.net>                                          |");
    println!("|--------------------------------------------------------------------------------------|");
    std::process::exit(0); // change later
}

// error() can be called by any function and will exit the program
// TODO: Add exit code passing so each error will have different code
fn error(message: &str) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

// Will ask the user for coordinates and return them to main
// Handles errors on its own
fn set() -> (String, String) { 
    let mut lat = String::new();
    let mut long = String::new();
    println!("Weatherbird location setup");
    print!("Latitude: ");
    std::io::stdout().flush().expect("Error flushing output");
    std::io::stdin().read_line(&mut lat).expect("Error reading user input");
    input_error_check(lat.as_str());
    print!("Longitude: ");
    std::io::stdout().flush().expect("Error flushing output");
    std::io::stdin().read_line(&mut long).expect("Error reading user input");
    input_error_check(long.as_str());
    lat = lat.trim().to_string();
    long = long.trim().to_string();
    return (lat, long);
}

// Mainly used for set(), but can be expanded for other functions in the future
fn input_error_check(num: &str) {
    match num.trim().parse::<f64>() {
        Ok(_) => {}
        Err(_) => {error("Value entered is not parseable");}
    }
}

// Asynchronous for Reqwest, this function takes the coordinates and
// combines them with the URL to get weather data for the area.
// For now it's just one big URL that gets all of the data possibly needed for the program,
// that can be changed in the future
// TODO: Allow for metric units
async fn meteo_get(days: u8) -> Obj {
    print!("Fetching weather...");
    let days: String = days.to_string();
    std::io::stdout().flush().expect("Error flushing output");
    let conf: conf::TomlInfo = conf::read_conf();
    let link: String = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + conf.lat.as_str() + "&longitude=" + conf.long.as_str() + "&timezone=auto&daily=weather_code,temperature_2m_max,temperature_2m_min&current=temperature_2m,weather_code,is_day&temperature_unit=fahrenheit&wind_speed_unit=mph&precipitation_unit=inch&hourly=temperature_2m,weather_code&forecast_days=" + &days;
    let response = reqwest::Client::new()
    .get(&link)
    .send()
    .await
    .expect("Error connecting to openmeteo")
    // .json uses serde::json
    .json::<Obj>()
    .await
    .expect("Error parsing JSON");
    return response
}

// Runs a for loop that prints each days' max and min temp, and the weather
// based off of the weather code
fn forecast(daily_info: Daily, days: usize) {
    let weather_codes = code_alloc(daily_info.weather_code, days);
    for num in 0..days {
        println!("\r-------------------");
        println!("Date: {}", daily_info.time[num]);
        println!("Weather: {}", weather_codes[num]);
        println!("Max temp: {}", daily_info.temperature_2m_max[num]);
        println!("Min temp: {}", daily_info.temperature_2m_min[num]);
    }
    println!("-------------------");
}

// Need to find which hour is current hour in vector
fn forecast_hourly(hourly_info: Hourly, hours: usize) {
    let local_time = chrono::Local::now();
    let mut hour_index: usize = 0;
    for hour in &hourly_info.time {
        if NaiveDateTime::parse_from_str(&hour, "%Y-%m-%dT%H:%M").unwrap().hour() == local_time.hour() {
            break;
        }
        hour_index += 1;
    }
    let hour_index_end: usize = hour_index + hours;
    let weather_codes: Vec<String> = code_alloc(hourly_info.weather_code, hour_index_end);
    for num in hour_index..hour_index_end {
        println!("\r-------------------");
        println!("Hour: {}", NaiveDateTime::parse_from_str(&hourly_info.time[num], "%Y-%m-%dT%H:%M").unwrap().hour());
        println!("Temp: {}", hourly_info.temperature_2m[num]);
        println!("Weather: {}", weather_codes[num]);
    }
    println!("-------------------");
}


// Used for both forecast() and forecast_hourly(), this converts weather codes
// into corresponding descriptions of the weather
fn code_alloc(codes: Vec<u8>, size: usize) -> Vec<String> {
    let mut weathers: Vec<String> = Vec::new();
    for num in 0..size {
        weathers.push((match codes[num] {
            0 => {"Clear"}
            1 | 2 => {"Partly cloudy"}
            3 => {"Overcast"}
            51 | 53 | 55 => {"Light rain"}
            61 | 63 | 65 | 80 | 81 | 82 => {
                "Heavy rain"
            }
            66 | 67 => {"Freezing rain"}
            71 | 73 | 77 => {"Light snow"}
            75 | 85 | 86 => {"heavy snow"}
            95 => {"Stormy"}
            96 | 99 => {"Hail"}
            _ => {"Unknown"}
            }
        ).to_string())
    }
    return weathers;
}

