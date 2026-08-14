//This module uses 2 APIs to get specifically the latitude and longitude of the users public facing IP address

use ureq;
use serde::Deserialize;

#[derive(Deserialize)]
struct Ip {
    ip: String
}

#[derive(Deserialize)]
struct Loc {
    lat: f32,
    lon: f32
}

//Gets public facing IP of user and passes it to the coordinate grabbing function
//Passes the latitude and longitude as strings to where it was called
pub fn location_get() -> (String, String) {
    let mut ip = String::new();
    match ip_get() {
        Ok(o) => {ip = o.ip}
        Err(_) => {crate::error("Network error");}
    }
    let loc_link = create_link(ip);
    let mut lat = String::new();
    let mut lon = String::new();
    match coordinates_get(loc_link) {
        Ok(o) => {lat = o.lat.to_string();lon = o.lon.to_string()}
        Err(_) => {crate::error("Network error");}
    }

    return (lat, lon);
}

//Uses the ipify API to grab the users public facing IP address
fn ip_get() -> Result<Ip, ureq::Error> {
    let response = ureq::get("https://api.ipify.org?format=json")
    .call()?
    .body_mut()
    .read_json::<Ip>();
    return response;
}

//Uses the ipapi API and passes the users public facing IP
//Returns approximate coordinates of the users device
fn coordinates_get(link: String) -> Result<Loc, ureq::Error> {
    let response = ureq::get(link)
    .call()?
    .body_mut()
    .read_json::<Loc>();
    return response;
}

//Creates the full link passed to the coordinates_get function
fn create_link(ip: String) -> String {
    return "http://ip-api.com/json/".to_owned() + ip.as_str();
}