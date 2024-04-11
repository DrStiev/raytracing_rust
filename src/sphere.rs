use std::sync::Arc;

use crate::{util::*, vec3::*, HitRecord, Hittable, Interval, Material, Point3, Ray, AABB, ONB};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec: Vec3 = Vec3::new(radius, radius, radius);
        Self {
            center: center,
            radius: radius,
            mat: material,
            is_moving: false,
            center_vec: Vec3::new_empty(),
            bbox: AABB::new_with_point(center - rvec, center + rvec),
        }
    }
    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let rvec: Vec3 = Vec3::new(radius, radius, radius);
        let box1: AABB = AABB::new_with_point(center1 - rvec, center1 + rvec);
        let box2: AABB = AABB::new_with_point(center2 - rvec, center2 + rvec);

        Self {
            center: center1,
            radius: radius,
            mat: material,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: AABB::new_with_aabb(box1, box2),
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center + time * self.center_vec
    }
    fn set_sphere_uv(p: Point3, mut u: f64, mut v: f64) {
        let theta: f64 = f64::acos(-p.y());
        let phi: f64 = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;

        u = phi / (2.0 * std::f64::consts::PI);
        v = theta / std::f64::consts::PI;
    }
    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1: f64 = random_double();
        let r2: f64 = random_double();
        let z: f64 = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / distance_squared) - 1.0);

        let phi: f64 = 2.0 * std::f64::consts::PI * r1;
        let x: f64 = f64::cos(phi) * f64::sqrt(1.0 - z * z);
        let y: f64 = f64::sin(phi) * f64::sqrt(1.0 - z * z);

        Vec3::new(x, y, z)
    }

    pub fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let mut rec: HitRecord = HitRecord::new();
        if self.hit(
            Ray::with_origin_and_direction(o, v),
            Interval::with_values(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let cos_theta_max: f64 =
            f64::sqrt(1.0 - self.radius * self.radius / (self.center - o).length_squared());
        let solid_angle: f64 = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    pub fn random(&self, o: Point3) -> Vec3 {
        let direction: Vec3 = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw: ONB = ONB::build_from_w(direction);
        uvw.vec3_local(Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: crate::Ray, ray_t: crate::Interval, rec: &mut crate::HitRecord) -> bool {
        let center: Point3 = if self.is_moving {
            self.center(r.time())
        } else {
            self.center
        };
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = f64::sqrt(discriminant);
        let mut root: f64 = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Sphere::set_sphere_uv(outward_normal, rec.u, rec.v);
        rec.mat = Some(self.mat);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
