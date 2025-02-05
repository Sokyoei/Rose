use std::fs::read_to_string;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn main() {
    let sokyoei_data_dir = rose::get_sokyoei_data_dir();
    let xml_file_path = sokyoei_data_dir.join("Ahri").join("Ahri.xml");
    let xml_data = read_to_string(xml_file_path).expect("Failed to read file");

    let mut reader = Reader::from_str(&xml_data);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let tag_name = std::str::from_utf8(name.as_ref()).unwrap();
                println!("Start tag: {}", tag_name);
                // let tag_name = std::str::from_utf8(e.name().as_ref()).unwrap();
                // println!("Start tag: {}", tag_name);
                for attr in e.attributes() {
                    let attr = attr.unwrap();
                    let key = std::str::from_utf8(attr.key.as_ref()).unwrap();
                    let value = std::str::from_utf8(&attr.value).unwrap();
                    println!("  Attribute: {} = {}", key, value);
                }
            }
            Ok(Event::Text(e)) => {
                let text_bytes = e.into_inner();
                let text = std::str::from_utf8(&text_bytes).unwrap();
                println!("Text: {}", text);
            }
            Ok(Event::End(ref e)) => {
                let name = e.name();
                let tag_name = std::str::from_utf8(name.as_ref()).unwrap();
                println!("End tag: {}", tag_name);
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                panic!("Error at position {}: {:?}", reader.buffer_position(), e);
            }
            _ => (),
        }
        buf.clear();
    }
}
