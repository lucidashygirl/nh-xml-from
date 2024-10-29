pub mod astro_object;
pub mod dialogue;
pub mod text_block;

use crate::data::ConfigFile;
use {
    astro_object::generate_astro_object_xml_string, dialogue::generate_dialogue_tree_xml_string,
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
