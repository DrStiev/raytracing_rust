use nalgebra::Vector3;

use crate::hittable::Hittable;
use crate::ray::Ray;

pub fn color(ray: &Ray, world: &Box<dyn Hittable>, depth: usize) -> Vector3<f64> {
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                return emitted
                    + attenuation.zip_map(&color(&scattered, &world, depth + 1), |l, r| l * r);
            }
        }
        // Vector3::new(0.0, 0.0, 0.0)
        emitted
    } else {
        // let unit_direction = ray.direction().normalize();
        // let t = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        Vector3::new(0.0, 0.0, 0.0)
    }
}
