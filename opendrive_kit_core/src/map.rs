use std::collections::HashMap;
use crate::road::Road;


pub struct OpenDriveMap {
    pub name: String,
    pub id_to_road: HashMap<String,Road>,
}