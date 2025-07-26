#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    pub fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        pi * self.radius.powi(2)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        if self.center.distance(other.center) <= (self.radius + other.radius) {
            return true
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let x = other.0 - self.0;
        let y = other.1 - self.1;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}