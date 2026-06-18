use std::fs;

const CONF: &str = "~/.config/duck/duck.conf";

pub fn write_conf() {
    match fs::metadata(CONF) {
        Ok(_) => {println!("Exists!")}
        Err(_) => {println!("Does not exist")}
    }
}