use crate::{BytesEnd, BytesStart, Cursor, Event, Reader, Writer};

pub fn get_file_extention(file_name: &str) -> String {
    let mut extention_characters: Vec<char> = Vec::new();
    for character in file_name.chars().rev() {
        if character == '.' {
            return convert_extention_to_string(extention_characters);
        }
        extention_characters.push(character);
    }
    String::new()
}

pub fn get_file_name(file_name: &str) -> String {
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

pub fn convert_extention_to_string(extention_characters: Vec<char>) -> String {
    extention_characters.into_iter().rev().collect()
}

pub fn create_xml_byte_vector(xml: &str) -> Vec<u8> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), 32, 2);
    // NOTE: might replace with a currying function later, but i cannot get it to work lol
    // also this is from the quickxml examples so no i dont know what this does :3c
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"this_tag" => {
                // crates a new element ... alternatively we could reuse `e` by calling
                // `e.into_owned()`
                let mut elem = BytesStart::new("my_elem");

                // collect existing attributes
                #[allow(clippy::unwrap_used)] // you literally cant error here lol
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
    writer.into_inner().into_inner()
}
