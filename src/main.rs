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

use configs::astro_object::generate_astro_object_config;
use configs::dialogue::generate_dialogue_config;
use configs::text_block::generate_nomai_object_config;
use {
    configs::{
        astro_config_from_xml, config_from_json, config_from_ron, config_from_toml,
        config_from_yaml, create_xml, dialogue_config_from_xml, nomai_config_from_xml,
    },
    data::{
        AstroObjectEntry, Conditions, ConditionsXml, ConfigFile, DialogueTree, Entry, EntryXml,
        ExploreFact, ExploreFactXml, Fact, NomaiObject, NomaiTextBlock, NomaiTextBlockXml,
        RumorFact, RumorFactXml, SchemaFormat,
    },
    files::{create_xml_byte_vector, get_file_extension, get_file_name, get_input_format},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0..=1 => quit!("No arguments provided"),
        2..=3 => (),
        _ => quit!("Too many arguments provided"),
    }

    let file_path = &args[1];

    let output_format = {
        if args.len() == 3 {
            &args[2]
        } else {
            "xml"
        }
    };
    let Ok(mut config) = File::open(file_path) else {
        quit!("File doesn't exist")
    };
    let file_name = get_file_name(file_path);
    let extension = get_file_extension(&file_name);
    match extension.as_str() {
        "xml" | "toml" | "json" | "yaml" | "yml" | "ron" => (),
        _ => quit!("Incorrect Format!"),
    }
    let name = file_name.replace(&extension, "");
    let mut contents: String = String::new();
    let _ = config
        .read_to_string(&mut contents)
        .unwrap_or_else(|_| quit!("Failed conversion to string."));

    let result = if extension.as_str() != "xml" {
        let parsed_config: ConfigFile = match extension.as_str() {
            "toml" => config_from_toml(&contents),
            "json" => config_from_json(&contents),
            "yaml" | "yml" => config_from_yaml(&contents),
            "ron" => config_from_ron(&contents),
            _ => quit!("previous check for extension failed, please report"),
        };
        let xml = create_xml(&parsed_config);
        create_xml_byte_vector(xml.as_str())
    } else {
        let parsed_config = match get_input_format(&contents) {
            SchemaFormat::AstroObjectEntry => {
                generate_astro_object_config(astro_config_from_xml(&contents))
            }
            SchemaFormat::NomaiObject => {
                generate_nomai_object_config(nomai_config_from_xml(&contents))
            }
            SchemaFormat::DialogueTree => {
                generate_dialogue_config(dialogue_config_from_xml(&contents))
            }
        };
        match output_format {
            "toml" => toml_string(parsed_config),
            "json" => json_string(parsed_config),
            "ron" => ron_string(parsed_config),
            "yaml" | "yml" => yaml_string(parsed_config),
            _ => quit!("Impossible case, or it should be at least..."),
        }
        .as_bytes()
        .to_vec()
    };
    let mut file = match File::create(format!("{name}{output_format}")) {
        Ok(f) => f,
        Err(err) => quit!(format!("Failed to create file:\n{}", err)),
    };
    match file.write_all(&result) {
        Ok(()) => quit!(),
        Err(err) => quit!(format!("Failed to write to file:\n{}", err)),
    }
}

fn toml_string(cfg: ConfigFile) -> String {
    let result = toml::to_string_pretty(&cfg);
    match result {
        Ok(r) => r,
        Err(e) => quit!(format!("{}", e)),
    }
}

fn json_string(cfg: ConfigFile) -> String {
    let result = serde_json::to_string_pretty(&cfg);
    match result {
        Ok(r) => r,
        Err(e) => quit!(format!("{}", e)),
    }
}

fn ron_string(cfg: ConfigFile) -> String {
    let result = ron::ser::to_string_pretty(&cfg, ron::ser::PrettyConfig::default());
    match result {
        Ok(r) => r,
        Err(e) => quit!(format!("{}", e)),
    }
}

fn yaml_string(cfg: ConfigFile) -> String {
    let result = serde_yml::ser::to_string(&cfg);
    match result {
        Ok(r) => r,
        Err(e) => quit!(format!("{}", e)),
    }
}
