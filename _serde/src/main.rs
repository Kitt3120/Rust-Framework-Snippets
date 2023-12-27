/*
    Serde (SErialize and DEserialize) can be used to easily convert Rust data structures into
    different formats, like JSON, and vice versa.
    To make a struct serializable, add the Serialize trait to it, by either deriving it or implementing it manually.

    Make sure to add serde with the derive feature to your Cargo.toml file:
    cargo add serde --features derive

    And, for json support:
    cargo add serde_json

    Supported formats:
    https://serde.rs/#data-formats
*/

use std::{fs::File, path::Path, time};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    some_setting: String,
    color: Color,
    enabled_features: Vec<String>,
}

fn main() {
    let now = time::Instant::now();

    let path = Path::new("./config.json");
    let exists = path.exists();

    let file = match exists {
        true => match File::open(path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the config file: {:?}", error),
        },
        false => match File::create(path) {
            Ok(file) => file,
            Err(error) => panic!("Problem creating the config file: {:?}", error),
        },
    };

    let config = match exists {
        true => load_config(&file),
        false => generate_default_config(&file),
    };

    println!("Current config:\n{:#?}", config);
    println!("Elapsed time: {:.2?}", now.elapsed());
}

fn load_config(file: &File) -> Config {
    match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(error) => panic!("Problem parsing the file: {:?}", error),
    }
}

fn generate_default_config(file: &File) -> Config {
    let config = Config {
        some_setting: String::from("some value"),
        color: Color::RgbColor(0, 0, 0),
        enabled_features: vec![String::from("feature1"), String::from("feature2")],
    };

    match serde_json::to_writer_pretty(file, &config) {
        Ok(_) => (),
        Err(error) => panic!("Problem writing the file: {:?}", error),
    }

    config
}
