// TODO: to be completed
use crate::{random_in_unit_sphere, Color, HitRecord, Point3, Ray, SolidColor, Texture, Vec3};
use std::{ptr, sync::Arc};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

pub trait Material {
    fn emitted(&self, r_in: Ray, rec: HitRecord, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: ScatterRecord) -> bool {
        false
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn color(a: Color) -> Self {
        Self {
            albedo: SolidColor::new(a),
        }
    }
    pub fn texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = CosinePdf::new(rec.normal);
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cos_theta = Vec3::dot(rec.normal, Vec3::unit_vector(scattered.direction()));
        if cos_theta < 0 {
            return 0;
        }
        cos_theta / std::f64::consts::PI
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1 { f } else { 1 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: ScatterRecord) -> bool {
        srec.attenuation = self.albedo;
        srec.pdf_ptr = ptr::null();
        srec.skip_pdf = true;
        let reflected: Vec3 = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        srec.skip_pdf_ray = Ray::with_origin_direction_and_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        true
    }
}

// TODO: complete dielectric, diffuse_light, and isotropic
