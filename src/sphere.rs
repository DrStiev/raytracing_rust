use nalgebra::Vector3;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

// <M: Material> means that the struct can hold any type of 'M'
// that implements the 'Material' trait
pub struct Sphere<M: Material> {
    center: Vector3<f64>,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f64>, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.pointing_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: &self.material,
                });
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.pointing_at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: &self.material,
                });
            }
        }
        None
    }
}
