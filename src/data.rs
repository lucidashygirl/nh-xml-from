use serde::Deserialize;
use toml::Table;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub schema: String,
    pub file_type: String,
    // NomaiObject
    pub text_block: Option<Vec<Table>>,
    pub log_condition: Option<Vec<Table>>,
    // AstralObject
    pub id: Option<String>,
    pub entry: Option<Vec<Table>>,
    // DialogueTree
    pub name_field: Option<Table>,
    pub dialogue_node: Option<Table>,
}

#[derive(Deserialize, Default, Debug)]
pub struct NomaiTextBlock {
    pub id: u16,
    pub parent: Option<u16>,
    pub text: String,
    pub location: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug)]
pub struct DialogueTree {
    pub name: String,
    pub node: Vec<DialogueNode>,
}

#[derive(Deserialize, Default, Debug)]
pub struct DialogueNode {
    pub name: String,
    pub randomize: Option<bool>,
    pub entry_condition: Option<String>,
    pub dialogue: Option<Vec<Dialogue>>,
    pub reveal_fact: Option<Vec<Fact>>,
    pub set_persistent_condition: Option<String>,
    pub set_condition: Option<Vec<String>>,
    pub dialogue_target_shiplog_condition: Option<Vec<String>>,
    pub dialogue_target: Option<String>,
    pub dialogue_options_list: Option<DialogueOptionsList>,
}

#[derive(Deserialize, Default, Debug)]
pub struct DialogueOptionsList {
    pub dialogue_options: Option<Vec<DialogueOption>>,
    pub reuse_dialogue_options_list_from: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct DialogueOption {
    pub text: String,
    pub required_log_condition: Option<Vec<String>>,
    pub required_persistent_condition: Option<Vec<String>>,
    pub cancelled_persistent_condition: Option<Vec<String>>,
    pub required_condition: Option<String>,
    pub cancelled_condition: Option<String>,
    pub dialogue_target: Option<String>,
    pub condition_to_set: Option<String>,
    pub condition_to_cancel: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Dialogue {
    pub page: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct AstroObjectEntry {
    pub id: String,
    pub entry: Option<Vec<Entry>>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub curiosity: Option<String>,
    pub is_curiosity: Option<bool>,
    pub ignore_more_to_explore: Option<bool>,
    pub parent_ignore_not_revealed: Option<bool>,
    pub ignore_more_to_explore_condition: Option<String>,
    pub alt_photo_condition: Option<String>,
    pub rumor_fact: Option<Vec<RumorFact>>,
    pub explore_fact: Option<Vec<ExploreFact>>,
    pub entry: Option<Vec<Entry>>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Fact {
    pub id: String,
    pub condition: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct RumorFact {
    pub id: String,
    pub source_id: Option<String>,
    pub rumor_name: Option<String>,
    pub rumor_name_priority: Option<i64>,
    pub ignore_more_to_explore: Option<bool>,
}
#[derive(Deserialize, Default, Debug)]
pub struct ExploreFact {
    pub id: String,
    pub ignore_more_to_explore: Option<bool>,
}
#[derive(Deserialize, Debug, Default)]
pub struct Conditions {
    pub reveal_fact: Vec<Fact>,
    pub location: Option<Vec<String>>,
}