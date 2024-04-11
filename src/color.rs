use crate::{Interval, Vec3};
use std::io::Write;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color<W: Write>(
    mut out: W,
    pixel_color: Color,
    samples_per_pixel: usize,
) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Write the translated [0,255] value of each color component.
    let intensity = Interval::with_values(0.0, 0.999);
    write!(
        out,
        "{} {} {}\n",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    )
}
