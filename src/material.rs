// TODO: to be completed
use crate::{
    random_in_unit_sphere, util::*, vec3::*, Color, CosinePdf, HitRecord, Pdf, Point3, Ray,
    SolidColor, SpherePdf, Texture,
};
use std::{ptr, sync::Arc};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

pub trait Material {
    fn emitted(&self, r_in: Ray, rec: HitRecord, u: f64, v: f64, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: &mut ScatterRecord) -> bool {
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
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(rec.normal)));
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(scattered.direction()));
        if cos_theta < 0.0 {
            return 0.0;
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
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.normal);
        srec.skip_pdf_ray = Ray::with_origin_direction_and_time(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        true
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflactance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf((1.0 - cosine), 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = unit_vector(r_in.direction());
        let cos_theta: f64 = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let mut direction: Vec3 = Vec3::new_empty();

        if cannot_refract || Dielectric::reflactance(cos_theta, refraction_ratio) > random_double()
        {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, refraction_ratio);
        }
        srec.skip_pdf_ray = Ray::with_origin_direction_and_time(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn with_texture(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }
    pub fn with_color(c: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, r_in: Ray, rec: HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.emit.value(u, v, &p)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn with_color(c: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn with_texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: Ray, rec: HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(SpherePdf::new()));
        srec.skip_pdf = false;
        true
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}
