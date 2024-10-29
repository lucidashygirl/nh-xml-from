use crate::*;

pub fn generate_dialogue_tree_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = match &toml.schema {
        Some(s) => s,
        None => &"https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/dialogue_schema.xsd".to_owned(),
    };
    xml += format!(
        r#"<DialogueTree xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        schema
    )
    .as_str();
    match &toml.name_field {
        Some(name) => xml += format!("<NameField>{}</NameField>", name).as_str(),
        None => quit!("Field name_field is required for DialogueTree"),
    }
    if let Some(dialogue_node) = &toml.dialogue_node {
        let mut default_condition = false;
        for block in dialogue_node {
            if let Some(entry_condition) = &block.entry_condition {
                if entry_condition.contains(&"DEFAULT".to_owned()) {
                    default_condition = true
                }
            }
        }
        if !default_condition {
            quit!("ERROR: No default condition in dialogue_node")
        }
        for block in dialogue_node {
            xml += "<DialogueNode>";
            xml += format!("<Name>{}</Name>", block.name).as_str();

            if let Some(conditions) = &block.entry_condition {
                for condition in conditions {
                    xml += format!("<EntryCondition>{}</EntryCondition>", condition).as_str()
                }
            }

            if let Some(random) = block.randomize {
                if random {
                    xml += "<Randomize />"
                }
            };

            if let Some(dialogue) = &block.dialogue {
                for mutter in dialogue {
                    xml += "<Dialogue>";
                    for p in &mutter.page {
                        xml += format!("<Page>{}</Page>", p).as_str()
                    }
                    xml += "</Dialogue>";
                }
            }

            if let Some(reveal_facts) = &block.reveal_facts {
                xml += "<RevealFacts>";
                for id in &reveal_facts.fact_id {
                    xml += format!("<FactID>{}</FactID>", id).as_str()
                }
                xml += "</RevealFacts>";
            }

            if let Some(condition) = &block.set_persistent_condition {
                xml += format!(
                    "<SetPersistentCondition>{}</SetPersistentCondition>",
                    condition
                )
                .as_str();
            }

            if let Some(set_condition) = &block.set_condition {
                for condition in set_condition {
                    xml += format!("<SetCondition>{}</SetCondition>", condition).as_str();
                }
            }

            if let Some(condition) = &block.disable_persistent_condition {
                xml += format!(
                    "<DisablePersistentCondition>{}</DisablePersistentCondition>",
                    condition
                )
                .as_str();
            }

            if let Some(conditions) = &block.dialogue_target_shiplog_condition {
                for condition in conditions {
                    xml += format!(
                        "<DialogueTargetShipLogCondition>{}</DialogueTargetShipLogCondition>",
                        condition
                    )
                    .as_str();
                }
            }

            if let Some(target) = &block.dialogue_target {
                xml += format!("<DialogueTarget>{}</DialogueTarget>", target).as_str();
            }

            if let Some(dialogue_options_list) = &block.dialogue_options_list {
                xml += "<DialogueOptionsList>";
                if let Some(dialogue_option) = &dialogue_options_list.dialogue_option {
                    for option in dialogue_option {
                        xml += "<DialogueOption>";
                        if let Some(required_log_condition) = &option.required_log_condition {
                            for condition in required_log_condition {
                                xml += format!(
                                    "<RequiredLogCondition>{}</RequiredLogCondition>",
                                    condition
                                )
                                .as_str()
                            }
                        }

                        if let Some(required_persistent_condition) =
                            &option.required_persistent_condition
                        {
                            for condition in required_persistent_condition {
                                xml += format!(
                                    "<RequiredPersistentCondition>{}</RequiredPersistentCondition>",
                                    condition
                                )
                                .as_str()
                            }
                        }

                        if let Some(cancelled_persistent_condition) =
                            &option.cancelled_persistent_condition
                        {
                            for condition in cancelled_persistent_condition {
                                xml += format!(
                                "<CancelledPersistentCondition>{}</CancelledPersistentCondition>",
                                condition
                            )
                                .as_str()
                            }
                        }

                        if let Some(required_condition) = &option.required_condition {
                            xml += format!(
                                "<RequiredCondition>{}</RequiredCondition>",
                                required_condition
                            )
                            .as_str()
                        }

                        if let Some(cancelled_condition) = &option.cancelled_condition {
                            xml += format!(
                                "<CancelledCondition>{}</CancelledCondition>",
                                cancelled_condition
                            )
                            .as_str()
                        }

                        // Required field, no check needed
                        xml += format!("<Text>{}</Text>", &option.text).as_str();

                        if let Some(dialogue_target) = &option.dialogue_target {
                            xml += format!("<DialogueTarget>{}</DialogueTarget>", dialogue_target)
                                .as_str()
                        }

                        if let Some(condition) = &option.condition_to_set {
                            xml +=
                                format!("<ConditionToSet>{}</ConditionToSet>", condition).as_str()
                        }

                        if let Some(condition) = &option.condition_to_cancel {
                            xml += format!("<ConditionToCancel>{}</ConditionToCancel>", condition)
                                .as_str()
                        }

                        xml += "</DialogueOption>"
                    }
                }
                if let Some(from) = &dialogue_options_list.reuse_dialogue_options_list_from {
                    xml += format!(
                        "<ReuseDialogueOptionsListFrom>{}</ReuseDialogueOptionsListFrom>",
                        from
                    )
                    .as_str()
                }
                xml += "</DialogueOptionsList>"
            }

            xml += "</DialogueNode>";
        }
    }
    xml += "</DialogueTree>";
    xml
}
