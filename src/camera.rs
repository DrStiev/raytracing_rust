use nalgebra::Vector3;
use rand::Rng;
use std::f64;

use crate::ray::Ray;
use crate::util::random_in_unit_disk;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    time0: f64,
    time1: f64,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        view_up: Vector3<f64>,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = vertical_fov * f64::consts::PI / 180.0;
        let half_height = focus_dist * f64::tan(theta / 2.0);
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);

        Self {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - focus_dist * w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            u,
            v,
            time0,
            time1,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let origin = if self.lens_radius == 0.0 {
            self.origin
        } else {
            let rd = self.lens_radius * random_in_unit_disk();
            let offset = self.u * rd.x + self.v * rd.y;
            self.origin + offset
        };
        let time = self.time0 + rand::thread_rng().gen::<f64>() * (self.time1 - self.time0);
        Ray::new(
            origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - origin,
            time,
        )
    }
}
