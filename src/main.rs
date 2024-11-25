use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    reader::Reader,
    writer::Writer,
};
use std::{
    env,
    fs::File,
    io::{Cursor, Read, Write},
};

#[macro_use]
mod util;

mod configs;
mod data;
mod files;

use {
    configs::{config_from_json, config_from_ron, config_from_toml, config_from_yaml, create_xml},
    data::{Conditions, ConfigFile, Entry, FileExtention, NomaiTextBlock},
    files::{create_xml_byte_vector, get_file_extention, get_file_name},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0..=1 => quit!("No arguments provided"),
        2 => (),
        _ => quit!("Too many arguments provided"),
    }

    let file_path = &args[1];
    let Ok(mut config) = File::open(file_path) else {
        quit!("File doesn't exist")
    };
    let file_name = get_file_name(file_path);
    let extention = get_file_extention(&file_name);
    let extention_enum = match extention.as_str() {
        "toml" => FileExtention::Toml,
        "json" => FileExtention::Json,
        "yaml" | "yml" => FileExtention::Yaml,
        "ron" => FileExtention::Ron,
        _ => quit!("Incorrect Format!"),
    };
    let name = file_name.replace(&extention, "");

    let mut contents: String = String::new();
    if let Ok(c) = config.read_to_string(&mut contents) {
        c
    } else {
        quit!("Couldn't convert to a string.")
    };

    let parsed_config: ConfigFile = match extention_enum {
        FileExtention::Toml => config_from_toml(&contents),
        FileExtention::Json => config_from_json(&contents),
        FileExtention::Yaml => config_from_yaml(&contents),
        FileExtention::Ron => config_from_ron(&contents),
    };

    let xml = create_xml(&parsed_config);

    let result = create_xml_byte_vector(xml.as_str());

    let mut file = match File::create(format!("{name}xml")) {
        Ok(f) => f,
        Err(err) => quit!(format!("Could not create file:\n{}", err)),
    };

    match file.write_all(&result) {
        Ok(()) => (),
        Err(err) => quit!(format!("Failed to write to file:\n{}", err)),
    }
}
