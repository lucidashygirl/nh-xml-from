use quick_xml::{events::*, reader::*, writer::*};
use std::{env, fs::*, io::*};

#[macro_use]
mod util;
mod configs;
mod data;
mod files;

use {configs::*, data::*, files::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => quit!("No arguments provided"),
        2 => (),
        _ => quit!("Too many arguments provided"),
    }

    let file_path = &args[1];
    let mut config = match File::open(file_path) {
        Ok(c) => c,
        Err(_) => quit!("File doesn't exist"),
    };

    let file_name = get_file_name(file_path);
    let extention = get_file_extention(&file_name);
    let name = file_name.replace(&extention, "");
    if extention.as_str() != "toml" {
        quit!("Incorrect Format!");
    }

    let mut contents: String = String::new();
    match config.read_to_string(&mut contents) {
        Ok(c) => c,
        Err(_) => quit!("Couldn't convert to a string."),
    };

    let toml_config: ConfigFile = match toml::from_str(contents.as_str()) {
        Ok(c) => c,
        Err(err) => quit!(format!("{}", err)),
    };

    let xml = validate_config(&toml_config);

    let result = create_xml_byte_vector(xml.as_str());

    let mut file = match File::create(format!("{}xml", name)) {
        Ok(f) => f,
        Err(err) => quit!(format!("Could not create file:\n{}", err)),
    };

    match file.write_all(&result) {
        Ok(_) => (),
        Err(err) => quit!(format!("Failed to write to file:\n{}", err)),
    }
}
