use nalgebra::Vector3;
use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::util::random_in_unit_sphere;

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
    fn emitted(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64>;
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture + std::marker::Sync> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

// '_' before a variable name tells the compiler to not worry if the
// parameter is not used. unless it throw a warning
impl<T: Texture + std::marker::Sync> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p, ray.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::zeros()
    }
}

#[derive(Clone)]
pub struct Metal<T: Texture> {
    albedo: T,
    fuzz: f64,
}

impl<T: Texture + std::marker::Sync> Metal<T> {
    pub fn new(albedo: T, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl<T: Texture + std::marker::Sync> Material for Metal<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut reflected = reflect(&ray.direction().normalize(), &hit.normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * random_in_unit_sphere()
        };
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected, ray.time());
            Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::zeros()
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            let cosine =
                self.ref_idx * ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (-hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = -ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };
        if let Some(refracted) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f64>() >= reflect_prob {
                let scattered = Ray::new(hit.p, refracted, ray.time());
                return Some((scattered, attenuation));
            }
        }
        let reflected = reflect(&ray.direction(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected, ray.time());
        Some((scattered, attenuation))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::zeros()
    }
}

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl<T: Texture + std::marker::Sync> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        self.emit.value(u, v, &p)
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture + std::marker::Sync> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let scattered = Ray::new(hit.p, random_in_unit_sphere(), ray.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::zeros()
    }
}
