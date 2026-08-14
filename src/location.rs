//This module uses 2 APIs to get specifically the latitude and longitude of the users public facing IP address

use ureq;
use serde::Deserialize;

#[derive(Deserialize)]
struct Ip
{
    ip: String
}

#[derive(Deserialize)]
struct Loc
{
    lat: f32,
    lon: f32
}

//Gets public facing IP of user and passes it to the coordinate grabbing function
//Passes the latitude and longitude as strings to where it was called
pub fn location_get() -> (String, String){
    let ip_structure = ip_get();
    let ip_json = ip_structure.unwrap();
    let ip = ip_json.ip;

    let loc_link = create_link(ip);
    let loc_structure = coordinates_get(loc_link);
    let loc_json = loc_structure.unwrap();
    let latitude = loc_json.lat.to_string();
    let longitude = loc_json.lon.to_string();

    return (latitude, longitude);
}

//Uses the ipify API to grab the users public facing IP address
fn ip_get() -> Result<Ip, ureq::Error>{
    let response = ureq::get("https://api.ipify.org?format=json")
    .call()?
    .body_mut()
    .read_json::<Ip>();
    return response;
}

//Uses the ipapi API and passes the users public facing IP
//Returns approximate coordinates of the users device
fn coordinates_get(link: String) -> Result<Loc, ureq::Error>{
    let response = ureq::get(link)
    .call()?
    .body_mut()
    .read_json::<Loc>();
    return response;
}

//Creates the full link passed to the coordinates_get function
fn create_link(ip: String) -> (String){
    let full_link = "http://ip-api.com/json/".to_owned() + ip.as_str();
    return full_link
}