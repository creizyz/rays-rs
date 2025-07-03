use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let image_width : isize = 256;
    let image_height : isize = 256;

    let mut file : File = File::create("./image.ppm")?;

    write!(file, "P3\n")?;
    write!(file, "{} {}\n", image_width, image_height)?;
    write!(file, "255\n")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let r : f64 = i as f64 / (image_width - 1) as f64;
            let g : f64 = j as f64 / (image_height - 1) as f64;
            let b : f64 = 0.0;

            let ir : u8 = (255.99 * r) as u8;
            let ig : u8 = (255.99 * g) as u8;
            let ib : u8 = (255.99 * b) as u8;

            write!(file, "{} {} {}\n", ir, ig, ib)?;
        }
    }

    Ok(())
}
