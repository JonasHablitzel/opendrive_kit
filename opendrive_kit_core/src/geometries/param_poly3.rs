
use uom::si::f64::{Angle, Length};
pub enum ParamPoly3pRange {
    ArcLength,
    Normalized
}

pub struct ParamPoly3 {
    /// Length of the element's reference line
    pub length: Length,
    /// Start orientation (inertial heading)
    pub hdg: Angle,
    /// s-coordinate of start position
    pub s: Length,
    /// Start position (x inertial)
    pub x: Length,
    /// Start position (y inertial)
    pub y: Length,
    pub a_u: f64,
    pub a_v: f64,
    pub b_u: f64,
    pub b_v: f64,
    pub c_u: f64,
    pub c_v: f64,
    pub d_u: f64,
    pub d_v: f64,
    pub p_range: ParamPoly3pRange,
}


