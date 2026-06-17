use std::{env, io::Write};
use serde::Deserialize;
mod print;

struct ArgInfo {
    days: u8,
    set: bool,
}

#[derive(Deserialize, Debug)]
struct Obj {
    current: JsonInfo
}

#[derive(Deserialize, Debug)]
struct JsonInfo {
    weather_code: u8,
    temperature_2m: f64
}

#[tokio::main]
async fn main() {
    println!("Duck v0.1");
    let info: ArgInfo = collect_args();
    match info.set {
        true => {set()}
        false => {
            let json: Obj = meteo_get().await;
            let code = json.current.weather_code;
            //handle_json(json);
            print::print_weather(code);
            println!("Temperature: {}", json.current.temperature_2m)
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
fn set() {
    let mut lat = String::new();
    let mut long = String::new();
    println!("Duck location setup");
    print!("Latitude: ");
    std::io::stdout().flush().expect("Error flushing output");
    std::io::stdin().read_line(&mut lat).expect("Error reading user input");
    print!("Longitude: ");
    std::io::stdout().flush().expect("Error flushing output");
    std::io::stdin().read_line(&mut long).expect("Error reading user input");
    lat = lat.trim().to_string();
    long = long.trim().to_string();
    println!("{},{}", lat, long)
}

async fn meteo_get() -> Obj {
    let response = reqwest::Client::new()
    //.get("https://api.open-meteo.com/v1/forecast?latitude=37.2&longitude=-80.41&daily=weather_code&timezone=auto&forecast_days=1") //debug
    .get("https://api.open-meteo.com/v1/forecast?latitude=37.2&longitude=-80.41&daily=weather_code,temperature_2m_max,temperature_2m_min&current=temperature_2m,weather_code&timezone=auto&wind_speed_unit=mph&temperature_unit=fahrenheit&precipitation_unit=inch")
    .send()
    .await
    .unwrap()
    .json::<Obj>()
    .await
    .unwrap();
    //println!("{:?}", response);
    return response
}
/*
fn handle_json(json: Obj) {
    //let njson = serde_json::to_string(&json).expect("Error serializing");
    //println!("{:?}", json.daily.weather_code);
    //let params: JsonInfo = serde_json::from_value(json).expect("JSON was not parsed");
    //println!("Weather code: {:?}", params.daily[0])
}
*/