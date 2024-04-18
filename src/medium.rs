use nalgebra::Vector3;
use rand::Rng;
use std::f64;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Isotropic;
use crate::ray::Ray;
use crate::texture::Texture;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    density: f64,
    phase_function: Isotropic<T>,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f64, texture: T) -> Self {
        Self {
            boundary,
            density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hittable, T: Texture + std::marker::Sync> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(&ray, -f64::MAX, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(&ray, hit1.t + 0.0001, f64::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min
                }
                if hit2.t > t_max {
                    hit2.t = t_max
                }
                if hit1.t < hit2.t {
                    let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().norm();
                    let hit_distance = -(1.0 / self.density) * rng.gen::<f64>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray.direction().norm();
                        return Some(HitRecord {
                            t,
                            u: 0.0,
                            v: 0.0,
                            p: ray.pointing_at(t),
                            normal: Vector3::new(1.0, 0.0, 0.0), // choose arbitrary
                            material: &self.phase_function,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
