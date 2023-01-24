pub mod camera;
pub mod color;
pub mod hit;
pub mod material;
pub mod plane_surf;
pub mod rect;
pub mod cube;
pub mod ray;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use clap::Parser;
use color::Color;
use rand::prelude::*;
use ray::Ray;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use vec3::Vec3;
use sphere::Sphere;
use plane_surf::Plane;
use cube::Cube;
use hit::{Hittable, HittableList};
use material::{scatter, Material};

fn color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Color::default();
        if depth < 50 && scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
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

/// Program that renders 3d objects
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config file to use
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    world: HittableList,
}

fn main() {
    let args = Args::parse();
    if !args.config.exists() || !args.config.is_file() {
        eprintln!("Please choose a valid file");
        return;
    }

    let raw_config = std::fs::read_to_string(args.config).expect("Failed to read file");
    let config: Config = ron::from_str(&raw_config).expect("Invalid config file");

    /* Main setup */
    let width = 240; //Picture width
    let height = 180; //Picture height
    let samples = 100; //Nr of samples - higher nr will give better picture quality
    let max_val = 255; //Max value in RGB colours (0...255)
    let light = 100; //Light level in the world (0...100)

    /* Camera setup */
    let look_from = Vec3::new(1.0, 2.0, 2.0); //Where is camera looking from
    let look_at = Vec3::new(0.0, 0.0, -1.0); //Where camera is pointing to
    let vup = Vec3::new(0.0, 1.0, 0.0); //Camera angle (better to leave as is)
    let vfov = 45.0; //View angle - can be used for zoom (smaller angle = zoomed in)
    let aspect = width as f64 / height as f64; //Ratio of camera picture
    let aperture = 0.1; //Focus depth
    let camera = Camera::new(look_from, look_at, vup, vfov, aspect, aperture);

    /* Objects setup */
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, Material::Lambertian { albedo: Color::new(0.4, 0.4, 1.0) })));
    list.push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Color::new(1.0, 1.0, 1.0) })));
    //list.push(Box::new(Cube::new(Vec3::new(-1.5, 0.0, -1.0), Vec3::new(-0.5, 1.0, 0.0), Material::Lambertian { albedo: Color::new(1.0, 0.1, 0.1) })));
    list.push(Box::new(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, Material::Dielectric { ref_idx: 1.5 } )));
    list.push(Box::new(Plane::new(Vec3::new(0.0, 2.0, -1.0), 0.0, 4.0, 5.0, Material::Lambertian { albedo: Color::new(0.9, 0.8, 0.1) })));
    
    let world = HittableList::new(list);
    let mut rng = rand::thread_rng();
    let brightness = if light > 0 && light <= 100 {
        light as f64 / 100.0
    } else {
        1.0
    };

    let debug_pad = height.to_string().len();

    println!("P3\n{width} {height}\n{max_val}");
    for j in (0..height).rev() {
        eprint!("\rScanlines remaining: {j: <debug_pad$}");
        for i in 0..width {
            let mut col = Color::default();

            for _ in 0..samples {
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;
                let r = &camera.get_ray(u, v);
                col += color(r, &config.world, 1);
            }

            col /= samples as f64;
            col = brightness * Color::new(col.r(), col.g(), col.b());
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\nDone!");
}
