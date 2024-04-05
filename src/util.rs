use crate::{dot, Vec3};
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// functions to generate random numbers. Not sure if keep them
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
pub fn random_int() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen::<i64>()
}
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = dot(&-uv, &n).min(1.0);
    let r_out_perp: Vec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: Vec3 = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_parallel + r_out_perp
}

pub fn random_cosine_direction() -> Vec3 {
    let r1: f64 = rand::random();
    let r2: f64 = rand::random();

    let phi: f64 = 2.0 * std::f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();
    Vec3::new(x, y, z)
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new(
            -1.0 + (1.0 - -1.0) * rand::random::<f64>(),
            -1.0 + (1.0 - -1.0) * rand::random::<f64>(),
            -1.0 + (1.0 - -1.0) * rand::random::<f64>(),
        );
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}
