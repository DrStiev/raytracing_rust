// HELP: https://github.com/fralken/ray-tracing-in-one-weekend
// https://github.com/lopossumi/Rust-Output-Image
use std::{thread, time};
extern crate termsize;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod logger;
mod material;
mod progressbar;
mod ray;
mod sphere;
mod util;

// use image;
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
use crate::util::{random_in_unit_disk, random_in_unit_sphere};
use crate::{logger::*, progressbar::*, LogLevel::*};

// #[cfg(test)]
mod test;
use crate::test::{random_scene, set_camera};

// cargo run > image.ppm

fn main() {
    // let total_steps: usize = 100; // this value is used to set the size of the pb and to make it display gracefully
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");

    let mut rng = rand::thread_rng();

    let mut l = Logger::new();
    l.set_level(INFO);
    l.set_description("START WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file("log.log");

    let ns = 100;
    let nx = 1280;
    let ny = 720;

    l.set_level(DEBUG);
    l.set_description("Initialize Camera");
    let _ = l.write_to_file("log.log");

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
    let _ = l.write_to_file("log.log");

    let world = random_scene();
    println!("P3\n{} {}\n255", nx, ny);

    // USE OF RAYON. NOT CONVINCING RIGHT NOW
    // // create the image
    // l.set_level(DEBUG);
    // l.set_description("Create Image exploiting parallelization with rayon");
    // let _ = l.write_to_file("log.log");
    // let image = (0..ny)
    //     .into_par_iter()
    //     .rev()
    //     .flat_map(|y| {
    //         (0..nx)
    //             .flat_map(|x| {
    //                 let col: Vector3<f64> = (0..ns)
    //                     .map(|_| {
    //                         let mut rng = rand::thread_rng();
    //                         let u = (x as f64 + rng.gen::<f64>()) / nx as f64;
    //                         let v = (y as f64 + rng.gen::<f64>()) / ny as f64;
    //                         let ray = cam.get_ray(u, v);
    //                         color(&ray, &world, 0)
    //                     })
    //                     .sum();
    //                 col.iter()
    //                     .map(|c| (255.99 * (c / ns as f64).sqrt().max(0.0).min(1.0)) as u8)
    //                     .collect::<Vec<u8>>()
    //             })
    //             .collect::<Vec<u8>>()
    //     })
    //     .collect::<Vec<u8>>();

    // // save the image to file
    // l.set_level(DEBUG);
    // l.set_description("Save image into a .ppm file");
    // let _ = l.write_to_file("log.log");
    // for col in image.chunks(3) {
    //     println!("{} {} {}", col[0], col[1], col[2]);
    // }

    // create the image
    l.set_level(DEBUG);
    l.set_description("Create Image");
    let _ = l.write_to_file("log.log");
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
            println!("{} {} {}", ir, ig, ib);
        }
    }

    l.set_level(INFO);
    l.set_description("END WORKING WITH RAYTRACING AND RUST!");
    let _ = l.write_to_file("log.log");
}
