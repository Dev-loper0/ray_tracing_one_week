mod color;
mod vec3;

use crate::color::*;
use crate::vec3::*;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {} \n255\n", image_width, image_height);

    let mut j: i32 = image_height;

    while j >= 0 {
        for i in 0..image_width {
            let pixel_color: Color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(pixel_color);
        }

        j -= 1;
    }

    //let mut v: Vec3 = Vec3::new(1, 2, 3);
    //let mut u: Vec3 = Vec3::new(1, 2, 3);

    //v.op_add(&mut u);

    //println!(" v is {:#?}", v);

    //let mut x: Vec3 = Vec3::add(&u, &v);

    //println!(" x is {:#?}", x);
}
