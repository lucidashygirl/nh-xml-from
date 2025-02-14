use crate::{
    Conditions, ConditionsXml, ConfigFile, Fact, NomaiObject, NomaiTextBlock, NomaiTextBlockXml,
};

const DEFAULT_SCHEMA: &str = "https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/text_schema.xsd";

pub fn generate_nomai_text_xml_string(toml: &ConfigFile) -> String {
    let mut xml = String::new();
    let schema = toml.schema.as_ref().map_or(DEFAULT_SCHEMA, |s| s);
    xml += format!(
        r#"<NomaiObject xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
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

pub fn generate_nomai_object_config(xml: NomaiObject) -> ConfigFile {
    let mut config: ConfigFile = ConfigFile::default();
    config.file_type = String::from("NomaiObject");
    if let Some(blocks) = xml.text_block {
        config.text_block = Some(textblock_thingy(blocks));
    }
    if let Some(cond) = xml.log_condition {
        config.log_condition = Some(log_thingy(cond));
    }
    config
}

pub fn textblock_thingy(blocks: Vec<NomaiTextBlockXml>) -> Vec<NomaiTextBlock> {
    let mut block_vec: Vec<NomaiTextBlock> = Vec::new();
    for block in blocks {
        let mut new_block: NomaiTextBlock = NomaiTextBlock::default();
        new_block.id = block.id;
        new_block.parent = block.parent;
        new_block.location = match (block.location_a, block.location_b) {
            (Some(_), Some(_)) => Some(Vec::from(["A".to_owned(), "B".to_owned()])),
            (Some(_), None) => Some(Vec::from(["A".to_owned()])),
            (None, Some(_)) => Some(Vec::from(["B".to_owned()])),
            _ => None,
        };
        new_block.text = block.text;
        block_vec.push(new_block);
    }
    block_vec
}

pub fn log_thingy(logs: Vec<ConditionsXml>) -> Vec<Conditions> {
    let mut log_vec: Vec<Conditions> = Vec::new();
    for log in logs {
        let mut new_log: Conditions = Conditions::default();
        new_log.location = match (log.location_a, log.location_b) {
            (Some(_), Some(_)) => Some(Vec::from(["A".to_owned(), "B".to_owned()])),
            (Some(_), None) => Some(Vec::from(["A".to_owned()])),
            (None, Some(_)) => Some(Vec::from(["B".to_owned()])),
            _ => None,
        };
        let mut fact_vec: Vec<Fact> = Vec::new();
        for fact in log.reveal_fact {
            fact_vec.push(Fact {
                id: fact.id,
                condition: fact.condition,
            })
        }
        new_log.reveal_fact = fact_vec;
        log_vec.push(new_log);
    }
    log_vec
}
