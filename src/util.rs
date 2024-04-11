use nalgebra::Vector3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
