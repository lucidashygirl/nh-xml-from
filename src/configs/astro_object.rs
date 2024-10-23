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
        Some(blocks) => for_entry_config(blocks, entries).unwrap(),
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
            Some(id) => entry_block.id = id.as_str().unwrap().to_string(),
            None => quit!("ID required"),
        }
        match block.get("name") {
            Some(name) => entry_block.name = name.as_str().unwrap().to_string(),
            None => quit!("Name required"),
        }
        if let Some(curiosity) = block.get("curiosity") {
            entry_block.curiosity = Some(curiosity.as_str().unwrap().to_string());
        }
        if let Some(is_curiosity) = block.get("is_curiosity") {
            entry_block.is_curiosity = is_curiosity.as_bool();
        }
        if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
            entry_block.ignore_more_to_explore = ignore_more_to_explore.as_bool();
        }
        if let Some(parent_ignore_not_revealed) = block.get("parent_ignore_not_revealed") {
            entry_block.parent_ignore_not_revealed = parent_ignore_not_revealed.as_bool();
        }
        if let Some(ignore_more_to_explore_condition) =
            block.get("ignore_more_to_explore_condition")
        {
            entry_block.ignore_more_to_explore_condition = Some(
                ignore_more_to_explore_condition
                    .as_str()
                    .unwrap()
                    .to_string(),
            );
        }
        if let Some(alt_photo_condition) = block.get("alt_photo_condition") {
            entry_block.alt_photo_condition =
                Some(alt_photo_condition.as_str().unwrap().to_string());
        }
        if let Some(rumor_facts) = block.get("rumor_fact") {
            let mut facts: Vec<RumorFact> = Vec::new();
            #[allow(for_loops_over_fallibles)]
            for rumor_fact in rumor_facts.as_array() {
                for fact in rumor_fact {
                    let mut rumor = RumorFact::default();
                    match fact.get("id") {
                        Some(id) => rumor.id = id.as_str().unwrap().to_string(),
                        None => quit!("ID required"),
                    }
                    match fact.get("text") {
                        Some(text) => rumor.text = text.as_str().unwrap().to_string(),
                        None => quit!("Text required"),
                    }
                    if let Some(source_id) = fact.get("source_id") {
                        rumor.source_id = Some(source_id.as_str().unwrap().to_string());
                    }
                    if let Some(rumor_name) = fact.get("rumor_name") {
                        rumor.rumor_name = Some(rumor_name.as_str().unwrap().to_string());
                    }
                    if let Some(rumor_name_priority) = fact.get("rumor_name_priority") {
                        rumor.rumor_name_priority = Some(rumor_name_priority.as_integer().unwrap());
                    }
                    if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
                        entry_block.ignore_more_to_explore = ignore_more_to_explore.as_bool();
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
                        Some(id) => explore.id = id.as_str().unwrap().to_string(),
                        None => quit!("ID required"),
                    }
                    match fact.get("text") {
                        Some(text) => explore.text = text.as_str().unwrap().to_string(),
                        None => quit!("Text required"),
                    }
                    if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
                        entry_block.ignore_more_to_explore = ignore_more_to_explore.as_bool();
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
                    local_entries.push(t.as_table().unwrap().clone())
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
