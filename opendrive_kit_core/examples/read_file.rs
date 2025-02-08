use opendrive_kit_core::quick_xml::OpenDriveMap as OpenDriveMapSerde;
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_path = "example.xodr";
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let odr: OpenDriveMapSerde = from_reader(reader).expect("Failed to parse XML");
    println!("{:#?}", odr);
}