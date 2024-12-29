use uom::si::reciprocal_length::reciprocal_meter;
use uom::si::f64::{Angle, ReciprocalLength, Length};
use crate::geometries::RoadGeometry;
use crate::math::{Point2D, Tangent2D};

pub struct Line {
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
}

impl RoadGeometry for Line {
    fn position_at(&self, s:Length) -> Point2D {
        let x = (self.hdg.cos() * (s - self.s)) + self.x;
        let y = (self.hdg.sin() * (s - self.s)) + self.y;
        Point2D{x, y}
    }

    fn tangent_at(&self, _s:Length) -> Tangent2D {
        let dx = self.hdg.cos().value;
        let dy = self.hdg.sin().value;        
        Tangent2D{dx, dy}
    }

    fn sample_s(&self, _eps: f64) -> Vec<Length> {
        vec![self.s_start(), self.s_end()]
    }

    fn curvature_at(&self, _s: Length) -> ReciprocalLength {
        ReciprocalLength::new::<reciprocal_meter>(0.0)
    }

    fn s_end(&self) -> Length {
        self.length + self.s
    }

    fn s_start(&self) -> Length {
        self.s
    }
}




#[cfg(test)]
mod tests {
    use super::*; // Import the implementation and dependencies
    use uom::si::f64::{Angle, Length};
    use uom::si::angle::radian;
    use uom::si::length::meter;

    #[test]
    fn test_get_xy() {
        let line = Line {
            length: Length::new::<meter>(100.0),
            hdg: Angle::new::<radian>(0.0), // Heading along the positive x-axis
            s: Length::new::<meter>(0.0),
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0),
        };

        // Query point at s = 50
        let s_query = Length::new::<meter>(50.0);
        let position = line.position_at(s_query);

        assert_eq!(position.x, Length::new::<meter>(50.0));
        assert_eq!(position.y, Length::new::<meter>(0.0));

        // Query point at s = 100
        let s_query = Length::new::<meter>(100.0);
        let position = line.position_at(s_query);

        assert_eq!(position.x, Length::new::<meter>(100.0));
        assert_eq!(position.y, Length::new::<meter>(0.0));
    }

    #[test]
    fn test_get_grad() {
        let line = Line {
            length: Length::new::<meter>(100.0),
            hdg: Angle::new::<radian>(std::f64::consts::FRAC_PI_4), // 45 degrees
            s: Length::new::<meter>(0.0),
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0),
        };

        let grad = line.tangent_at(Length::new::<meter>(50.0)); // `s` is irrelevant for grad

        assert!((grad.dx - 0.70710678118).abs() < 1e-10); // cos(45 degrees)
        assert!((grad.dy - 0.70710678118).abs() < 1e-10); // sin(45 degrees)
    }

    #[test]
    fn test_get_grad_and_xy() {
        let line = Line {
            length: Length::new::<meter>(100.0),
            hdg: Angle::new::<radian>(std::f64::consts::FRAC_PI_4), // 45 degrees
            s: Length::new::<meter>(0.0),
            x: Length::new::<meter>(10.0),
            y: Length::new::<meter>(20.0),
        };

        let grad = line.tangent_at(Length::new::<meter>(50.0)); // `s` is irrelevant for grad

        assert!((grad.dx - 0.70710678118).abs() < 1e-10); // cos(45 degrees)
        assert!((grad.dy - 0.70710678118).abs() < 1e-10); // sin(45 degrees)

        let p1 = line.position_at(Length::new::<meter>(10.0));
        assert!((p1.x  - Length::new::<meter>(17.707)).value < 1e-6);
        assert!((p1.y  - Length::new::<meter>(27.707)).value < 1e-6);
    }

    #[test]
    fn test_get_grad_and_xy_s() {
        let line = Line {
            length: Length::new::<meter>(100.0),
            hdg: Angle::new::<radian>(std::f64::consts::FRAC_PI_4), // 45 degrees
            s: Length::new::<meter>(10.0),
            x: Length::new::<meter>(10.0),
            y: Length::new::<meter>(20.0),
        };

        let p1 = line.position_at(Length::new::<meter>(20.0)); // same values as before but we start now from 10 so 20-10 =10 again.
        assert!((p1.x  - Length::new::<meter>(17.707)).value < 1e-6);
        assert!((p1.y  - Length::new::<meter>(27.707)).value < 1e-6);
    }
}