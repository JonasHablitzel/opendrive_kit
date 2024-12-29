use std::collections::BTreeMap;
use bezier_rs::Bezier;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Poly3 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}


impl Poly3 {
    pub fn evaluate_at(&self, ds: f64) -> f64 {
        self.a + self.b * ds + self.c * ds.powi(2) + self.d * ds.powi(3)
    }

    pub fn derivative_at(&self, ds: f64) -> f64 {
        self.b + 2.0 * self.c * ds + 3.0 * self.d * ds * ds
    }

    pub fn second_derivative_at(&self, ds: f64) -> f64 {
        2.0 * self.c + 6.0 * self.d * ds
    }

    pub fn negate(&self) -> Poly3 {
        Poly3{
            a:-1.0*self.a,
            b:-1.0*self.b,
            c:-1.0*self.c,
            d:-1.0*self.d,
        }
    }


    fn bezier_control_points(&self, s_start: f64, s_end: f64) -> [(f64, f64); 4] {
        let mid = (s_start + s_end) / 2.0;

        // Polynomial values at start, mid, and end
        let y_start = self.evaluate_at(s_start);
        let y_mid = self.evaluate_at(mid);
        let y_end = self.evaluate_at(s_end);

        // Use these points to approximate the cubic Bezier control points
        [
            (s_start, y_start),
            ((2.0 * s_start + s_end) / 3.0, (2.0 * y_start + y_mid) / 3.0),
            ((s_start + 2.0 * s_end) / 3.0, (y_mid + 2.0 * y_end) / 3.0),
            (s_end, y_end),
        ]
    }

    pub fn sample_s(&self, eps:f64, s_start:f64, s_end:f64) -> Vec<f64> {
        // similar to the implementation of https://github.com/pageldev/libOpenDRIVE/blob/main/src/Geometries/CubicSpline.cpp
        
        if (s_start - s_end).abs() < std::f64::EPSILON {
            return vec![]
        }
        
        let mut s_vals = Vec::new();
        
        if self.d.abs() < std::f64::EPSILON && self.c.abs() < std::f64::EPSILON {
            // Handle the case where d == 0 and c == 0 (linear polynomial)
            s_vals.push(s_start);
            s_vals.push(s_end);
            return s_vals;
        }

        
        
        if self.d.abs() < std::f64::EPSILON && self.c.abs() >= std::f64::EPSILON {
            // Quadratic polynomial (d == 0, c != 0)
            let mut s = s_start;
            while s < s_end {
                s_vals.push(s);
                // Update s based on the condition
                s = if self.c.abs() > std::f64::EPSILON {
                    s + ((eps / self.c).abs().sqrt())
                } else {
                    s + eps
                }
            }     

            // Ensure the end point is included
            if s_vals.last().unwrap_or(&s_start) < &s_end {
                s_vals.push(s_end);
            }
            return s_vals; 
        } 
        // Cubic polynomial (d != 0)
        // transform to parametric form and use the bezier curve to approximate.

        let control_points = self.bezier_control_points(s_start, s_end);
        let bezier = Bezier::from_linear_coordinates(control_points);


    }
}


