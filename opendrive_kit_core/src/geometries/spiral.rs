use uom::si::f64::{Angle, Length};
pub struct Spiral {
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
    pub curv_end: f64,
    pub curv_start: f64,
}