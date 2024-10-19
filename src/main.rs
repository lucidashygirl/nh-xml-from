use quick_xml::{events::*, reader::*, writer::*};
use serde::*;
use std::{env, fs::*, io::*};
use toml::*;

mod data;

use data::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        panic!("No arguments provided!")
    }

    let file_path = &args[1];
    let mut config = File::open(file_path).expect("File doesn't exist");

    let file_name = get_file_name(file_path);
    let extention = get_file_extention(&file_name);
    let name = file_name.replace(&extention, "");
    if extention.as_str() != "toml" {
        panic!("Incorrect Format!")
    }

    let mut contents: String = String::new();
    config
        .read_to_string(&mut contents)
        .expect("Couldn't convert to string");

    let toml_config: ConfigFile = toml::from_str(contents.as_str()).expect("Invalid Syntax");

    let xml = validate_config(&toml_config);

    let mut reader = Reader::from_str(xml.as_str());
    reader.config_mut().trim_text(true); // NOTE: might need to remove trim_text to get formatting
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"this_tag" => {
                // crates a new element ... alternatively we could reuse `e` by calling
                // `e.into_owned()`
                let mut elem = BytesStart::new("my_elem");

                // collect existing attributes
                elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));

                // copy existing attributes, adds a new my-key="some value" attribute
                elem.push_attribute(("my-key", "some value"));

                // writes the event to the writer
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"this_tag" => {
                assert!(writer
                    .write_event(Event::End(BytesEnd::new("my_elem")))
                    .is_ok());
            }
            Ok(Event::Eof) => break,
            // we can either move or borrow the event to write, depending on your use-case
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
        }
    }

    let result = writer.into_inner().into_inner();
    let mut file = File::create(format!("{}xml", name)).unwrap();
    file.write_all(&result).unwrap();
}

fn validate_config(config: &ConfigFile) -> String {
    match config.file_type.as_str() {
        "NomaiObject" => validate_nomai_text_config(config),
        //"DialogueTree" => validate_dialogue_tree_config(config),
        "AstroObjectEntry" => validate_astral_object_config(config),
        _ => panic!("doesnt match anything lol"),
    }
}

fn validate_nomai_text_config(config: &ConfigFile) -> String {
    let mut nomai_text_blocks: Vec<NomaiTextBlock> = Vec::new();
    let mut shiplog_conditions: Vec<Conditions> = Vec::new();
    let mut text_block_index = 0;

    if let Some(text_block) = &config.text_block {
        for block in text_block {
            nomai_text_blocks.push(NomaiTextBlock::default());
            match block.get("id") {
                Some(id) => {
                    nomai_text_blocks[text_block_index].id = id.as_integer().unwrap() as u16
                }
                None => panic!("field id required"),
            }
            if let Some(parent) = block.get("parent") {
                nomai_text_blocks[text_block_index].parent =
                    Some(parent.as_integer().unwrap() as u16)
            }
            match block.get("text") {
                Some(text) => {
                    nomai_text_blocks[text_block_index].text = text.as_str().unwrap().to_owned()
                }
                None => panic!("field id required"),
            }
            if let Some(loc) = block.get("location") {
                let locations = loc.as_array().unwrap();
                let mut new_locations: Vec<String> = Vec::new();
                for i in locations {
                    new_locations.push(i.as_str().unwrap().to_owned());
                }
                nomai_text_blocks[text_block_index].location = Some(new_locations);
            }
            text_block_index += 1;
        }
    }

    let mut text_block_index = 0;
    println!("{:?}", &config.log_condition);
    if let Some(cond) = &config.log_condition {
        for block in cond {
            shiplog_conditions.push(Conditions::default());
            println!("{}", block);
            if let Some(loc) = block.get("location") {
                let locations = loc.as_array().unwrap();
                let mut new_locations: Vec<String> = Vec::new();
                for i in locations {
                    new_locations.push(i.as_str().unwrap().to_owned());
                }
                shiplog_conditions[text_block_index].location = Some(new_locations);
            }
            if let Some(facts) = block.get("reveal_fact") {
                println!("fat {}", facts);
                if let Some(table) = facts.as_array() {
                    for thing in table {
                        let mut fact = Fact::default();
                        match thing.get("id") {
                            Some(id) => fact.id = id.to_string(),
                            None => break,
                        }
                        match thing.get("condition") {
                            Some(condition) => {
                                println!("cond {}", condition);
                                if let Some(con) = condition.as_array() {
                                    for i in con {
                                        println!("i {}", i);
                                        fact.condition.push(i.to_string());
                                    }
                                }
                            }
                            None => break,
                        }
                        println!("FACT REVEALED");
                        shiplog_conditions[text_block_index].reveal_fact.push(fact);
                    }
                }
            }
            text_block_index += 1;
        }
    }
    println!("{nomai_text_blocks:?}");
    println!("{shiplog_conditions:?}");
    //if shiplog_conditions.is_empty() {
    //    return (nomai_text_blocks, None);
    //}
    generate_nomai_text_xml_string(&config, (nomai_text_blocks, Some(shiplog_conditions)))
}

fn validate_astral_object_config(config: &ConfigFile) -> String {
    let mut entries: Vec<Entry> = Vec::new();
    let id = match &config.id {
        Some(id) => id.clone(),
        None => panic!("invalid id"),
    };
    validate_entry_config(config, &mut entries);
    let entry = Some(entries);
    let astro_object = AstroObjectEntry { id, entry };
    generate_astro_object_xml_string(config, astro_object)
}

fn validate_entry_config(config: &ConfigFile, entries: &mut Vec<Entry>) {
    if let Some(blocks) = &config.entry {
        for block in blocks {
            let mut entry_block = Entry::default();
            match block.get("id") {
                Some(id) => entry_block.id = id.to_string(),
                None => panic!("field id required"),
            }
            match block.get("name") {
                Some(name) => entry_block.name = name.to_string(),
                None => panic!("field id required"),
            }
            if let Some(curiosity) = block.get("curiosity") {
                entry_block.curiosity = Some(curiosity.to_string());
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
                entry_block.ignore_more_to_explore_condition =
                    Some(ignore_more_to_explore_condition.to_string());
            }
            if let Some(alt_photo_condition) = block.get("alt_photo_condition") {
                entry_block.alt_photo_condition = Some(alt_photo_condition.to_string());
            }
            if let Some(rumor_fact) = block.get("rumor_fact") {
                let mut facts: Vec<RumorFact> = Vec::new();
                for fact in rumor_fact.as_table() {
                    let mut rumor = RumorFact::default();
                    match fact.get("id") {
                        Some(id) => rumor.id = id.to_string(),
                        None => panic!("field id required"),
                    }
                    if let Some(source_id) = fact.get("source_id") {
                        rumor.source_id = Some(source_id.to_string());
                    }
                    if let Some(rumor_name) = fact.get("rumor_name") {
                        rumor.rumor_name = Some(rumor_name.to_string());
                    }
                    if let Some(rumor_name_priority) = fact.get("rumor_name_priority") {
                        rumor.rumor_name_priority = Some(rumor_name_priority.as_integer().unwrap());
                    }
                    if let Some(ignore_more_to_explore) = block.get("ignore_more_to_explore") {
                        entry_block.ignore_more_to_explore = ignore_more_to_explore.as_bool();
                    }
                }
            }
            entries.push(entry_block);
        }
    }
}

fn get_file_extention(file_name: &String) -> String {
    let mut extention_characters: Vec<char> = Vec::new();
    for character in file_name.chars().rev() {
        if character == '.' {
            return convert_extention_to_string(extention_characters);
        }
        extention_characters.push(character);
    }
    "".to_owned()
}

fn get_file_name(file_name: &String) -> String {
    let mut name_chars: Vec<char> = Vec::new();
    let mut name = String::new();
    for character in file_name.chars().rev() {
        if character == '/' {
            name = convert_extention_to_string(name_chars);
            break;
        }
        name_chars.push(character);
    }
    name
}

fn convert_extention_to_string(extention_characters: Vec<char>) -> String {
    extention_characters.into_iter().rev().collect()
}

fn generate_nomai_text_xml_string(
    toml: &ConfigFile,
    blocks: (Vec<NomaiTextBlock>, Option<Vec<Conditions>>),
) -> String {
    let mut xml = String::new();
    xml += format!(
        r#"<{} xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        toml.file_type,
        toml.schema
    ).as_str();
    println!("block 0 started");
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
    println!("block 1 started");
    if let Some(ref blocks) = blocks.1 {
        for block in blocks {
            println!("bloc {:?}", block);
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
                xml += format!("<RevealFact>").as_str();
                xml += format!("<FactID>{}</FactID>", fact.id).as_str();
                xml += "<Condition>";
                let mut loops = 0;
                for condition in &fact.condition {
                    xml += format!("{}", condition).as_str();
                    if loops == fact.condition.len() - 1 {
                        break;
                    }
                    xml += ", ";
                    loops += 1;
                }
                xml += "</Condition>";
                xml += "</RevealFact>";
            }

            xml += format!("</ShipLogConditions>").as_str();
        }
    }
    xml += "</NomaiObject>";
    xml
}

fn generate_astro_object_xml_string(toml: &ConfigFile, blocks: AstroObjectEntry) -> String {
    let mut xml = String::new();
    xml += format!(
        r#"<{} xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        toml.file_type,
        toml.schema
    ).as_str();
    xml += "<AstroObjectEntry>";
    xml += format!("<ID>{}</ID>", blocks.id).as_str();
    if let Some(entries) = blocks.entry {
        xml += entry_convert_xml(entries).as_str();
    }
    xml += "</AstroObjectEntry>";
    xml
}

fn entry_convert_xml(entries: Vec<Entry>) -> String {
    let mut xml = String::new();
    for entry in entries {
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
                xml += format!("<ID>{}</ID>", rumor.id).as_str();
                if let Some(source_id) = rumor.source_id {
                    xml += format!("<SourceID>{}</SourceID>", source_id).as_str();
                }
                if let Some(rumor_name) = rumor.rumor_name {
                    xml += format!("<RumorName>{}</RumorName>", rumor_name).as_str();
                }
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
            }
        }
        if let Some(explore_fact) = entry.explore_fact {
            for fact in explore_fact {
                xml += format!("<ID>{}</ID>", fact.id).as_str();
                if let Some(ignore_more_to_explore) = fact.ignore_more_to_explore {
                    if ignore_more_to_explore {
                        xml += "<IgnoreMoreToExplore />";
                    }
                }
            }
        }
        if let Some(entry) = entry.entry {
            xml += entry_convert_xml(entry).as_str();
        }
    }
    xml
}