// start  working on raytracing porting from c++ to rust
// HELP: https://github.com/fralken/ray-tracing-in-one-weekend

/*
* MAIN IDEA
* logger.rs file to handle writing log on a file
* progressbar.rs to show the progression of the software
* try to use as many external crate as possible to better understand
* the code and algorithm. Then try to optimize the code
*/
#![allow(warnings)]

mod progressbar;
use progressbar::progressbar;

mod vec3;
use vec3::*; // use all functions declared with pub keyword

mod util;
use util::*;

mod logger;
use logger::*;
use LogLevel::*;

use std::{thread, time};

fn main() {
    let e = [1.0, 2.0, 3.0];
    let res: f64 = e.iter().map(|x| x * x).sum();
    print!("e:{:?}, res:{}\n", e, res);

    let x = Vec3::new_empty();
    print!("x: {}\n", x);

    let y = random_cosine_direction();
    print!("x: {}\n", y);

    let mut l = Logger::new();
    print!("l: {}\n", l);

    thread::sleep(time::Duration::from_millis(5000)); // wait 5000 millisec

    l.set_description("Testo lungo lungo lunghissimo");
    print!("l: {}\n", l);

    l.write_to_file("log.log");
    thread::sleep(time::Duration::from_millis(5000)); // wait 5000 millisec
    l.set_level(ERROR);
    print!("l: {}\n", l);
    l.write_to_file("log.log");
    // let total_steps = 100;
    // progressbar(total_steps, "RAY TRACING IN ONE WEEK WITH RUST");
}
