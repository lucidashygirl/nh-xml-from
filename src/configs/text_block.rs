use crate::*;

pub fn validate_nomai_text_config(config: &ConfigFile) -> String {
    let mut nomai_text_blocks: Vec<NomaiTextBlock> = Vec::new();
    let mut shiplog_conditions: Vec<Conditions> = Vec::new();

    if let Some(text_block) = &config.text_block {
        for (loops, block) in text_block.iter().enumerate() {
            nomai_text_blocks.push(NomaiTextBlock::default());
            match block.get("id") {
                Some(id) => {
                    nomai_text_blocks[loops].id = match id.as_integer() {
                        Some(id) => id,
                        None => quit!(format!(
                            "Invalid type for text_block[{}].id, expected Integer",
                            loops
                        )),
                    }
                }
                None => quit!(format!("text_block[{}].id not found", loops)),
            }
            if let Some(parent) = block.get("parent") {
                nomai_text_blocks[loops].parent = match parent.as_integer() {
                    Some(p) => Some(p),
                    None => quit!(format!(
                        "Invalid type for text_block[{}].parent, expected Integer",
                        loops
                    )),
                }
            }
            match block.get("text") {
                Some(text) => {
                    nomai_text_blocks[loops].text = match text.as_str() {
                        Some(t) => t.to_owned(),
                        None => quit!(format!(
                            "Invalid type for text_block[{}].text, expected String",
                            loops
                        )),
                    }
                }
                None => quit!(format!("text_block[{}].text not found", loops)),
            }
            if let Some(loc) = block.get("location") {
                let locations = match loc.as_array() {
                    Some(l) => l,
                    None => quit!(format!(
                        "Invalid type for text_block[{}].location, expected [String]",
                        loops
                    )),
                };
                let mut new_locations: Vec<String> = Vec::new();
                for (local_loop, i) in locations.iter().enumerate() {
                    let valid_loc = match i.as_str() {
                        Some(l) => l.to_owned(),
                        None => quit!(format!(
                            "Invalid type for text_block[{}].location[{}], expected String",
                            loops, local_loop
                        )),
                    };
                    new_locations.push(valid_loc);
                }
                nomai_text_blocks[loops].location = Some(new_locations);
            }
        }
    }

    if let Some(cond) = &config.log_condition {
        for (root_loops, block) in cond.iter().enumerate() {
            shiplog_conditions.push(Conditions::default());
            if let Some(loc) = block.get("location") {
                let locations = match loc.as_array() {
                    Some(l) => l,
                    None => quit!(format!(
                        "Invalid type for log_condition[{}].location, expected [String]",
                        root_loops
                    )),
                };
                let mut new_locations: Vec<String> = Vec::new();
                for (local_loop, i) in locations.iter().enumerate() {
                    let valid_loc = match i.as_str() {
                        Some(l) => l.to_owned(),
                        None => quit!(format!(
                            "Invalid type for log_condition[{}].location[{}], expected String",
                            root_loops, local_loop
                        )),
                    };
                    new_locations.push(valid_loc);
                }
                shiplog_conditions[root_loops].location = Some(new_locations);
            }
            if let Some(facts) = block.get("reveal_fact") {
                if let Some(table) = facts.as_array() {
                    for (loops, thing) in table.iter().enumerate() {
                        let mut fact = Fact::default();
                        match thing.get("id") {
                            Some(id) => fact.id = match id.as_str() {
                                Some(i) => i.to_owned(),
                                None => quit!(format!("Invalid type for log_condition[{}].reveal_fact[{}].id, expected String", root_loops, loops))
                            },
                            None => quit!(format!("log_condition[{}].reveal_fact[{}].id not found", root_loops, loops)),
                        }
                        match thing.get("condition") {
                            Some(condition) => {
                                let con = match condition.as_array() {
                                    Some(c) => c,
                                    None => quit!(format!("Invalid type for log_condition[{}].reveal_fact[{}].condition", root_loops, loops))
                                };
                                for (local_loop, i) in con.iter().enumerate() {
                                    let var_con = match i.as_str() {
                                        Some(c) => c.to_owned(),
                                        None => quit!(format!("Invalid type for log_condition[{}].reveal_fact[{}].condition[{}]", root_loops, loops, local_loop)),
                                    };
                                    fact.condition.push(var_con);
                                }
                            }
                            None => quit!(format!(
                                "log_condition[{}].reveal_fact[{}].condition not found",
                                root_loops, loops
                            )),
                        }
                        shiplog_conditions[loops].reveal_fact.push(fact);
                    }
                }
            }
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
