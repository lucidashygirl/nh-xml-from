use crate::*;

pub fn validate_nomai_text_config(config: &ConfigFile) -> String {
    let mut nomai_text_blocks: Vec<NomaiTextBlock> = Vec::new();
    let mut shiplog_conditions: Vec<Conditions> = Vec::new();
    let mut loops = 0;

    if let Some(text_block) = &config.text_block {
        for block in text_block {
            nomai_text_blocks.push(NomaiTextBlock::default());
            match block.get("id") {
                Some(id) => nomai_text_blocks[loops].id = id.as_integer().unwrap() as u16,
                None => quit!("ID required"),
            }
            if let Some(parent) = block.get("parent") {
                nomai_text_blocks[loops].parent = Some(parent.as_integer().unwrap() as u16)
            }
            match block.get("text") {
                Some(text) => nomai_text_blocks[loops].text = text.as_str().unwrap().to_owned(),
                None => quit!("Text required"),
            }
            if let Some(loc) = block.get("location") {
                let locations = loc.as_array().unwrap();
                let mut new_locations: Vec<String> = Vec::new();
                for i in locations {
                    new_locations.push(i.as_str().unwrap().to_owned());
                }
                nomai_text_blocks[loops].location = Some(new_locations);
            }
            loops += 1;
        }
    }

    let mut loops = 0;
    if let Some(cond) = &config.log_condition {
        for block in cond {
            shiplog_conditions.push(Conditions::default());
            if let Some(loc) = block.get("location") {
                let locations = match loc.as_array() {
                    Some(l) => l,
                    None => quit!("Failed to get locations"),
                };
                let mut new_locations: Vec<String> = Vec::new();
                for i in locations {
                    new_locations.push(i.as_str().unwrap().to_owned());
                }
                shiplog_conditions[loops].location = Some(new_locations);
            }
            if let Some(facts) = block.get("reveal_fact") {
                if let Some(table) = facts.as_array() {
                    for thing in table {
                        let mut fact = Fact::default();
                        match thing.get("id") {
                            Some(id) => fact.id = id.as_str().unwrap().to_owned(),
                            None => break,
                        }
                        match thing.get("condition") {
                            Some(condition) => {
                                if let Some(con) = condition.as_array() {
                                    for i in con {
                                        fact.condition.push(i.to_string());
                                    }
                                }
                            }
                            None => break,
                        }
                        shiplog_conditions[loops].reveal_fact.push(fact);
                    }
                }
            }
            loops += 1;
        }
    }
    generate_nomai_text_xml_string(config, (nomai_text_blocks, Some(shiplog_conditions)))
}

pub fn generate_nomai_text_xml_string(
    toml: &ConfigFile,
    blocks: (Vec<NomaiTextBlock>, Option<Vec<Conditions>>),
) -> String {
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
    for block in blocks.0 {
        xml += "<TextBlock>";
        xml += format!("<ID>{}</ID>", block.id).as_str();
        if let Some(parent) = block.parent {
            xml += format!("<ParentID>{}</ParentID>", parent).as_str();
        };
        if let Some(location) = block.location {
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
    if let Some(ref blocks) = blocks.1 {
        for block in blocks {
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
    }
    xml += "</NomaiObject>";
    xml
}
