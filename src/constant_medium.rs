use std::sync::Arc;

use crate::{
    util::*, Color, HitRecord, Hittable, Interval, Isotropic, Material, Ray, Texture, Vec3, AABB,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn with_texture(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: d,
            phase_function: Arc::new(Isotropic::with_texture(a)),
        }
    }

    pub fn with_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: d,
            phase_function: Arc::new(Isotropic::with_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(
            r,
            Interval::with_values(f64::NEG_INFINITY, f64::INFINITY),
            &mut rec1,
        ) {
            return false;
        }
        if !self.boundary.hit(
            r,
            Interval::with_values(rec1.t + 0.0001, f64::INFINITY),
            &mut rec2,
        ) {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length: f64 = r.direction().length();
        let distance_inside_boundary: f64 = (rec2.t - rec1.t) * ray_length;
        let hit_distance: f64 = self.neg_inv_density * random_double().log(std::f64::consts::E);

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
