use std::io::Write;

use crate::rtweekend;
use crate::vec3::Color;

pub fn write_color(write: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    writeln!(
        write,
        "{} {} {}",
        (256.0 * rtweekend::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * rtweekend::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}
