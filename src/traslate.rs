use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Traslate<H: Hittable> {
    hitable: H,
    offset: Vector3<f64>,
}

impl<H: Hittable> Traslate<H> {
    pub fn new(hitable: H, offset: Vector3<f64>) -> Self {
        Self { hitable, offset }
    }
}

impl<H: Hittable> Hittable for Traslate<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hitable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1).map(|mut b| {
            b.min += self.offset;
            b.max += self.offset;
            b
        })
    }
}
