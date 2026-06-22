use std::{env, io::Write};
use serde::Deserialize;
mod print;
mod conf;

struct ArgInfo {
    days: u8,
    set: bool,
}

#[derive(Deserialize, Debug)]
struct Obj {
    current: JsonInfo,
    daily: Daily
}

#[derive(Deserialize, Debug)]
struct JsonInfo {
    weather_code: u8,
    temperature_2m: f64
}

#[derive(Deserialize, Debug)]
struct Daily {
    temperature_2m_max: Vec<f64>,
    temperature_2m_min: Vec<f64>
}

const VERSION: &str = "v0.3.0";

#[tokio::main]
async fn main() {
    println!("Duck {}", VERSION);
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
            print::print_weather(code);
            println!("Temperature: {}", json.current.temperature_2m);
            println!("Min: {}", json.daily.temperature_2m_min[0]);
            println!("Max: {}", json.daily.temperature_2m_max[0]);
        }
    }
}

fn collect_args() -> ArgInfo {
    let args: Vec<String> = env::args().collect();
    let mut days_next: bool = false;
    let mut info = ArgInfo {
        days: 7,
        set: false
    };
    for arg in &args[1..] {
        match days_next {
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
                 _ => {error(&("Unrecognized arguement: \"".to_owned() + arg + "\""))}
                }
            }
        }
    }
    return info
}

// error() can be called by any function and will exit the program
fn error(message: &str) {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

// Will store config in ~/.config/duck/duck.conf
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
    let conf: conf::TomlInfo = conf::read_conf();
    let link: String = "https://api.open-meteo.com/v1/forecast?latitude=".to_owned() + conf.lat.as_str() + "&longitude=" + &conf.long.as_str() + "&daily=temperature_2m_max,temperature_2m_min,weather_code&current=temperature_2m,weather_code&timezone=auto&wind_speed_unit=mph&temperature_unit=fahrenheit&precipitation_unit=inch";
    let response = reqwest::Client::new()
    .get(link)
    .send()
    .await
    .unwrap()
    .json::<Obj>()
    .await
    .unwrap();
    return response
}
