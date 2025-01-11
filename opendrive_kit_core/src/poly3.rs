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
        Poly3 {
            a: -1.0 * self.a,
            b: -1.0 * self.b,
            c: -1.0 * self.c,
            d: -1.0 * self.d,
        }
    }

    pub fn sample_s(&self, eps: f64, s_start: f64, s_end: f64) -> Vec<f64> {
        let count = ((s_end - s_start) / eps).ceil() as usize + 1;
        let mut vec = Vec::with_capacity(count);
        let mut current = s_start;
        while current <= s_end {
            vec.push(current);
            current = (current + eps).min(s_end); // Avoid overshooting due to floating-point precision
        }
        vec
    }
}
