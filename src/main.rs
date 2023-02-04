use std::io::{self, Write};

use raytracer::color;
use raytracer::vec3::Vec3;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stdout().flush().expect("flusing stdout");
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            color::write_color(&mut io::stdout(), &Vec3::new(r, g, b));
        }
    }

    eprintln!("\nDone.");
}
