use crate::road::Road;
use std::collections::HashMap;

pub struct OpenDriveMap {
    pub roads: HashMap<u32, Road>,
}
