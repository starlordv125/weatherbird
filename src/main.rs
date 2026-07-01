use std::{env, io::Write};
use serde::Deserialize;
mod print;
mod conf;

struct ArgInfo {
    days: u8,
    set: bool,
}

#[derive(Deserialize)]
struct Obj {
    current: JsonInfo,
    daily: Daily
}

#[derive(Deserialize)]
struct JsonInfo {
    weather_code: u8,
    temperature_2m: f64,
    is_day: u8
}

#[derive(Deserialize)]
struct Daily {
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>
}

const VERSION: &str = "v0.3.2";

#[tokio::main]
async fn main() {
    //println!("Duck {}", VERSION);
    let info: ArgInfo = collect_args();
    match info.set {
        true => {
            let (lat, long) = set();
            conf::write_conf(lat, long);
        }
        false => {
            // let lat, long = readconf()
            let json: Obj = meteo_get().await;
            let code = json.current.weather_code;
            let is_day = json.current.is_day;
            print::print_weather(code, is_day);
            println!("Temperature: {}", json.current.temperature_2m);
            println!("Min: {}", json.daily.temperature_2m_min[0]);
            println!("Max: {}", json.daily.temperature_2m_max[0]);
        }
    }
}

fn collect_args() -> ArgInfo {
    let args: Vec<String> = env::args().collect();
    let mut days_next: bool = false;
    //let mut arg_error = false;
    let mut info = ArgInfo {
        days: 7,
        set: false
    };
    for arg in &args[1..] {
        /*match arg_error {
            true => {error("Incorrect arguement format")}
            false => {}
        }*/
        match days_next {
            // reformat this
            true => {
                match arg.parse::<u8>() {
                    Ok(o) => {println!("Days: {}", o);info.days = o}
                    Err(_) => {error(&("Unrecognized arguement: \"".to_owned() + arg + "\""))}
                }
            }
            false => {
                match arg.as_str() {
                "set" => {info.set = true}
                "--days" => {days_next = true}
                "--version" => {println!("{}", VERSION);std::process::exit(0)}
                "--help" => {help()}
                _ => {error(&("Unrecognized arguement: \"".to_owned() + arg + "\""))}
                }
            }
        }
    }
    return info
}

fn help() {
    println!("Weatherbird version {} Copyright (C) 2026 Cameron Reynolds", VERSION);
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>");
    println!("This program comes with ABSOLUTELY NO WARRANTY");
    println!("This is free software, and you are welcome to redistribute it under certain conditions");
    println!("--------------------------------------------------------------------------------------");
    println!("Arguements");
    println!("--help -> Displays this menu");
    println!("--version -> Shows version number");
    println!("set -> Allows you to set coordinates, will overwrite previous configuration");
    println!("--------------------------------------------------------------------------------------");
    println!("Repo: https://forgejo.starlordv125.net/starlordv125/duck");
    println!("Maintainer email: cameron@starlordv125.net");
    std::process::exit(0); // change later
}

// error() can be called by any function and will exit the program
fn error(message: &str) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

fn set() -> (String, String) { 
    let mut lat = String::new();
    let mut long = String::new();
    println!("Duck location setup");
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

fn input_error_check(num: &str) {
    match num.trim().parse::<f64>() {
        Ok(_) => {}
        Err(_) => {error("Value entered is not parseable");}
    }
}

async fn meteo_get() -> Obj {
    print!("Fetching weather...");
    std::io::stdout().flush().expect("Error flushing output");
    let conf: conf::TomlInfo = conf::read_conf();
    let link: String = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + conf.lat.as_str() + "&longitude=" + &conf.long.as_str() + "&daily=temperature_2m_max,temperature_2m_min&current=temperature_2m,weather_code,is_day&timezone=auto&wind_speed_unit=mph&temperature_unit=fahrenheit&precipitation_unit=inch";
    let response = reqwest::Client::new()
    .get(&link)
    .send()
    .await
    .expect("Error connecting to openmeteo")
    .json::<Obj>()
    .await
    .expect("Error parsing JSON");
    return response
}
