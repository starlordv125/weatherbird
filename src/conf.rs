use std::fs;

const CONF: &str = "~/.config/duck/duck.conf";

pub fn write_conf() {
    match fs::metadata(CONF) {
        Ok(_) => {println!("Exists!")}
        Err(_) => {
            println!("Creating configuration file in {}", CONF);
            let path = std::path::Path::new(CONF);
            let prefix = path.parent().expect("Fatal error in write_conf()");
            std::fs::create_dir_all(prefix).expect("Error creating directory");
        }
    }
}