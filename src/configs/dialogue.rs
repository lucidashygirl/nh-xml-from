use crate::data::DialogueNode;
use crate::data::DialogueOption;
use crate::ConfigFile;

const DEFAULT_SCHEMA: &str = "https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/dialogue_schema.xsd";

pub fn generate_dialogue_tree_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = toml.schema.as_ref().map_or(DEFAULT_SCHEMA, |s| s);
    xml += format!(
        r#"<DialogueTree xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{schema}">"#
    )
    .as_str();
    if let Some(name) = &toml.name_field {
        xml += format!("<NameField>{name}</NameField>").as_str();
    } else {
        quit!("Field name_field is required for DialogueTree")
    }
    if let Some(dialogue_node) = &toml.dialogue_node {
        let mut default_condition = false;
        for block in dialogue_node {
            if let Some(entry_condition) = &block.entry_condition {
                if entry_condition.contains(&"DEFAULT".to_owned()) {
                    default_condition = true;
                }
            }
        }
        if !default_condition {
            quit!("ERROR: No default condition in dialogue_node")
        }
        for block in dialogue_node {
            xml += for_block_in_dialogue_node(block).as_str();
        }
    }
    xml += "</DialogueTree>";
    xml
}

fn for_block_in_dialogue_node(block: &DialogueNode) -> String {
    let mut xml = String::new();
    xml += "<DialogueNode>";
    xml += format!("<Name>{}</Name>", block.name).as_str();

    if let Some(conditions) = &block.entry_condition {
        for condition in conditions {
            xml += format!("<EntryCondition>{condition}</EntryCondition>").as_str();
        }
    }

    if let Some(random) = block.randomize {
        if random {
            xml += "<Randomize />";
        }
    };

    if let Some(dialogue) = &block.dialogue {
        for mutter in dialogue {
            xml += "<Dialogue>";
            for p in &mutter.page {
                xml += format!("<Page>{p}</Page>").as_str();
            }
            xml += "</Dialogue>";
        }
    }

    if let Some(reveal_facts) = &block.reveal_facts {
        xml += "<RevealFacts>";
        for id in &reveal_facts.fact_id {
            xml += format!("<FactID>{id}</FactID>").as_str();
        }
        xml += "</RevealFacts>";
    }

    if let Some(condition) = &block.set_persistent_condition {
        xml += format!("<SetPersistentCondition>{condition}</SetPersistentCondition>").as_str();
    }

    if let Some(set_condition) = &block.set_condition {
        for condition in set_condition {
            xml += format!("<SetCondition>{condition}</SetCondition>").as_str();
        }
    }

    if let Some(condition) = &block.disable_persistent_condition {
        xml += format!("<DisablePersistentCondition>{condition}</DisablePersistentCondition>")
            .as_str();
    }

    if let Some(conditions) = &block.dialogue_target_shiplog_condition {
        for condition in conditions {
            xml += format!(
                "<DialogueTargetShipLogCondition>{condition}</DialogueTargetShipLogCondition>"
            )
            .as_str();
        }
    }

    if let Some(target) = &block.dialogue_target {
        xml += format!("<DialogueTarget>{target}</DialogueTarget>").as_str();
    }

    if let Some(dialogue_options_list) = &block.dialogue_options_list {
        xml += "<DialogueOptionsList>";
        if let Some(dialogue_option) = &dialogue_options_list.dialogue_option {
            for option in dialogue_option {
                xml += for_option_in_dialogue_option(option).as_str();
            }
        }
        if let Some(from) = &dialogue_options_list.reuse_dialogue_options_list_from {
            xml += format!("<ReuseDialogueOptionsListFrom>{from}</ReuseDialogueOptionsListFrom>")
                .as_str();
        }
        xml += "</DialogueOptionsList>";
    }

    xml += "</DialogueNode>";
    xml
}

fn for_option_in_dialogue_option(option: &DialogueOption) -> String {
    let mut xml = String::new();
    xml += "<DialogueOption>";
    if let Some(required_log_condition) = &option.required_log_condition {
        for condition in required_log_condition {
            xml += format!("<RequiredLogCondition>{condition}</RequiredLogCondition>").as_str();
        }
    }

    if let Some(required_persistent_condition) = &option.required_persistent_condition {
        for condition in required_persistent_condition {
            xml +=
                format!("<RequiredPersistentCondition>{condition}</RequiredPersistentCondition>")
                    .as_str();
        }
    }

    if let Some(cancelled_persistent_condition) = &option.cancelled_persistent_condition {
        for condition in cancelled_persistent_condition {
            xml +=
                format!("<CancelledPersistentCondition>{condition}</CancelledPersistentCondition>")
                    .as_str();
        }
    }

    if let Some(required_condition) = &option.required_condition {
        xml += format!("<RequiredCondition>{required_condition}</RequiredCondition>").as_str();
    }

    if let Some(cancelled_condition) = &option.cancelled_condition {
        xml += format!("<CancelledCondition>{cancelled_condition}</CancelledCondition>").as_str();
    }

    // Required field, no check needed
    xml += format!("<Text>{}</Text>", &option.text).as_str();

    if let Some(dialogue_target) = &option.dialogue_target {
        xml += format!("<DialogueTarget>{dialogue_target}</DialogueTarget>").as_str();
    }

    if let Some(condition) = &option.condition_to_set {
        xml += format!("<ConditionToSet>{condition}</ConditionToSet>").as_str();
    }

    if let Some(condition) = &option.condition_to_cancel {
        xml += format!("<ConditionToCancel>{condition}</ConditionToCancel>").as_str();
    }

    xml += "</DialogueOption>";
    xml
}
