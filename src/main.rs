// Weatherbird (C) Cameron Reynolds <cameron@starlordv125.net> 2026
// A simple CLI based weather program

use std::error::Error;
use std::io;
use std::num::ParseIntError;
use std::{env, io::Write};
use serde::Deserialize;
use chrono::{self, NaiveDateTime, Timelike};
use ureq;
use crate::NumArg::*;
use crate::ArgInfo::*;

mod print;
mod conf;
mod location;

// Top level JSON struct, needed to parse JSON response from openmeteo
#[derive(Deserialize)]
struct Obj {
    current: Current,
    daily: Daily,
    hourly: Hourly
}

// Current weather data
#[derive(Deserialize)]
struct Current {
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

enum ArgInfo {
    NumArg(NumArg),
    Set,
    None
}

enum NumArg {
    Days(String),
    Hours(String)
}

impl NumArg {
    pub fn convert(&self) -> Result<usize, Box<dyn Error>> {
        let num = &self.check_parse()?;
        Ok(self.check_size(*num)?)
    }

    fn check_parse(&self) -> Result<usize, ParseIntError> {
        match self {
            Days(num) => {return num.parse::<usize>()}
            Hours(num) => {return num.parse::<usize>()}
        }
    }

    fn check_size(&self, num: usize) -> Result<usize, io::Error> {
        match self {
            Days(_) => {
                if num > 7 || num < 1 {
                    return Err(std::io::Error::other("Number out of range"))
                }
                return Ok(num)
            }
            Hours(_) => {
                if num > 24 || num < 1 {
                    return Err(std::io::Error::other("Number out of range"))
                }
                return Ok(num)
            }
        }
    }
}
// Change this when upgrading
const VERSION: &str = "v0.6.2";

fn main() {
    let info: ArgInfo = collect_args();
    match info {
        Set => {
            let (lat, long, metric) = set();
            conf::write_conf(lat, long, metric);
            std::process::exit(0)
        }
        ArgInfo::NumArg(arg) => {
            let num: usize = arg.convert().unwrap_or_else(|err| {error(&err.to_string());0});
            match arg {
                Hours(_) => {
                    let json: Obj = meteo_get(2).unwrap_or_else(|error| {eprintln!("\rError: {}", error);std::process::exit(2)});
                    forecast_hourly(json.hourly, num);
                }
                Days(_) => {
                    let json: Obj = meteo_get(num.try_into().expect("Critical error")).unwrap_or_else(|error| {eprintln!("\rError: {}", error);std::process::exit(2)});
                    forecast(json.daily, num);
                }
            }
        }
        None => {
            let json: Obj = meteo_get(1).unwrap_or_else(|error| {eprintln!("\rError: {}", error);std::process::exit(2)});
            let code = json.current.weather_code;
            let is_day = json.current.is_day;
            print::print_weather(code, is_day);
            println!("Temperature: {}", json.current.temperature_2m);
            println!("Min: {}", json.daily.temperature_2m_min[0]);
            println!("Max: {}", json.daily.temperature_2m_max[0]);
        }
    }
}

// Collects arguements and does some basic error handling on its own
// num_next will make the program expect a number for the next arguement
fn collect_args() -> ArgInfo {
    let mut arg_var: ArgInfo = None;
    let args: Vec<String> = env::args().collect();
    let mut num_next: bool = false;
    let mut invalid_format: bool = false;
    for arg in &args[1..] {
        if invalid_format == true {
            error("Invalid arguement format")
        }
        match num_next {
            true => {
                match arg_var {
                    NumArg(Days(_)) => arg_var = NumArg(Days(arg.to_string())),
                    NumArg(Hours(_)) => arg_var = NumArg(Hours(arg.to_string())),
                    _ => panic!("Arguement error")
                }
                num_next = false;
                invalid_format = true;
            }
            false => {
                match arg.as_str() {
                "set" => {arg_var = Set;invalid_format = true}
                "days" => {num_next = true;arg_var = NumArg(Days(String::new()));}
                "hours" => {num_next = true;arg_var = NumArg(Hours(String::new()));}
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
    return arg_var
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
    println!("|set -> Interactive settings menu, will overwrite previous configuration               |");
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

// Gets coordinates either automatically or manually from the user and returns them to main
// Handles errors on its own
fn set() -> (String, String, bool){ 
    let mut metric = String::new();
    let mut auto = String::new();
    let mut metric_bool: bool = false;
    let mut lat = String::new();
    let mut long = String::new();
    println!("Weatherbird location setup");
    println!("Would you like Weatherbird to automatically set location?");
    print!("(Y or N, not recomended if using vpn): ");
    auto = flush_read(auto);
    match auto.as_str().trim() {
        "Y" | "y" => {(lat, long) = automatic_setup()}
        "N" | "n" => {(lat, long) = manual_setup()}
        _ => {error("Invalid character(s)");}
    }
    print!("Use Metric system?(Y or N): ");
    metric = flush_read(metric);
    match metric.as_str().trim() {
        "Y" | "y" => {metric_bool = true}
        "N" | "n" => {}
        _ => {error("Invalid character(s)");}
    }
    return (lat, long, metric_bool);
}

//Allows the user to manually set their location using coordinates
fn manual_setup() -> (String, String) {
    let mut lat = String::new();
    let mut long = String::new();
    print!("Latitude: ");
    lat = flush_read(lat);
    input_error_check(lat.as_str());
    print!("Longitude: ");
    long = flush_read(long);
    input_error_check(long.as_str());
    return (lat.trim().to_string(), long.trim().to_string())
}

fn flush_read(mut input: String) -> String {
    std::io::stdout().flush().expect("Error flushing output");
    std::io::stdin().read_line(&mut input).expect("Error reading user input");
    return input
}

//Automatically sets the users coordinates
fn automatic_setup() -> (String, String){
    let mut lat = String::new();
    let mut long = String::new();
    let mut allow = String::new();
    println!("By using automatic setup you are allowing Weatherbird to access your public IP and forward it to the ipapi service.");
    print!("Would you still like to continue?(Y or N): ");
    allow = flush_read(allow);
    match allow.as_str().trim() {
        "Y" | "y" => {(lat, long) = location::location_get()}
        "N" | "n" => {std::process::exit(0)}
        _ => {error("Invalid character(s)");}
    }
    return (lat, long);
}

// Mainly used for set(), but can be expanded for other functions in the future
fn input_error_check(num: &str) {
    match num.trim().parse::<f64>() {
        Ok(_) => {}
        Err(_) => {error("Invalid character(s)");}
    }
}

// This function takes the coordinates and
// combines them with the URL to get weather data for the area.
// For now it's just one big URL that gets all of the data possibly needed for the program,
// that can be changed in the future
// TODO: Allow for metric units
fn meteo_get(days: u8) -> Result<Obj, ureq::Error> {
    print!("Fetching weather...");
    let days: String = days.to_string();
    std::io::stdout().flush().expect("Error flushing output");
    let conf: conf::TomlInfo = conf::read_conf();
    let temp = match conf.metric {
        true => {"celsius"}
        false => {"fahrenheit"}
    };
    let link: String = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + conf.lat.as_str() + "&longitude=" + conf.long.as_str() + "&timezone=auto&daily=weather_code,temperature_2m_max,temperature_2m_min&current=temperature_2m,weather_code,is_day&temperature_unit=" + temp + "&wind_speed_unit=mph&precipitation_unit=inch&hourly=temperature_2m,weather_code&forecast_days=" + &days;
    let response = ureq::get(link)
    .call()?
    .body_mut()
    .read_json::<Obj>();
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
    let weather_codes: Vec<&str> = code_alloc(hourly_info.weather_code, hour_index_end);
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
fn code_alloc(codes: Vec<u8>, size: usize) -> Vec<&'static str> {
    let mut weathers: Vec<&str> = Vec::new();
    for num in 0..size {
        weathers.push(match codes[num] {
            0 => {"Clear"}
            1 | 2 => {"Partly cloudy"}
            3 => {"Overcast"}
            45 | 48 => {"Foggy"}
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
        )
    }
    return weathers;
}

