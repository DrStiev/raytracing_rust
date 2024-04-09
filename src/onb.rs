use crate::vec3::*;
use std::ops::{Index, IndexMut};

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn f64_local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
    pub fn vec3_local(&self, a: Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    pub fn build_from_w(w: Vec3) -> Self {
        let unit_w: Vec3 = unit_vector(w);
        let a: Vec3 = if f64::abs(unit_w.x()) > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v: Vec3 = unit_vector(cross(&unit_w, &a));
        let u: Vec3 = cross(&unit_w, &v);
        Self {
            axis: [u, v, unit_w],
        }
    }
}

// ridefinizione operatori
impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, i: usize) -> &Vec3 {
        &self.axis[i]
    }
}

impl IndexMut<usize> for ONB {
    fn index_mut(&mut self, i: usize) -> &mut Vec3 {
        &mut self.axis[i]
    }
}
