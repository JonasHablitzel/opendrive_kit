use crate::geometries::arc::Arc;
use crate::geometries::line::Line;
use crate::geometries::param_poly3::ParamPoly3;
use crate::geometries::spiral::Spiral;
use crate::math::{Point2D, Tangent2D};
use uom::si::f64::{Length, ReciprocalLength};

pub trait RoadGeometry {
    fn position_at(&self, s: Length) -> Point2D;
    fn tangent_at(&self, s: Length) -> Tangent2D;
    fn curvature_at(&self, s: Length) -> ReciprocalLength;
    fn sample_s(&self, eps: f64) -> Vec<Length>;
    fn s_start(&self) -> Length;
    fn s_end(&self) -> Length;
}

pub enum Geometry {
    Line(Line),
    Spiral(Spiral),
    Arc(Arc),
    ParamPoly3(ParamPoly3),
}
