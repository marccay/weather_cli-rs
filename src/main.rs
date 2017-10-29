extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io;
use std::io::Read;
use serde_json::{Value, Error};

fn main() {

	let mut latitude = String::new();
	let mut longitude = String::new();
	println!("enter latitude:");
	io::stdin().read_line(&mut latitude)
		.expect("failed to read latitude");

	println!("enter longitude:");
	io::stdin().read_line(&mut longitude)
		.expect("failed to read longitude");	

	let mut file = File::open("darksky_key.txt").unwrap();
	let mut key = String::new();
	file.read_to_string(&mut key).unwrap();

	let data = get_darksky_info(String::from(key), latitude, longitude);
	let json: Value = serde_json::from_str(&data).unwrap();

	print_info(json);
}

fn print_info(data: Value) {
	let precip = &data["currently"]["precipProbability"];
	let precip_intensity = &data["currently"]["precipIntensity"];
	let temp = &data["currently"]["temperature"];
	let summary = &data["currently"]["summary"];
	let wind_speed = &data["currently"]["windSpeed"];
	let wind_gust = &data["currently"]["windGust"];
	let current_time = &data["currently"]["time"];

	println!("\ntime: \t\t{}", current_time);	
	println!("temperature: \t{}", temp);
	println!("summary: \t{}", summary);
	println!("precip %: \t{}", precip);
	println!("pIntensity: \t{}", precip_intensity);
	println!("windspeed: \t{}", wind_speed);	
	println!("windgust: \t{}", wind_gust);	

	let info = &data["minutely"]["data"];
	let minutely = &data["minutely"]["summary"];
	println!("\nminutely: \t{}",minutely); 
	for x in 0..61 {
		let probability = &info[x]["precipProbability"];
		let intensity  = &info[x]["precipIntensity"];
		let time_plus = &info[x]["time"];
		if probability != 0 {
			println!("time - {}: probability: {} , intensity: {}", time_plus, probability, intensity);
		}
	}

	let hourly = &data["hourly"]["summary"];
	println!("hourly: \t{}\n", hourly);	
}


fn get_darksky_info(key: String, latitude: String, longitude: String) -> String {
	let mut address = String::from("https://api.darksky.net/forecast/");
	address += &key;
	address += &String::from("/");
	address += &latitude;
	address += &String::from(",");
	address += &longitude;

	let mut request = reqwest::get(&address)
		.expect("failed to send request");

	let mut content = String::new();
	request.read_to_string(&mut content)
		.expect("error writing to file");

	content
}


