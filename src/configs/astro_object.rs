use crate::{ConfigFile, Entry};

pub fn generate_astro_object_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = match &toml.schema {
        Some(s) => s,
        None => "https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/shiplog_schema.xsd",
    };
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
    xml += format!("</{}>", toml.file_type).as_str();
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
