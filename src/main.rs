use std::io::{self, Write};

use raytracer::camera::Camera;
use raytracer::color;
use raytracer::hittable::HitRecord;
use raytracer::hittable::Hittable;
use raytracer::hittable_list::HittableList;
use raytracer::ray::Ray;
use raytracer::rtweekend::random_double;
use raytracer::sphere::Sphere;
use raytracer::vec3::random_in_unit_sphere;
use raytracer::vec3::unit_vector;
use raytracer::vec3::Color;
use raytracer::vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.add(sphere1);
    world.add(sphere2);

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().expect("flusing stdout");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}
