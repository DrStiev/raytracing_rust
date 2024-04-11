use nalgebra::Vector3;

use crate::material::Material;
use crate::ray::Ray;

// 'a is a lifetime parameter. is used to indicate that the struct
// can contain references with a specific lifetime
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // 'static is a special lifetime that indicates that a reference is valid
    // for the entire duration of the program
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}
