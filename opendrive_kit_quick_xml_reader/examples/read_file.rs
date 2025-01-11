use opendrive_kit_quick_xml_reader::schemas::OpenDriveMap;
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_path = "example.xodr";
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let odr: OpenDriveMap = from_reader(reader).expect("Failed to parse XML");
    println!("{:#?}", odr);
}
