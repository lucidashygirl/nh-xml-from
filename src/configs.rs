pub mod astro_object;
pub mod dialogue;
pub mod text_block;

use crate::data::ConfigFile;
use {
    astro_object::validate_astral_object_config, dialogue::*,
    text_block::validate_nomai_text_config,
};

pub fn validate_config(config: &ConfigFile) -> String {
    match config.file_type.as_str() {
        "NomaiObject" => validate_nomai_text_config(config),
        //"DialogueTree" => validate_dialogue_tree_config(config),
        "AstroObjectEntry" => validate_astral_object_config(config),
        _ => quit!("No matching file type"),
    }
}
