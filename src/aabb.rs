use nalgebra::Vector3;
use std::f64;

use crate::ray::Ray;

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vector3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z),
    );
    let max = Vector3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z),
    );
    AABB { min, max }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl AABB {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
