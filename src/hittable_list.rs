use std::sync::Arc;
use std::vec::Vec;

use crate::{util::*, HitRecord, Hittable, Interval, Point3, Ray, Vec3, AABB};

pub struct HittableList {
    bbox: AABB,
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = AABB::new_with_aabb(self.bbox, object.bounding_box())
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(
                r,
                Interval::with_values(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }

    pub fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let weight: f64 = 1.0 / self.objects.len() as f64;
        let mut sum: f64 = 0.0;
        for object in self.objects.iter() {
            sum += weight * object.pdf_value(&o, &v);
        }
        sum
    }

    pub fn random(&self, o: Vec3) -> Vec3 {
        let init_size = self.objects.len();
        self.objects[random_double_in_range(0.0, init_size as f64) as usize].random(&o)
    }
}
