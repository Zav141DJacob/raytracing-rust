#![feature(let_chains)]

pub mod camera;
pub mod color;
pub mod config;
pub mod cube;
pub mod flags;
pub mod hit;
pub mod material;
pub mod plane_surf;
pub mod ray;
pub mod sphere;
pub mod vec3;

use clap::Parser;
use color::Color;
use hit::{Hittable, HittableList};
use material::scatter;
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

use crate::flags::Flags;

fn color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        if depth < 50 && let Some((attenuation, scattered)) = scatter(&rec.material, r, &rec) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

const MAX_RGB_VALUE: u8 = 255; // Max value in RGB colours (0...255)

fn main() {
    let flags = Flags::parse();
    if !flags.config.exists() || !flags.config.is_file() {
        eprintln!("Please choose a valid file");
        return;
    }

    let app = flags.get_application().expect("Failed to parse config");

    /* Main setup */
    let width = app.width; // Picture width
    let height = app.height; // Picture height
    let samples = app.samples; // Nr of samples - higher nr will give better picture quality
    let max_val = 255; // Max value in RGB colours (0...255)
    let light = app.brightness; // Light level in the world (0...100)

    let mut rng = rand::thread_rng();
    let brightness = if light > 0 && light <= 100 {
        light as f64 / 100.0
    } else {
        1.0
    };

    let debug_pad = app.camera.height.to_string().len();

    println!("P3\n{} {}\n{MAX_RGB_VALUE}", app.camera.width, app.camera.height);

    for j in (0..app.camera.height).rev() {
        eprint!("\rScanlines remaining: {j: <debug_pad$}");

        for i in 0..app.camera.width {
            let mut col: Color = (0..samples)
                .map(|_| {
                    let u = (i as f64 + rng.gen::<f64>()) / app.camera.width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / app.camera.height as f64;
                    let r = &app.camera.get_ray(u, v);

                    color(r, &app.world, 1)
                })
                .sum();

            col /= samples as f64;
            col = brightness * col;

            let adjust = |f: f64| (255.99 * f) as i32;

            let ir = adjust(col.r());
            let ig = adjust(col.g());
            let ib = adjust(col.b());

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\nDone!");
}
