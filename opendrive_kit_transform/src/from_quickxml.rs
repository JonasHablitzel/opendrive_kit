use std::collections::HashMap;

use opendrive_kit_quick_xml_reader::OpenDriveMap as XmlOpenDriveMap;
use opendrive_kit_core::OpenDriveMap as CoreOpenDriveMap;

impl From<XmlOpenDriveMap> for CoreOpenDriveMap {
    fn from(_map: XmlOpenDriveMap) -> Self {
        let roads = HashMap::new();
        Self {
            roads: roads,
        }
    }
}