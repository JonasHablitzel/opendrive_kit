use crate::road::Road;
use std::collections::HashMap;

pub struct OpenDriveMap {
    pub name: String,
    pub roads: HashMap<u32, Road>,
}
