use std::io::Write;

use crate::vec3::Color3;

pub fn write_color<T: Write>(write: &mut T, pixel_color: &Color3) {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    write!(write, "{} {} {}\n", r, g, b).expect("writing color");
}
