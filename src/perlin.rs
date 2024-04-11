use crate::{util::*, vec3::*, Point3};

pub struct Perlin {
    ranvec: [Vec3; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    pub const POINT_COUNT: usize = 256;
    pub fn new() -> Self {
        let mut ranvec = [Vec3::new_empty(); Perlin::POINT_COUNT];
        for i in 0..Perlin::POINT_COUNT {
            ranvec[i] = unit_vector(Vec3::random_in_range(-1.0, 1.0));
        }
        let mut perm_x = [0; Perlin::POINT_COUNT];
        let mut perm_y = [0; Perlin::POINT_COUNT];
        let mut perm_z = [0; Perlin::POINT_COUNT];

        for i in 0..Perlin::POINT_COUNT {
            perm_x[i] = i;
            perm_y[i] = i;
            perm_z[i] = i;
        }

        Perlin::perlin_generate_perm();
        Perlin::perlin_generate_perm();
        Perlin::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u: f64 = p.x() - f64::floor(p.x());
        let v: f64 = p.y() - f64::floor(p.y());
        let w: f64 = p.z() - f64::floor(p.z());

        let i: usize = (4.0 * p.x()) as usize & 255;
        let j: usize = (4.0 * p.y()) as usize & 255;
        let k: usize = (4.0 * p.z()) as usize & 255;
        let mut c = [[[Vec3::new_empty(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        Perlin::perlin_interpolation(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum: f64 = 0.0;
        let mut temp_p = p.clone();
        let mut weight: f64 = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        f64::abs(accum)
    }

    fn perlin_generate_perm() -> [usize; Perlin::POINT_COUNT] {
        let mut p = [0; Perlin::POINT_COUNT];
        for i in 0..Perlin::POINT_COUNT {
            p[i] = i;
        }
        Perlin::permute(p, Perlin::POINT_COUNT);
        p
    }
    fn permute(mut p: [usize; Perlin::POINT_COUNT], n: usize) {
        for i in n - 1..0 {
            let target: usize = random_double_in_range(0.0, i as f64) as usize;
            let tmp: usize = p[i];
            p[i] = p[target];
            p[target] = tmp
        }
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu: f64 = u * u * (3.0 - 2.0 * u);
        let vv: f64 = v * v * (3.0 - 2.0 * v);
        let ww: f64 = w * w * (3.0 - 2.0 * w);
        let mut accum: f64 = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v: Vec3 = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(&c[i][j][k], &weight_v);
                }
            }
        }
        accum
    }
}
