use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
    pub fn pointing_at(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}
