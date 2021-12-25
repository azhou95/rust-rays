mod vec3;
mod ray;

use vec3::Vec3;
use vec3::Point3;
use crate::ray::Ray;
use crate::vec3::Color;

fn hit_sphere(centre: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin - centre;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discr = b*b - 4.0*a*c;
    if discr < 0.0 {
        return -1.0
    } else {
        return (-b - discr.sqrt() ) / (2.0 * a);
    }
}

fn ray_colour(ray: Ray) -> Vec3 {
    let t = hit_sphere(Point3{x: 0.0, y: 0.0, z: -1.0}, 0.5, &ray);

    if t > 0.0 {
        let n: Vec3 = ray.at(t) - Vec3{x: 0.0, y: 0.0, z: -1.0};
        0.5 * Vec3{x: n.x + 1.0, y: n.y + 1.0, z: n.z + 1.0}
    } else {
        let unit_direction: Vec3 = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0-t) * vec3::Color{x: 1.0, y: 1.0, z: 1.0} + t * vec3::Color{x: 0.5, y: 0.7, z: 1.0}
        }
    }

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: f64 = 400 as f64;
    let image_height: f64 = image_width as f64 / aspect_ratio;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};


    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height as i32).rev() {
        eprintln!("\rScanlines remaining {}", j);
        for i in 0..image_width as i32 {
            let u = i as f64 / (image_width - 1.0) as f64;
            let v = j as f64 / (image_height - 1.0) as f64;

            let r = ray::Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin
            };
            let pixel_colour = ray_colour(r);
            println!("{}", pixel_colour.to_rgb())
        }
    };

    eprintln!("\nDone.\n");
}
