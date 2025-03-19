// will use these later to create planet jsons
use rand::Rng;
use serde_json::json;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use serde_json::Value;


use rand::seq::SliceRandom;
//use serde::Deserialize;
use std::fs;

use crate::planet_utils;

fn create_planet_json()
{
}

// Useful for single list json only. The "planet_tags" list won't work
fn get_random_entry(filename: &str, key: &str) -> Result<String, Box<dyn Error>> {
    // Read file content
    let file_content = fs::read_to_string(filename)?;

    // Parse as generic JSON
    let json: Value = serde_json::from_str(&file_content)?;

    // Try to extract the array corresponding to the key
    let entries = json.get(key)
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("Key '{}' not found or not an array in {}", key, filename))?;

    // Pick a random entry
    let mut rng = rand::thread_rng();
    let random_entry = entries.choose(&mut rng)
        .and_then(|v| v.as_str())
        .ok_or("No valid string entries found")?;

    Ok(random_entry.to_string())
}

pub fn test_planet_gen() {
    println!("{}", planet_utils::generate_name());
    //TODO: make random tag selector
    //
    // print random atmosphere
    match get_random_entry("../config/planet/planet_atmosphere.json", "atmospheres") {
        Ok(res) => println!("Atmosphere  : {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
    // print random biosphere
    match get_random_entry("../config/planet/planet_biosphere.json", "biospheres") {
        Ok(res) => println!("Biosphere   : {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
    // print random population
    match get_random_entry("../config/planet/planet_population.json", "populations") {
        Ok(res) => println!("Population  : {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
    // print random tech level
    match get_random_entry("../config/planet/planet_tech.json", "technologies") {
        Ok(res) => println!("Tech Level  : {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
    // print random temperature
    match get_random_entry("../config/planet/planet_temperature.json", "temperatures") {
        Ok(res) => println!("Temperature : {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}
