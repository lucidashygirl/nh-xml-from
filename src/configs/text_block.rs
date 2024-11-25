use crate::{Conditions, ConfigFile, NomaiTextBlock};

pub fn generate_nomai_text_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = match &toml.schema {
        Some(s) => s,
        None => &"https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/refs/heads/main/NewHorizons/Schemas/text_schema.xsd".to_owned(),
    };
    xml += format!(
        r#"<{} xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        toml.file_type,
        schema
    )
    .as_str();
    if let Some(text_blocks) = &toml.text_block {
        xml = get_text_blocks(xml, text_blocks);
    }

    if let Some(log_conditions) = &toml.log_condition {
        xml = get_log_conditions(xml, log_conditions);
    }
    xml += "</NomaiObject>";
    xml
}

fn get_text_blocks(mut xml: String, text_blocks: &[NomaiTextBlock]) -> String {
    for block in text_blocks {
        xml += "<TextBlock>";
        xml += format!("<ID>{}</ID>", block.id).as_str();
        if let Some(parent) = block.parent {
            xml += format!("<ParentID>{parent}</ParentID>").as_str();
        };
        if let Some(location) = &block.location {
            for loc in location {
                match loc.to_uppercase().as_str() {
                    "A" => xml += "<LocationA />",
                    "B" => xml += "<LocationB />",
                    _ => (),
                }
            }
        }
        xml += format!("<Text>{}</Text>", block.text).as_str();
        xml += "</TextBlock>";
    }
    xml
}

fn get_log_conditions(mut xml: String, log_conditions: &[Conditions]) -> String {
    for block in log_conditions {
        xml += "<ShipLogConditions>";
        if let Some(location) = &block.location {
            for l in location {
                match l.to_uppercase().as_str() {
                    "A" => xml += "<LocationA />",
                    "B" => xml += "<LocationB />",
                    _ => (),
                }
            }
        }
        for fact in &block.reveal_fact {
            xml += "<RevealFact>";
            xml += format!("<FactID>{}</FactID>", fact.id).as_str();
            xml += "<Condition>";
            for (loops, condition) in fact.condition.iter().enumerate() {
                xml += condition.to_string().as_str();
                if loops == fact.condition.len() - 1 {
                    break;
                }
                xml += ", ";
            }
            xml += "</Condition>";
            xml += "</RevealFact>";
        }

        xml += "</ShipLogConditions>".to_string().as_str();
    }
    xml
}
