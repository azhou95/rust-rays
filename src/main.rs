mod vec3;
use vec3::Vec3;

fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let v = Vec3::new(
                r, g, b
            );
            println!("{}", v.to_rgb())
        }
    }

    eprintln!("\nDone.\n");
}
