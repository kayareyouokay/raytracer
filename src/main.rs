mod camera;
mod hittable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittable, HittableList, Sphere};
use material::{Lambertian, Metal};
use ray::Ray;
use std::sync::Arc;
use vec3::{Color, Point3};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // Create a dummy material for the temporary hit record
    let dummy_material = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
    let mut rec = hittable::HitRecord::new(dummy_material.clone());
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // Background gradient
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Camera
    let cam = Camera::new(ASPECT_RATIO);

    // Render
    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if x % 50 == 0 && y == 0 {
            println!("Scanlines remaining: {}", IMAGE_HEIGHT - y);
        }

        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = ((IMAGE_HEIGHT - y - 1) as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            
            let r = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, MAX_DEPTH);
        }

        let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
        let r = (pixel_color.x * scale).sqrt().clamp(0.0, 0.999) * 255.0;
        let g = (pixel_color.y * scale).sqrt().clamp(0.0, 0.999) * 255.0;
        let b = (pixel_color.z * scale).sqrt().clamp(0.0, 0.999) * 255.0;

        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }

    // Save the image
    imgbuf.save("output.png").map_err(|e| Box::new(e) as Box<dyn Error>)?;
    println!("Done! Image saved as output.png");
    
    Ok(())
}
