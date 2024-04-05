use std::cmp::{max, min};

use crate::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AABB {
    // maybe is better to keep the fields private and access them
    // using a dedicated method?
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    // constructors
    pub fn new() -> Self {
        Self {
            x: Interval::new(),
            y: Interval::new(),
            z: Interval::new(),
        }
    }
    pub fn new_with_interval(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Self {
            x: ix,
            y: iy,
            z: iz,
        }
    }
    pub fn new_with_point(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::with_values(f64::min(a.x(), b.x()), f64::max(a.x(), b.x())),
            y: Interval::with_values(f64::min(a.y(), b.y()), f64::max(a.y(), b.y())),
            z: Interval::with_values(f64::min(a.z(), b.z()), f64::max(a.z(), b.z())),
        }
    }
    pub fn new_with_aabb(box0: AABB, box1: AABB) -> Self {
        Self {
            x: Interval::with_interval(box0.x, box1.x),
            y: Interval::with_interval(box0.y, box1.y),
            z: Interval::with_interval(box0.z, box1.z),
        }
    }

    pub fn pad(&self) -> Self {
        let delta: f64 = 0.0001;
        let new_x: Interval = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y: Interval = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z: Interval = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };

        AABB::new_with_interval(new_x, new_y, new_z)
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];
            let t0 = (self.axis(a).min - orig) * inv_d;
            let t1 = (self.axis(a).max - orig) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            ray_t.min = f64::min(t0, ray_t.min);
            ray_t.max = f64::max(t1, ray_t.max);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}

impl std::ops::Add<Vec3> for AABB {
    type Output = Self;

    fn add(self, offset: Vec3) -> Self::Output {
        Self::new_with_interval(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl std::ops::Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, bbox: AABB) -> Self::Output {
        bbox + self
    }
}
