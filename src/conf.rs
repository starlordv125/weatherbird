use std::fs;
use std::env;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Deserialize, Serialize)]
pub struct TomlInfo {
    pub lat: String,
    pub long: String,
    pub metric: bool
}

pub fn write_conf(lat: String, long: String, metric: bool) {
    let home = env::home_dir().expect("Error finding home directory").into_os_string().into_string().expect("Error parsing home_dir()");
    let strpath = home + "/.config/weatherbird/weatherbird.toml";
    match fs::metadata(&strpath) {
        Ok(_) => {}
        Err(_) => {
            println!("Creating configuration file in {}", strpath);
            let path = std::path::Path::new(&strpath);
            let prefix = path.parent().expect("Fatal error in write_conf()");
            std::fs::create_dir_all(prefix).expect("Error creating directory");
            fs::File::create(&strpath).expect("Error creating weatherbird.toml");
        }
    }
    let tomlstr = TomlInfo {
        lat: lat,
        long: long,
        metric: metric
    };
    print!("Generating new configuration file...");
    let tomlout = toml::ser::to_string(&tomlstr).expect("Error serializing toml");
    fs::write(strpath, tomlout).expect("Error writing to configuration file");
    println!("Success!")
}

pub fn read_conf() -> TomlInfo {
    let home = env::home_dir().expect("Error finding home directory").into_os_string().into_string().expect("Error parsing home_dir()");
    let strpath = home + "/.config/weatherbird/weatherbird.toml";
    match fs::metadata(&strpath) {
        Ok(_) => {},
        Err(_) => {crate::error("\rConfiguration file does not exist, use \"weatherbird set\" to generate configuration");}
    }
    let toml_in = fs::read_to_string(strpath).expect("Error reading configuration");
    let tomlstr: TomlInfo = toml::de::from_str(&toml_in).unwrap_or_else(|_| {eprintln!("\rConfiguration error: Use \"weatherbird set\" to regenerate configuration");std::process::exit(1)});
    return tomlstr;
}
