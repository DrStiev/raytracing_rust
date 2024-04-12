use nalgebra::Vector3;

use crate::aabb::{self, AABB};
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<crate::aabb::AABB> {
        let radius = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }
}

pub struct MovingSphere<M: Material> {
    center0: Vector3<f64>,
    center1: Vector3<f64>,
    time0: f64,
    time1: f64,
    radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vector3<f64>,
        center1: Vector3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vector3<f64> {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let oc = ray.origin() - center;

        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.pointing_at(t);
                let normal = (p - center) / self.radius;
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
                let normal = (p - center) / self.radius;
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self.radius);
        let min0 = self.center(t0) - radius;
        let max0 = self.center(t0) + radius;
        let min1 = self.center(t1) - radius;
        let max1 = self.center(t1) + radius;
        let aabb0 = AABB::new(min0, max0);
        let aabb1 = AABB::new(min1, max1);
        Some(aabb::surrounding_box(&aabb0, &aabb1))
    }
}
