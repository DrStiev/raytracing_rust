use std::fs::File;
use std::io::{self, Write};
use std::sync::Arc;

use crate::{
    color::*, degrees_to_radians, util::*, vec3::*, Color, HitRecord, Hittable, HittablePdf,
    Interval, MixturePdf, Pdf, Point3, Ray, ScatterRecord, Vec3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    background: Color,
    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,

    image_height: usize,
    sqrt_spp: usize,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1920,
            samples_per_pixel: 100,
            max_depth: 50,
            background: Color::new_empty(),
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
            center: Point3::new_empty(),
            pixel00_loc: Point3::new_empty(),
            pixel_delta_u: Vec3::new_empty(),
            pixel_delta_v: Vec3::new_empty(),
            u: Vec3::new_empty(),
            v: Vec3::new_empty(),
            w: Vec3::new_empty(),
            defocus_disk_u: Vec3::new_empty(),
            defocus_disk_v: Vec3::new_empty(),
        }
    }

    pub fn render(&self, file: File, world: impl Hittable, lights: impl Hittable) {
        self.init();
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r: Ray = self.get_ray(i, j, si, sj);
                        pixel_color += self.ray_color(r, self.max_depth, world, lights);
                    }
                }
                write_color(file, pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn init(&self) {
        self.image_height = self.image_width / self.aspect_ratio as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;

        let theta: f64 = degrees_to_radians(self.vfov);
        let h: f64 = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 = viewport_height * self.aspect_ratio;

        self.sqrt_spp = f64::sqrt(self.samples_per_pixel as f64) as usize;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;

        let pixel_delta_u: Vec3 = viewport_u / self.image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / self.image_height as f64;

        let viewport_upper_left: Vec3 =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius: f64 =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: usize, j: usize, si: usize, sj: usize) -> Ray {
        let pixel_center: Vec3 =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square(si, sj);

        let ray_origin: Vec3 = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time: f64 = random_double();

        Ray::with_origin_direction_and_time(ray_origin, ray_direction, ray_time)
    }

    fn pixel_sample_square(&self, si: usize, sj: usize) -> Vec3 {
        let px: f64 = -0.5 + self.recip_sqrt_spp * (sj as f64 + random_double());
        let py: f64 = -0.5 + self.recip_sqrt_spp * (sj as f64 + random_double());
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn pixel_sample_disk(&self, radius: f64) -> Vec3 {
        let p: Vec3 = radius * random_in_unit_disk();
        (p[0] * self.pixel_delta_u) + (p[1] * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(
        &self,
        r: Ray,
        depth: usize,
        world: impl Hittable,
        lights: impl Hittable,
    ) -> Color {
        let mut rec: HitRecord = HitRecord::new();

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if !world.hit(r, Interval::with_values(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut srec: ScatterRecord = ScatterRecord::new();
        let color_from_emission: Color = rec
            .mat
            .expect("REASON") // use to unpack Option type. Should add some significant msg
            .emitted(r, rec, rec.u, rec.v, rec.p);

        if !rec.mat.expect("REASON").scatter(r, rec, &mut srec) {
            return color_from_emission;
        }

        if srec.skip_pdf {
            return srec.attenuation * self.ray_color(srec.skip_pdf_ray, depth - 1, world, lights);
        }

        let light_ptr: Arc<HittablePdf> = Arc::new(HittablePdf::new(lights, rec.p));
        let p: MixturePdf = MixturePdf::new(light_ptr, srec.pdf_ptr.expect("REASON"));

        let scattered: Ray = Ray::with_origin_direction_and_time(rec.p, p.generate(), r.time());
        let pdf_val: f64 = p.value(scattered.direction());

        let scattering_pdf: f64 = rec.mat.expect("REASON").scattering_pdf(r, rec, scattered);

        let sample_color: Color = self.ray_color(scattered, depth - 1, world, lights);
        let color_from_scatter: Color =
            (srec.attenuation * scattering_pdf * sample_color) / pdf_val;

        color_from_emission + color_from_scatter
    }
}
