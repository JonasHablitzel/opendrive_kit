use crate::geometries::RoadGeometry;
use crate::math::{Point2D, Tangent2D};
use std::f64::consts::FRAC_PI_2;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length, ReciprocalLength};
use uom::si::length::meter;
use uom::si::ratio::ratio;
use uom::si::reciprocal_length::reciprocal_meter;

pub struct Arc {
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
    /// The Curvature of the Arc
    pub curvature: ReciprocalLength,
}

impl RoadGeometry for Arc {
    fn position_at(&self, s: Length) -> Point2D {
        // (s - s0) is a length. curvature is 1/length. Multiplying them gives a dimensionless f64.
        let ds: f64 = (s - self.s).get::<meter>();
        let c: f64 = self.curvature.get::<reciprocal_meter>();
        // heading in radians
        let hdg0: f64 = self.hdg.get::<radian>();

        // angle_at_s is dimensionless in code, but physically it's in radians
        let angle_at_s = ds * c - FRAC_PI_2;

        // r is 1 / curvature => length in meters
        let r = 1.0 / c;

        // compute new X, Y in f64
        let xs = r * ((hdg0 + angle_at_s).cos() - hdg0.sin()) + self.x.get::<meter>();
        let ys = r * ((hdg0 + angle_at_s).sin() + hdg0.cos()) + self.y.get::<meter>();

        // Wrap them back into strongly typed lengths
        Point2D {
            x: Length::new::<meter>(xs),
            y: Length::new::<meter>(ys),
        }
    }

    fn tangent_at(&self, s: Length) -> Tangent2D {
        let ds = (s - self.s).get::<meter>(); // [m]
        let c = self.curvature.get::<reciprocal_meter>(); // [1/m]
        let hdg0 = self.hdg.get::<radian>(); // [rad], but typed as f64

        let angle = std::f64::consts::FRAC_PI_2 - c * ds - hdg0;

        let dx = angle.sin();
        let dy = angle.cos();

        Tangent2D { dx: dx, dy: dy }
    }

    fn curvature_at(&self, _s: Length) -> ReciprocalLength {
        self.curvature
    }

    fn sample_s(&self, eps: f64) -> Vec<Length> {
        // 1) Step size in meters (0.01 rad / curvature = meters)
        let step_len = Length::new::<meter>(0.01 / self.curvature.get::<per_meter>().abs());

        // 2) Start and end along the arc
        let arc_start = self.s;
        let arc_end = self.s + self.length;

        // 3) Compute how many steps fit in `self.length`
        //    (length / step_len is dimensionless, so use `.get::<ratio>()`)
        let total_steps = (self.length / step_len).get::<ratio>().floor() as usize;

        // 4) Pre-allocate a vector for efficiency
        let mut s_vals = Vec::with_capacity(total_steps + 1);

        // 5) Use a for loop to generate intermediate points
        for i in 0..total_steps {
            let s_current = arc_start + step_len * (i as f64);
            s_vals.push(s_current);
        }

        // 6) Ensure the exact final endpoint is included
        s_vals.push(arc_end);

        s_vals
    }

    fn s_start(&self) -> Length {
        self.s
    }

    fn s_end(&self) -> Length {
        self.s + self.length
    }
}
