use uom::si::f64::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: Length,
    pub y: Length,
}

impl Point2D {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }
}

// as the gardient is the unit diretion of travel its unitless
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tangent2D {
    pub dx: f64,
    pub dy: f64,
}

impl Tangent2D {
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}
