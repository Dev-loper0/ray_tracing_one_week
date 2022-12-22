fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {} \n255\n", image_width, image_height);

    let mut j: i32 = image_height;

    while j >= 0 {
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width - 1) as f32;
            let g: f32 = j as f32 / (image_height - 1) as f32;
            let b: f32 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }

        j -= 1;
    }
}
