use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

// functions to generate random numbers. Not sure if keep them
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
pub fn random_int() -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen::<i64>()
}
pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
