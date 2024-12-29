use uom::si::f64::Length;
use vec1::Vec1;
use crate::geometries::geometry::Geometry;

pub enum TrafficRule {
    LHT,
    RHT
}


pub struct Road {
    pub id: String,
    pub junction: String,
    pub name: String,
    pub traffic_rule: TrafficRule,
    pub length: Length,
    pub plan_view: Vec1<Geometry>,
}