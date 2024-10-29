use toml::map::Map;
use toml::value::Value;

use crate::*;

pub fn validate_astral_object_config(config: &ConfigFile) -> String {
    let entries: Vec<Entry> = Vec::new();
    let id = match &config.id {
        Some(id) => id.clone(),
        None => quit!("Invalid ID"),
    };
    let entries = validate_entry_config(config, &entries);
    let entry = Some(entries);
    let astro_object = AstroObjectEntry { id, entry };
    generate_astro_object_xml_string(config, astro_object)
}

pub fn validate_entry_config(config: &ConfigFile, entries: &Vec<Entry>) -> Vec<Entry> {
    match &config.entry {
        Some(blocks) => match for_entry_config(blocks, entries) {
            Some(config) => config,
            None => quit!("Invalid entry config.. apparently.."),
        },
        None => quit!("No entries found"),
    }
}

pub fn for_entry_config(
    blocks: &Vec<Map<String, Value>>,
    entry_vec: &Vec<Entry>,
) -> Option<Vec<Entry>> {
    let mut return_vec = entry_vec.clone();
    for block in blocks {
        let mut entry_block = Entry::default();
        match block.get("id") {
            Some(id) => {
                entry_block.id = match id.as_str() {
                    Some(id) => id.to_owned(),
                    None => quit!("Invalid type for id, expected String"),
                }
            }
            None => quit!("ID required"),
        }
        match block.get("name") {
            Some(name) => {
                entry_block.name = match name.as_str() {
                    Some(name) => name.to_owned(),
                    None => quit!("Invalid type for name, expected String"),
                }
            }
            None => quit!("Name required"),
        }
        if let Some(curiosity) = block.get("curiosity") {
            entry_block.curiosity = match curiosity.as_str() {
                Some(c) => Some(c.to_owned()),
                None => quit!(format!("Invalid type for curiosity, expected String")),
            }
        }
        if let Some(is_curiosity) = block.get("is_curiosity") {
            entry_block.is_curiosity = match is_curiosity.as_bool() {
                Some(b) => Some(b),
                None => {
                    quit!("Invalid type for is_curiosity, expected Boolean")
                }
            };
        }
        if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
            entry_block.ignore_more_to_explore = match ignore_more_to_explore.as_bool() {
                Some(b) => Some(b),
                None => {
                    quit!("Invalid type for ignore_more_to_explore, expected Boolean")
                }
            };
        }
        if let Some(parent_ignore_not_revealed) = block.get("parent_ignore_not_revealed") {
            entry_block.parent_ignore_not_revealed = match parent_ignore_not_revealed.as_bool() {
                Some(b) => Some(b),
                None => {
                    quit!("Invalid type for parent_ignore_not_revealed, expected Boolean")
                }
            };
        }
        if let Some(ignore_more_to_explore_condition) =
            block.get("ignore_more_to_explore_condition")
        {
            entry_block.ignore_more_to_explore_condition =
                match ignore_more_to_explore_condition.as_str() {
                    Some(i) => Some(i.to_owned()),
                    None => {
                        quit!("Invalid type for ignore_more_to_explore_condition, expected String.")
                    }
                };
        }
        if let Some(alt_photo_condition) = block.get("alt_photo_condition") {
            entry_block.alt_photo_condition = match alt_photo_condition.as_str() {
                Some(a) => Some(a.to_owned()),
                None => quit!("Invalid type for alt_photo_condition, expected String"),
            }
        }
        if let Some(rumor_facts) = block.get("rumor_fact") {
            let mut facts: Vec<RumorFact> = Vec::new();
            #[allow(for_loops_over_fallibles)]
            for rumor_fact in rumor_facts.as_array() {
                for fact in rumor_fact {
                    let mut rumor = RumorFact::default();
                    match fact.get("id") {
                        Some(id) => {
                            rumor.id = match id.as_str() {
                                Some(i) => i.to_owned(),
                                None => quit!("Invalid type for rumor_fact.id, expected String."),
                            }
                        }
                        None => quit!("ID required"),
                    }
                    match fact.get("text") {
                        Some(text) => {
                            rumor.text = match text.as_str() {
                                Some(t) => t.to_owned(),
                                None => quit!("Invalid type for rumor_fact.text, expected String."),
                            }
                        }
                        None => quit!("Text required"),
                    }
                    if let Some(source_id) = fact.get("source_id") {
                        rumor.source_id = match source_id.as_str() {
                            Some(s) => Some(s.to_owned()),
                            None => quit!("Invalid type for rumor_fact.source_id, expected String"),
                        }
                    }
                    if let Some(rumor_name) = fact.get("rumor_name") {
                        rumor.rumor_name = match rumor_name.as_str() {
                            Some(r) => Some(r.to_owned()),
                            None => {
                                quit!("Invalid type for rumor_fact.rumor_name, expected String")
                            }
                        }
                    }
                    if let Some(rumor_name_priority) = fact.get("rumor_name_priority") {
                        rumor.rumor_name_priority = match rumor_name_priority.as_integer() {
                            Some(i) => Some(i),
                            None => quit!(
                                "Invalid type for rumor_fact.rumor_name_priority, expected Integer"
                            ),
                        };
                    }
                    if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
                        entry_block.ignore_more_to_explore = match ignore_more_to_explore.as_bool()
                        {
                            Some(b) => Some(b),
                            None => {
                                quit!("Invalid type for rumor_fact.ignore_more_to_explore, expected Boolean")
                            }
                        };
                    }
                    facts.push(rumor);
                }
            }
            entry_block.rumor_fact = Some(facts);
        }
        if let Some(explore_facts) = block.get("explore_fact") {
            let mut facts: Vec<ExploreFact> = Vec::new();
            #[allow(for_loops_over_fallibles)]
            for explore_fact in explore_facts.as_array() {
                for fact in explore_fact {
                    let mut explore = ExploreFact::default();
                    match fact.get("id") {
                        Some(id) => {
                            explore.id = match id.as_str() {
                                Some(i) => i.to_owned(),
                                None => quit!("Invalid type for explore_fact.id, expected String"),
                            }
                        }
                        None => quit!("ID required"),
                    }
                    match fact.get("text") {
                        Some(text) => {
                            explore.text = match text.as_str() {
                                Some(t) => t.to_owned(),
                                None => {
                                    quit!("Invalid type for explore_fact.text, expected String")
                                }
                            }
                        }
                        None => quit!("Text required"),
                    }
                    if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
                        entry_block.ignore_more_to_explore = match ignore_more_to_explore.as_bool() {
                            Some(b) => Some(b),
                            None => quit!("Invalid type for explore_fact.ignore_more_to_explore, expected Boolean"),
                        };
                    }
                    facts.push(explore);
                }
            }
            entry_block.explore_fact = Some(facts);
        }
        if let Some(e) = block.get("entry") {
            let mut local_entries: Vec<Map<String, Value>> = Vec::new();
            #[allow(for_loops_over_fallibles)]
            for thing in e.as_array() {
                for t in thing {
                    let entry = match t.as_table() {
                        Some(e) => e.clone(),
                        None => {
                            quit!("Idk what to put here lol, something is wrong with your entries")
                        }
                    };
                    local_entries.push(entry)
                }
            }
            entry_block.entry = for_entry_config(&local_entries, entry_vec);
        }
        return_vec.push(entry_block);
    }
    Some(return_vec)
}

pub fn generate_astro_object_xml_string(toml: &ConfigFile, blocks: AstroObjectEntry) -> String {
    let mut xml = String::new();
    let schema = match &toml.schema {
        Some(s) => s,
        None => &"https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/shiplog_schema.xsd".to_owned(),
    };
    xml += format!(
        r#"<AstroObjectEntry xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        schema
    )
    .as_str();
    xml += format!("<ID>{}</ID>", blocks.id).as_str();
    if let Some(entries) = blocks.entry {
        xml += entry_convert_xml(entries).as_str();
    }
    xml += format!("</{}>", toml.file_type).as_str();
    xml
}

pub fn entry_convert_xml(entries: Vec<Entry>) -> String {
    let mut xml = String::new();

    for entry in entries {
        xml += "<Entry>";
        xml += format!("<ID>{}</ID>", entry.id).as_str();
        xml += format!("<Name>{}</Name>", entry.name).as_str();
        if let Some(curiosity) = entry.curiosity {
            xml += format!("<Curiosity>{}</Curiosity>", curiosity).as_str();
        }
        if let Some(is_curiosity) = entry.is_curiosity {
            if is_curiosity {
                xml += "<IsCuriosity />";
            }
        }
        if let Some(ignore_more_to_explore) = entry.ignore_more_to_explore {
            if ignore_more_to_explore {
                xml += "<IgnoreMoreToExplore />";
            }
        }
        if let Some(parent_ignore_not_revealed) = entry.parent_ignore_not_revealed {
            if parent_ignore_not_revealed {
                xml += "<ParentIgnoreNotRevealed />";
            }
        }
        if let Some(ignore_more_to_explore_condition) = entry.ignore_more_to_explore_condition {
            xml += format!(
                "<IgnoreMoreToExploreCondition>{}</IgnoreMoreToExploreCondition>",
                ignore_more_to_explore_condition
            )
            .as_str();
        }
        if let Some(alt_photo_condition) = entry.alt_photo_condition {
            xml += format!(
                "<AltPhotoCondition>{}</AltPhotoCondition>",
                alt_photo_condition
            )
            .as_str();
        }
        if let Some(rumor_fact) = entry.rumor_fact {
            for rumor in rumor_fact {
                xml += "<RumorFact>";
                xml += format!("<ID>{}</ID>", rumor.id).as_str();
                if let Some(source_id) = rumor.source_id {
                    xml += format!("<SourceID>{}</SourceID>", source_id).as_str();
                }
                if let Some(rumor_name) = rumor.rumor_name {
                    xml += format!("<RumorName>{}</RumorName>", rumor_name).as_str();
                }
                xml += format!("<Text>{}</Text>", rumor.text).as_str();
                if let Some(rumor_name_priority) = rumor.rumor_name_priority {
                    xml += format!(
                        "<RumorNamePriority>{}</RumorNamePriority>",
                        rumor_name_priority
                    )
                    .as_str();
                }
                if let Some(ignore_more_to_explore) = rumor.ignore_more_to_explore {
                    if ignore_more_to_explore {
                        xml += "<IgnoreMoreToExplore />";
                    }
                }
                xml += "</RumorFact>";
            }
        }
        if let Some(explore_fact) = entry.explore_fact {
            for fact in explore_fact {
                xml += "<ExploreFact>";
                xml += format!("<ID>{}</ID>", fact.id).as_str();
                xml += format!("<Text>{}</Text>", fact.text).as_str();
                if let Some(ignore_more_to_explore) = fact.ignore_more_to_explore {
                    if ignore_more_to_explore {
                        xml += "<IgnoreMoreToExplore />";
                    }
                }
                xml += "</ExploreFact>";
            }
        }
        if let Some(entry) = entry.entry {
            xml += entry_convert_xml(entry).as_str();
        }
        xml += "</Entry>";
    }
    xml
}
