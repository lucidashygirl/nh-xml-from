use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ConfigFile {
    pub schema: Option<String>,
    pub file_type: String,
    // NomaiObject
    pub text_block: Option<Vec<NomaiTextBlock>>,
    pub log_condition: Option<Vec<Conditions>>,
    // AstroObject
    pub id: Option<String>,
    pub entry: Option<Vec<Entry>>,
    // DialogueTree
    pub name_field: Option<String>,
    pub dialogue_node: Option<Vec<DialogueNode>>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NomaiTextBlock {
    pub id: i64,
    pub parent: Option<i64>,
    pub text: String,
    pub location: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueNode {
    pub name: String,
    pub randomize: Option<bool>,
    pub entry_condition: Option<Vec<String>>,
    pub dialogue: Option<Vec<Dialogue>>,
    pub reveal_facts: Option<RevealFacts>,
    pub set_persistent_condition: Option<String>,
    pub set_condition: Option<Vec<String>>,
    pub disable_persistent_condition: Option<String>,
    pub dialogue_target_shiplog_condition: Option<Vec<String>>,
    pub dialogue_target: Option<String>,
    pub dialogue_options_list: Option<DialogueOptionsList>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueOptionsList {
    pub dialogue_option: Option<Vec<DialogueOption>>,
    pub reuse_dialogue_options_list_from: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Dialogue {
    pub page: Vec<String>,
}

#[allow(clippy::struct_field_names)]
#[derive(Deserialize, Serialize, Default, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Fact {
    pub id: String,
    pub condition: Vec<i64>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct RumorFact {
    pub id: String,
    pub text: String,
    pub source_id: Option<String>,
    pub rumor_name: Option<String>,
    pub rumor_name_priority: Option<i64>,
    pub ignore_more_to_explore: Option<bool>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ExploreFact {
    pub id: String,
    pub text: String,
    pub ignore_more_to_explore: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Conditions {
    pub reveal_fact: Vec<Fact>,
    pub location: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RevealFacts {
    pub fact_id: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NomaiObject;
#[derive(Deserialize, Serialize, Debug)]
pub struct AstroObjectEntry {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Entry")]
    pub entry: Option<Vec<EntryXml>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DialogueTree;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ConfigFileXml {
    #[serde(rename = "NomaiObject")]
    pub nomai_object: Option<NomaiObject>,
    #[serde(rename = "AstroObjectEntry")]
    pub astro_object: Option<AstroObjectEntry>,
    #[serde(rename = "DialogueTree")]
    pub dialogue_tree: Option<DialogueTree>,
    // NomaiObject
    #[serde(rename = "TextBlock")]
    pub text_block: Option<Vec<NomaiTextBlockXml>>,
    #[serde(rename = "ShipLogConditions")]
    pub log_condition: Option<Vec<ConditionsXml>>,
    // AstralObject
    // DialogueTree
    #[serde(rename = "NameField")]
    pub name_field: Option<String>,
    #[serde(rename = "DialogueNode")]
    pub dialogue_node: Option<Vec<DialogueNodeXml>>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct NomaiTextBlockXml {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "ParentID")]
    pub parent: Option<i64>,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "LocationA")]
    pub location_a: Option<LocationA>,
    #[serde(rename = "LocationB")]
    pub location_b: Option<LocationB>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationA;

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationB;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueNodeXml {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Randomize")]
    pub randomize: Option<bool>,
    #[serde(rename = "EntryCondition")]
    pub entry_condition: Option<Vec<String>>,
    #[serde(rename = "Dialogue")]
    pub dialogue: Option<Vec<DialogueXml>>,
    #[serde(rename = "RevealFacts")]
    pub reveal_facts: Option<RevealFactsXml>,
    #[serde(rename = "SetPersistentCondition")]
    pub set_persistent_condition: Option<String>,
    #[serde(rename = "SetCondition")]
    pub set_condition: Option<Vec<String>>,
    #[serde(rename = "DisablePersistentCondition")]
    pub disable_persistent_condition: Option<String>,
    #[serde(rename = "DialogueTargetShiplogCondition")]
    pub dialogue_target_shiplog_condition: Option<Vec<String>>,
    #[serde(rename = "DialogueTarget")]
    pub dialogue_target: Option<String>,
    #[serde(rename = "DialogueOptionsList")]
    pub dialogue_options_list: Option<DialogueOptionsListXml>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueOptionsListXml {
    #[serde(rename = "DialogueOption")]
    pub dialogue_option: Option<Vec<DialogueOptionXml>>,
    #[serde(rename = "ReuseDialogueOptionsListFrom")]
    pub reuse_dialogue_options_list_from: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueOptionXml {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "RequiredLogCondition")]
    pub required_log_condition: Option<Vec<String>>,
    #[serde(rename = "RequiredPersistentCondition")]
    pub required_persistent_condition: Option<Vec<String>>,
    #[serde(rename = "CancelledPersistentCondition")]
    pub cancelled_persistent_condition: Option<Vec<String>>,
    #[serde(rename = "RequiredCondition")]
    pub required_condition: Option<String>,
    #[serde(rename = "CancelledCondition")]
    pub cancelled_condition: Option<String>,
    #[serde(rename = "DialogueTarget")]
    pub dialogue_target: Option<String>,
    #[serde(rename = "ConditionToSet")]
    pub condition_to_set: Option<String>,
    #[serde(rename = "ConditionToCancel")]
    pub condition_to_cancel: Option<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DialogueXml {
    #[serde(rename = "Page")]
    pub page: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct IsCuriosity;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct IgnoreMoreToExplore;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ParentIgnoreNotRevealed;

#[allow(clippy::struct_field_names)]
#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct EntryXml {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Curiosity")]
    pub curiosity: Option<String>,
    #[serde(rename = "IsCuriosity")]
    pub is_curiosity: Option<IsCuriosity>,
    #[serde(rename = "IgnoreMoreToExplore")]
    pub ignore_more_to_explore: Option<IgnoreMoreToExplore>,
    #[serde(rename = "ParentIgnoreNotRevealed")]
    pub parent_ignore_not_revealed: Option<ParentIgnoreNotRevealed>,
    #[serde(rename = "IgnoreMoreToExploreCondition")]
    pub ignore_more_to_explore_condition: Option<String>,
    #[serde(rename = "AltPhotoCondition")]
    pub alt_photo_condition: Option<String>,
    #[serde(rename = "RumorFact")]
    pub rumor_fact: Option<Vec<RumorFactXml>>,
    #[serde(rename = "ExploreFact")]
    pub explore_fact: Option<Vec<ExploreFactXml>>,
    #[serde(rename = "Entry")]
    pub entry: Option<Vec<EntryXml>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct FactXml {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Condition")]
    pub condition: Vec<i64>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct RumorFactXml {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "SourceID")]
    pub source_id: Option<String>,
    #[serde(rename = "RumorName")]
    pub rumor_name: Option<String>,
    #[serde(rename = "RumorNamePriority")]
    pub rumor_name_priority: Option<i64>,
    #[serde(rename = "IgnoreMoreToExplore")]
    pub ignore_more_to_explore: Option<IgnoreMoreToExplore>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ExploreFactXml {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "IgnoreMoreToExplore")]
    pub ignore_more_to_explore: Option<IgnoreMoreToExplore>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConditionsXml {
    #[serde(rename = "RevealFact")]
    pub reveal_fact: Vec<FactXml>,
    #[serde(rename = "LocationA")]
    pub location_a: Option<LocationA>,
    #[serde(rename = "LocationB")]
    pub location_b: Option<LocationB>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RevealFactsXml {
    #[serde(rename = "FactID")]
    pub fact_id: Vec<String>,
}
