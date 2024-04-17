use raytracing_in_rust::*;

use std::fmt::Write;
use std::fs::File;
use std::io::{self, Write as OtherWrite};
use std::{thread, time};
extern crate termsize;

use image;
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;
use std::f64;
use std::rc::Rc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::color::color;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidTexture};
use crate::util::{random_in_unit_disk, random_in_unit_sphere};
use crate::{logger::*, progressbar::*, LogLevel::*};

fn set_camera(
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

// trait objects without an explicit `dyn` are deprecated
// accepted in rust 2018 but hard erroin in rust 2021!
fn random_scene() -> Box<dyn Hittable> {
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

fn two_spheres() -> Box<dyn Hittable> {
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

fn two_perlin_sphere() -> Box<dyn Hittable> {
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

fn earth() -> Box<dyn Hittable> {
    let image = image::open("texture/earthmap.jpg")
        .expect("image not found")
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    let earth = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));
    Box::new(earth)
}

#[test]
fn test_random_scene() {
    // let total_steps: usize = 100; // this value is used to set the size of the pb and to make it display gracefully
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");

    let mut rng = rand::thread_rng();
    let log_path = "log/log.log";

    // set logger
    let mut l = Logger::new();
    l.set_level(INFO);
    l.set_description("START WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file(log_path);

    // create  file
    let mut file = File::create("output/random_spheres.ppm").expect("REASON");

    l.set_level(DEBUG);
    l.set_description(&format!("Rendering scene: {}", "output/random_spheres.ppm"));
    let _ = l.write_to_file(log_path);

    // set camera
    let ns = 100;
    let nx = 1280;
    let ny = 720;

    l.set_level(DEBUG);
    l.set_description("Initialize Camera");
    let _ = l.write_to_file(log_path);

    let cam = set_camera(
        nx,
        ny,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.1,
        0.0,
        1.0,
    );

    l.set_level(DEBUG);
    l.set_description("Initialize Scene (or World)");
    let _ = l.write_to_file(log_path);

    // chose which image to render
    let world = random_scene();

    // create the image
    l.set_level(DEBUG);
    l.set_description("Create Image");
    let _ = l.write_to_file(log_path);

    let mut output = String::new();
    write!(output, "P3\n{} {}\n255\n", nx, ny).unwrap();
    // println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f64;
            for c in col.iter_mut() {
                *c = c.sqrt();
            }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            write!(output, "{} {} {}", ir, ig, ib).unwrap();
            // println!("{} {} {}", ir, ig, ib);
        }
    }

    // write content into file
    write!(file, "{}", output).expect("REASON");

    l.set_level(INFO);
    l.set_description("END WORKING WITH RAYTRACING AND RUST!\n");
    let _ = l.write_to_file(log_path);
}

#[test]
fn test_two_sphere() {
    // let total_steps: usize = 100; // this value is used to set the size of the pb and to make it display gracefully
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");

    let mut rng = rand::thread_rng();
    let log_path = "log/log.log";

    // set logger
    let mut l = Logger::new();
    l.set_level(INFO);
    l.set_description("START WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file(log_path);

    // create  file
    let mut file = File::create("output/two_spheres.ppm").expect("REASON");

    l.set_level(DEBUG);
    l.set_description(&format!("Rendering scene: {}", "output/two_spheres.ppm"));
    let _ = l.write_to_file(log_path);

    // set camera
    let ns = 100;
    let nx = 1280;
    let ny = 720;

    l.set_level(DEBUG);
    l.set_description("Initialize Camera");
    let _ = l.write_to_file(log_path);

    let cam = set_camera(
        nx,
        ny,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.1,
        0.0,
        1.0,
    );

    l.set_level(DEBUG);
    l.set_description("Initialize Scene (or World)");
    let _ = l.write_to_file(log_path);

    // chose which image to render
    let world = two_spheres();

    // create the image
    l.set_level(DEBUG);
    l.set_description("Create Image");
    let _ = l.write_to_file(log_path);

    let mut output = String::new();
    write!(output, "P3\n{} {}\n255\n", nx, ny).unwrap();
    // println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f64;
            for c in col.iter_mut() {
                *c = c.sqrt();
            }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            write!(output, "{} {} {}", ir, ig, ib).unwrap();
            // println!("{} {} {}", ir, ig, ib);
        }
    }

    // write content into file
    write!(file, "{}", output).expect("REASON");

    l.set_level(INFO);
    l.set_description("END WORKING WITH RAYTRACING AND RUST!\n");
    let _ = l.write_to_file(log_path);
}

#[test]
fn test_perlin_spheres() {
    // let total_steps: usize = 100; // this value is used to set the size of the pb and to make it display gracefully
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");

    let mut rng = rand::thread_rng();
    let log_path = "log/log.log";

    // set logger
    let mut l = Logger::new();
    l.set_level(INFO);
    l.set_description("START WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file(log_path);

    // create  file
    let mut file = File::create("output/two_perlin_spheres.ppm").expect("REASON");

    l.set_level(DEBUG);
    l.set_description(&format!(
        "Rendering scene: {}",
        "output/two_perlin_spheres.ppm"
    ));
    let _ = l.write_to_file(log_path);

    // set camera
    let ns = 100;
    let nx = 1280;
    let ny = 720;

    l.set_level(DEBUG);
    l.set_description("Initialize Camera");
    let _ = l.write_to_file(log_path);

    let cam = set_camera(
        nx,
        ny,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.1,
        0.0,
        1.0,
    );

    l.set_level(DEBUG);
    l.set_description("Initialize Scene (or World)");
    let _ = l.write_to_file(log_path);

    // chose which image to render
    let world = two_perlin_sphere();

    // create the image
    l.set_level(DEBUG);
    l.set_description("Create Image");
    let _ = l.write_to_file(log_path);

    let mut output = String::new();
    write!(output, "P3\n{} {}\n255\n", nx, ny).unwrap();
    // println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f64;
            for c in col.iter_mut() {
                *c = c.sqrt();
            }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            write!(output, "{} {} {}", ir, ig, ib).unwrap();
            // println!("{} {} {}", ir, ig, ib);
        }
    }

    // write content into file
    write!(file, "{}", output).expect("REASON");

    l.set_level(INFO);
    l.set_description("END WORKING WITH RAYTRACING AND RUST!\n");
    let _ = l.write_to_file(log_path);
}

#[test]
fn test_earth() {
    // let total_steps: usize = 100; // this value is used to set the size of the pb and to make it display gracefully
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");

    let mut rng = rand::thread_rng();
    let log_path = "log/log.log";

    // set logger
    let mut l = Logger::new();
    l.set_level(INFO);
    l.set_description("START WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file(log_path);

    // create  file
    let mut file = File::create("output/earth.ppm").expect("REASON");

    l.set_level(DEBUG);
    l.set_description(&format!("Rendering scene: {}", "output/earth.ppm"));
    let _ = l.write_to_file(log_path);

    // set camera
    let ns = 100;
    let nx = 1280;
    let ny = 720;

    l.set_level(DEBUG);
    l.set_description("Initialize Camera");
    let _ = l.write_to_file(log_path);

    let cam = set_camera(
        nx,
        ny,
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.1,
        0.0,
        1.0,
    );

    l.set_level(DEBUG);
    l.set_description("Initialize Scene (or World)");
    let _ = l.write_to_file(log_path);

    // chose which image to render
    let world = earth();

    // create the image
    l.set_level(DEBUG);
    l.set_description("Create Image");
    let _ = l.write_to_file(log_path);

    let mut output = String::new();
    write!(output, "P3\n{} {}\n255\n", nx, ny).unwrap();
    // println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f64;
            for c in col.iter_mut() {
                *c = c.sqrt();
            }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            write!(output, "{} {} {}", ir, ig, ib).unwrap();
            // println!("{} {} {}", ir, ig, ib);
        }
    }

    // write content into file
    write!(file, "{}", output).expect("REASON");

    l.set_level(INFO);
    l.set_description("END WORKING WITH RAYTRACING AND RUST!\n");
    let _ = l.write_to_file(log_path);
}
