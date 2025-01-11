use crate::geometries::geometry::Geometry;
use crate::lanesection::Lanesection;
use std::collections::BTreeMap;
use uom::si::f64::Length;

pub enum TrafficRule {
    LHT,
    RHT,
}

pub struct Road {
    pub id: u32,
    pub junction: String,
    pub length: Length,
    pub name: Option<String>,
    pub rule: Option<TrafficRule>,
    pub plan_view: BTreeMap<Length, Geometry>,
    pub lane_sections: BTreeMap<Length, Lanesection>,
}
