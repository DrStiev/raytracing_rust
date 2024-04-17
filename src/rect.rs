use nalgebra::Vector3;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

pub enum Plane {
    YZ,
    ZX,
    XY,
}

pub struct Rect<M: Material> {
    plane: Plane,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    k: f64,
    material: M,
}

impl<M: Material> Rect<M> {
    pub fn new(plane: Plane, x0: f64, y0: f64, x1: f64, y1: f64, k: f64, material: M) -> Self {
        Self {
            plane,
            x0,
            y0,
            x1,
            y1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for Rect<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &&self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin()[a_axis] + t * ray.direction()[a_axis];
            let y = ray.origin()[b_axis] + t * ray.direction()[b_axis];
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (y - self.y0) / (self.y1 - self.y0);
                let p = ray.pointing_at(t);
                let mut normal = Vector3::zeros();
                normal[k_axis] = 1.0;
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: &self.material,
                })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let min = Vector3::new(self.x0, self.y0, self.k - 0.0001);
        let max = Vector3::new(self.x1, self.y1, self.k + 0.0001);
        Some(AABB { min, max })
    }
}
