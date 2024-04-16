use crate::perlin::Perlin;
use nalgebra::Vector3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64>;
}

#[derive(Clone)]
pub struct SolidTexture {
    color: Vector3<f64>,
}

impl SolidTexture {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Vector3::new(r, g, b),
        }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vector3<f64>) -> Vector3<f64> {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        Self { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        let sin = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sin < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(&p, 7)))
    }
}
