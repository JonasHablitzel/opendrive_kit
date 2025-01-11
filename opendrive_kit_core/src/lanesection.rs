use crate::lane::Lane;
use std::collections::BTreeMap;

pub struct Lanesection {
    pub s: f64,
    pub single_side: bool,
    pub lanes: BTreeMap<i32, Lane>,
}
