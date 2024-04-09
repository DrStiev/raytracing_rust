use std::fmt;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

// define base structure
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3], // declare an array of f64 with length 3
}

pub type Point3 = Vec3;

// implement method that will be used afterwards
impl Vec3 {
    // constructor with parameters
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    // constructor with no parameters called 'zero'
    pub fn new_empty() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    // get basic attributes of 3D-vector (x, y and z)
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // obtain length of vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        // use a more functional like style instead of a more dichiarative one
        // self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
        self.e.iter().map(|x| x * x).sum()
    }

    // return if a value is near zero
    pub fn near_zero(&self) -> bool {
        let s = f64::EPSILON;
        self.e.iter().all(|&x| x.abs() < s)
    }

    // get random number within range and not
    pub fn random() -> Self {
        Vec3::new(rand::random(), rand::random(), rand::random())
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Vec3::new(
            min + (max - min) * rand::random::<f64>(),
            min + (max - min) * rand::random::<f64>(),
            min + (max - min) * rand::random::<f64>(),
        )
    }
}

// ridefinizione operatori
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self / v.e[0], self / v.e[1], self / v.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] = self.e[0] + rhs.e[0];
        self.e[1] = self.e[1] + rhs.e[1];
        self.e[2] = self.e[2] + rhs.e[2];
    }
}

// ridefinizione operatore di stampa
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// funzioni d'appoggio
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e.iter().zip(v.e.iter()).map(|(u, v)| u * v).sum()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    }
    return -on_unit_sphere;
}
