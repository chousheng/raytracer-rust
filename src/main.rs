use std::io::{self, Write};

use raytracer::color;
use raytracer::hittable::HitRecord;
use raytracer::hittable::Hittable;
use raytracer::hittable_list::HittableList;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::unit_vector;
use raytracer::vec3::Color;
use raytracer::vec3::Point3;
use raytracer::vec3::Vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    // World
    let mut world = HittableList::new();
    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.add(sphere1);
    world.add(sphere2);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().expect("flusing stdout");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r, &world);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\nDone.");
}
