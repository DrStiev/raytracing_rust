use super::*;

// #[test]
pub fn set_camera(
    nx: usize,
    ny: usize,
    look_from: Vector3<f64>,
    look_at: Vector3<f64>,
    view_up: Vector3<f64>,
    vertical_fov: f64,
    focus_dist: f64,
    aperture: f64,
) -> Camera {
    Camera::new(
        look_from,
        look_at,
        view_up,
        vertical_fov,
        nx as f64 / ny as f64,
        aperture,
        focus_dist,
    )
}

// #[test]
pub fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HittableList::default();
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vector3::new(0.5, 0.5, 0.5)),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    ));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vector3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world
}
