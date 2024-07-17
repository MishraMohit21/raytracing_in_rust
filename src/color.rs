use crate::vec3;


pub type Color = vec3::Vec3;

pub fn print_color(pixel_color: Color) {

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let ir = (255.999 * r) as i64;
    let ig = (255.999 * g) as i64;
    let ib = (255.999 * b) as i64;


    println!("{} {} {}", ir, ig, ib);

}
