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
    time0: f64,
    time1: f64,
) -> Camera {
    Camera::new(
        look_from,
        look_at,
        view_up,
        vertical_fov,
        nx as f64 / ny as f64,
        aperture,
        focus_dist,
        time0,
        time1,
    )
}

// #[test]
// trait objects without an explicit `dyn` are deprecated
// accepted in rust 2018 but hard erroin in rust 2021!
pub fn random_scene() -> Box<dyn Hittable> {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = CheckerTexture::new(
        SolidTexture::new(0.2, 0.3, 0.1),
        SolidTexture::new(0.9, 0.9, 0.9),
    );
    world.push(Rc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(checker),
    )));

    for a in -10..10 {
        for b in -10..10 {
            let choose_material = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vector3::new(0.0, 0.5 * rng.gen::<f64>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::new(SolidTexture::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    )));
                } else if choose_material < 0.95 {
                    // metal
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            SolidTexture::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    )));
                } else {
                    // glass
                    world.push(Rc::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }
    world.push(Rc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    )));
    world.push(Rc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(SolidTexture::new(0.4, 0.2, 0.1)),
    )));
    world.push(Rc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(SolidTexture::new(0.7, 0.6, 0.5), 0.0),
    )));
    Box::new(BVHNode::new(&mut world, 0.0, 1.0))
}

// #[test]
pub fn two_spheres() -> Box<dyn Hittable> {
    let checker = CheckerTexture::new(
        SolidTexture::new(0.2, 0.3, 0.1),
        SolidTexture::new(0.9, 0.9, 0.9),
    );
    let mut world = HittableList::default();
    world.push(Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    ));
    world.push(Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));
    Box::new(world)
}

// #[test]
pub fn two_perlin_sphere() -> Box<dyn Hittable> {
    let noise = NoiseTexture::new(4.0);
    let mut world = HittableList::default();
    world.push(Sphere::new(
        Vector3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(noise.clone()),
    ));
    world.push(Sphere::new(
        Vector3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(noise),
    ));
    Box::new(world)
}
