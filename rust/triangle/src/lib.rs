#[derive(Debug, PartialEq)]
pub struct Triangle {
    t: TriangleType,
}

#[derive(Debug, PartialEq)]
pub enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
}

pub trait Number {
    fn to_f64(&self) -> f64;
}

impl Number for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl Number for f64 {
    fn to_f64(&self) -> f64 {
        *self
    }
}

fn sides_test(t: (f64, f64, f64)) -> bool {
    t.0 + t.1 >= t.2 && t.1 + t.2 >= t.0 && t.2 + t.0 >= t.1 && t.0 > 0.0 && t.1 > 0.0 && t.2 > 0.0
}

fn get_type(t: (f64, f64, f64)) -> TriangleType {
    match () {
        _ if t.0 == t.1 && t.1 == t.2 => TriangleType::Equilateral,
        _ if t.0 == t.1 || t.1 == t.2 || t.0 == t.2 => TriangleType::Isosceles,
        _ => TriangleType::Scalene,
    }
}

impl Triangle {
    pub fn build<I: Number>(input: [I; 3]) -> Option<Triangle> {
        let triangle = (input[0].to_f64(), input[1].to_f64(), input[2].to_f64());

        if !sides_test(triangle) {
            return None;
        }

        Some(Triangle {
            t: get_type(triangle),
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.t == TriangleType::Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        self.t == TriangleType::Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        self.t == TriangleType::Scalene
    }
}
