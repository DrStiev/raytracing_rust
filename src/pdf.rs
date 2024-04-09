use std::sync::Arc;

use crate::{util, vec3::*, Hittable, Point3, ONB};

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::new_empty()
    }
}

pub struct CosinePdf {
    uvw: ONB,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(direction), &self.uvw.w());
        f64::max(0.0, cosine_theta / std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.vec3_local(util::random_cosine_direction())
    }
}

pub struct SpherePdf;

impl SpherePdf {
    pub fn new() -> Self {
        Self
    }
}

impl Pdf for SpherePdf {
    fn value(&self, direction: Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct HittablePdf {
    origin: Point3,
    objects: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self {
            origin: origin,
            objects: objects,
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, &direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if util::random_double() < 0.5 {
            return self.p[0].generate();
        } else {
            return self.p[1].generate();
        }
    }
}
