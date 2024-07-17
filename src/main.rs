use ray::Ray;
use vec3::Vec3;
use color::Color;

mod vec3;
mod color;
mod ray;

fn main() {
    std::process::exit(real_main());
}

fn ray_color (ray: &Ray) -> Color {

    let unit_dir = Vec3::unit_vector(*ray.get_direction());
    let a = 0.5 * (unit_dir.y() + 1.0);
    let return_color = (1.0 - a) * Color::new_with_values(1.0, 1.0, 1.0) + a * Color::new_with_values(0.5, 0.7, 1.0);
    return_color
}


fn real_main() -> i32 {

    // env_logger::init();

    // let img_height = 256;
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let img_height = (img_width as f64/ aspect_ratio) as i64;
    let img_height = if img_height > 1 { img_height } else { 1 };

    // Setup the viewPort
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ( img_width as f64 / img_height as f64 );
    let focal_length = 1.0;
    let camera_center = vec3::Point3::new();

    let viewport_u = Vec3::new_with_values( viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_with_values(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / viewport_width;
    let pixel_delta_v = viewport_v / viewport_height;

    let viewport_upper_bound = camera_center - Vec3::new_with_values(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_bound + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n256", img_width, img_height);

    for j in 0..img_height {
        // eprintln!("\rScanning line {}", img_height - j);
        // io::stdout().flush().unwrap();
        for i in 0..img_width {

            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_dir);

            let pixel_color = ray_color(&r);
            color::print_color(pixel_color);
        }
    }
        // eprintln!("\rDone ..................................\n");
 0   
}