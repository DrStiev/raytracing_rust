use crate::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    // constructor
    pub fn new() -> Self {
        Ray {
            orig: Point3::new_empty(),
            dir: Vec3::new_empty(),
            tm: 0.0,
        }
    }
    pub fn with_origin_and_direction(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }
    pub fn with_origin_direction_and_time(origin: Point3, direction: Vec3, time:f64) -> Self {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t:f64) -> Point3 {
        self.orig + t * self.dir
    }
}