pub mod astro_object;
pub mod dialogue;
pub mod text_block;

use crate::{AstroObjectEntry, ConfigFile};

use {
    astro_object::{generate_astro_object_config, generate_astro_object_xml_string},
    dialogue::generate_dialogue_tree_xml_string,
    text_block::generate_nomai_text_xml_string,
};

pub fn create_xml(config: &ConfigFile) -> String {
    match config.file_type.as_str() {
        "NomaiObject" => generate_nomai_text_xml_string(config),
        "DialogueTree" => generate_dialogue_tree_xml_string(config),
        "AstroObjectEntry" => generate_astro_object_xml_string(config),
        _ => quit!("No matching file type"),
    }
}

pub fn config_from_toml(cfg_str: &str) -> ConfigFile {
    let toml = toml::from_str(cfg_str);
    match toml {
        Ok(t) => t,
        Err(e) => quit!(format!("{}", e)),
    }
}

pub fn config_from_json(cfg_str: &str) -> ConfigFile {
    let json = serde_json::from_str(cfg_str);
    match json {
        Ok(j) => j,
        Err(e) => quit!(format!("{}", e)),
    }
}

pub fn config_from_yaml(cfg_str: &str) -> ConfigFile {
    let yaml = serde_yml::from_str(cfg_str);
    match yaml {
        Ok(y) => y,
        Err(e) => quit!(format!("{}", e)),
    }
}

pub fn config_from_ron(cfg_str: &str) -> ConfigFile {
    let ron = ron::from_str(cfg_str);
    match ron {
        Ok(r) => r,
        Err(e) => quit!(format!("{}", e)),
    }
}

pub fn astro_config_from_xml(cfg_str: &str) -> AstroObjectEntry {
    let xml = quick_xml::de::from_str(cfg_str);
    match xml {
        Ok(t) => t,
        Err(e) => quit!(format!("{}", e)),
    }
}
