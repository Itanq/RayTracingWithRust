
extern crate image;
extern crate RayTracingWithRust;

use RayTracingWithRust::ray_tracing::{Vec3, Ray};

fn main() {
    let lower_left_corner = Vec3{ point: (-2.0, -1.0, -1.0) };
    let horizontal = Vec3{ point: (4.0, 0.0, 0.0) };
    let vertical = Vec3{ point: (0.0, 2.0, 0.0) };
    let original = Vec3{ point: (0.0, 0.0, 0.0) };

    let mut img_buf = image::RgbImage::new(512, 512);

    for y in (0..512).rev() {
        for x in 0..512 {
            let u = x as f32 / 512.0;
            let v = y as f32 / 512.0;
            let ray = Ray {
                original: original.clone(),
                direction: lower_left_corner.add(&horizontal.mul(u)).add(&vertical.mul(v))  
            };
            let color = ray.color();
            let mut pixel = img_buf.get_pixel_mut(x, y);
            *pixel = image::Rgb([(color.point.0*255.99) as u8, (color.point.1*255.99) as u8, (color.point.2*255.99) as u8]);  
        }
    }

//    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
//        let u = (512 - x) as f32 / 512.0;
//        let v = y as f32 / 512.0;
//        let ray = Ray{
//            original: original.clone(),
//            direction: lower_left_corner.add(&horizontal.mul(u)).add(&vertical.mul(v))
//        };
//        let color = ray.color();
//        *pixel = image::Rgb([(color.point.0*255.0) as u8, (color.point.1*255.0) as u8, (color.point.2*255.0) as u8]);
//        //println!("r: {}; g: {}; b: {}", (color.point.0*255.0) as u8, (color.point.1*255.0) as u8, (color.point.2*255.0) as u8);
//    }

    img_buf.save("ray_tracing_1.png").unwrap();

}
