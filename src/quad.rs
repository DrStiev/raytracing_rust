use std::sync::Arc;

use crate::{
    random_double, vec3::*, HitRecord, Hittable, HittableList, Interval, Material, Point3, Ray,
    Vec3, AABB,
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
    area: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, m: Arc<dyn Material>) -> Self {
        let n: Vec3 = cross(&u, &v);
        let normal: Vec3 = unit_vector(n);
        let d: f64 = dot(&normal, &q);
        let w: Vec3 = n / dot(&n, &n);
        let area: f64 = n.length();
        let bbox: AABB = AABB::new_with_point(q, q + u + v).pad();
        Self {
            q: q,
            u: u,
            v: v,
            mat: m,
            bbox: bbox,
            normal: normal,
            d: d,
            w: w,
            area: area,
        }
    }

    pub fn is_interior(a: f64, b: f64, mut rec: HitRecord) -> bool {
        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }

    pub fn pdf_value(&self, origin: Point3, v: Vec3) -> f64 {
        let mut rec: HitRecord = HitRecord::new();
        if !self.hit(
            Ray::with_origin_and_direction(origin, v),
            Interval::with_values(0.001, f64::INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared: f64 = rec.t * rec.t * v.length_squared();
        let cosine: f64 = f64::abs(dot(&v, &rec.normal) / v.length());

        distance_squared / (cosine * self.area)
    }

    pub fn random(&self, origin: Point3) -> Vec3 {
        let p = self.q + (random_double() * self.u) + (random_double() * self.v);
        p - origin
    }
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom: f64 = dot(&self.normal, &r.direction());

        if f64::abs(denom) < f64::EPSILON {
            return false;
        }

        let t: f64 = (self.d - dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection: Vec3 = r.at(t);
        let planar_hitpt_vector: Vec3 = intersection - self.q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, *rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat);
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn bbox(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = Arc::new(HittableList::new());

    let min: Point3 = Point3::new(
        f64::min(a.x(), b.x()),
        f64::min(a.y(), b.y()),
        f64::min(a.z(), b.z()),
    );
    let max: Point3 = Point3::new(
        f64::max(a.x(), b.x()),
        f64::max(a.y(), b.y()),
        f64::max(a.z(), b.z()),
    );

    let dx: Vec3 = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy: Vec3 = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz: Vec3 = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat,
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat,
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat,
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat,
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat,
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
    )));

    sides
}
