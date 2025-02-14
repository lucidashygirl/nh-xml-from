use crate::{
    AstroObjectEntry, ConfigFile, Entry, EntryXml, ExploreFact, ExploreFactXml, RumorFact,
    RumorFactXml,
};

const DEFAULT_SCHEMA: &str = "https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/shiplog_schema.xsd";

pub fn generate_astro_object_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = toml.schema.as_ref().map_or(DEFAULT_SCHEMA, |s| s);
    xml += format!(
        r#"<AstroObjectEntry xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{schema}">"#
    )
    .as_str();
    if let Some(id) = &toml.id {
        xml += format!("<ID>{id}</ID>").as_str();
    } else {
        quit!("Field id required for AstroObjectEntry")
    }
    if let Some(entries) = &toml.entry {
        xml += entry_convert_xml(entries).as_str();
    }
    xml += "</AstroObjectEntry>";
    xml
}

pub fn entry_convert_xml(entries: &[Entry]) -> String {
    let mut xml = String::new();

    for entry in entries {
        xml += "<Entry>";
        xml += format!("<ID>{}</ID>", entry.id).as_str();
        xml += format!("<Name>{}</Name>", entry.name).as_str();
        if let Some(curiosity) = &entry.curiosity {
            xml += format!("<Curiosity>{curiosity}</Curiosity>").as_str();
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
        if let Some(ignore_more_to_explore_condition) = &entry.ignore_more_to_explore_condition {
            xml += format!(
                "<IgnoreMoreToExploreCondition>{ignore_more_to_explore_condition}</IgnoreMoreToExploreCondition>"
            )
            .as_str();
        }
        if let Some(alt_photo_condition) = &entry.alt_photo_condition {
            xml += format!("<AltPhotoCondition>{alt_photo_condition}</AltPhotoCondition>").as_str();
        }
        if let Some(rumor_fact) = &entry.rumor_fact {
            for rumor in rumor_fact {
                xml += "<RumorFact>";
                xml += format!("<ID>{}</ID>", rumor.id).as_str();
                if let Some(source_id) = &rumor.source_id {
                    xml += format!("<SourceID>{source_id}</SourceID>").as_str();
                }
                if let Some(rumor_name) = &rumor.rumor_name {
                    xml += format!("<RumorName>{rumor_name}</RumorName>").as_str();
                }
                if let Some(ignore_more_to_explore) = rumor.ignore_more_to_explore {
                    if ignore_more_to_explore {
                        xml += "<IgnoreMoreToExplore />";
                    }
                }
                xml += format!("<Text>{}</Text>", rumor.text).as_str();
                if let Some(rumor_name_priority) = rumor.rumor_name_priority {
                    xml += format!("<RumorNamePriority>{rumor_name_priority}</RumorNamePriority>")
                        .as_str();
                }
                xml += "</RumorFact>";
            }
        }
        if let Some(explore_fact) = &entry.explore_fact {
            for fact in explore_fact {
                xml += "<ExploreFact>";
                xml += format!("<ID>{}</ID>", fact.id).as_str();
                if let Some(ignore_more_to_explore) = fact.ignore_more_to_explore {
                    if ignore_more_to_explore {
                        xml += "<IgnoreMoreToExplore />";
                    }
                }
                xml += format!("<Text>{}</Text>", fact.text).as_str();
                xml += "</ExploreFact>";
            }
        }
        if let Some(entry) = &entry.entry {
            xml += entry_convert_xml(entry).as_str();
        }
        xml += "</Entry>";
    }
    xml
}

pub fn generate_astro_object_config(xml: AstroObjectEntry) -> ConfigFile {
    let mut config: ConfigFile = ConfigFile::default();
    config.file_type = String::from("AstroObjectEntry");
    config.id = xml.id;
    if let Some(entries) = xml.entry {
        config.entry = Some(entry_thingy(entries));
    }
    config
}

pub fn entry_thingy(entries: Vec<EntryXml>) -> Vec<Entry> {
    let mut entry_vec: Vec<Entry> = Vec::new();
    for entry in entries {
        let mut new_entry: Entry = Entry::default();
        new_entry.id = entry.id;
        new_entry.name = entry.name;
        new_entry.curiosity = entry.curiosity;
        if entry.is_curiosity.is_some() {
            new_entry.is_curiosity = Some(true);
        }
        if entry.ignore_more_to_explore.is_some() {
            new_entry.ignore_more_to_explore = Some(true);
        }
        if entry.parent_ignore_not_revealed.is_some() {
            new_entry.parent_ignore_not_revealed = Some(true);
        }
        new_entry.ignore_more_to_explore_condition = entry.ignore_more_to_explore_condition;
        new_entry.alt_photo_condition = entry.alt_photo_condition;
        if let Some(rumor_fact) = entry.rumor_fact {
            new_entry.rumor_fact = Some(get_rumor_fact(rumor_fact));
        }
        if let Some(explore_fact) = entry.explore_fact {
            new_entry.explore_fact = Some(get_explore_fact(explore_fact));
        }
        if let Some(e) = entry.entry {
            new_entry.entry = Some(entry_thingy(e));
        }
        entry_vec.push(new_entry);
    }
    entry_vec
}

pub fn get_rumor_fact(rumors: Vec<RumorFactXml>) -> Vec<RumorFact> {
    let mut rumor_vec: Vec<RumorFact> = Vec::new();
    for rumor in rumors {
        let mut new_rumor = RumorFact::default();
        new_rumor.id = rumor.id;
        new_rumor.source_id = rumor.source_id;
        new_rumor.rumor_name = rumor.rumor_name;
        new_rumor.text = rumor.text;
        new_rumor.rumor_name_priority = rumor.rumor_name_priority;
        if rumor.ignore_more_to_explore.is_some() {
            new_rumor.ignore_more_to_explore = Some(true);
        }
        rumor_vec.push(new_rumor);
    }
    rumor_vec
}

pub fn get_explore_fact(explore_facts: Vec<ExploreFactXml>) -> Vec<ExploreFact> {
    let mut fact_vec: Vec<ExploreFact> = Vec::new();
    for fact in explore_facts {
        let mut new_fact = ExploreFact::default();
        new_fact.id = fact.id;
        new_fact.text = fact.text;
        if fact.ignore_more_to_explore.is_some() {
            new_fact.ignore_more_to_explore = Some(true);
        }
        fact_vec.push(new_fact);
    }
    fact_vec
}
