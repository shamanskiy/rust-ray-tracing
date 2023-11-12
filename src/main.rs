use std::io;

use image::{ImageBuffer, Rgba, RgbaImage};

fn main() -> io::Result<()> {
    let img: RgbaImage = generate_image();

    img.save("test.png")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

fn generate_image() -> RgbaImage {
    let mut img: RgbaImage = ImageBuffer::new(4, 4);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let red: u8 = if (x + y) % 4 == 3 { 255 } else { 0 };
        let green: u8 = if x % 2 == 1 { 255 } else { 0 };
        let blue: u8 = if y % 2 == 1 { 255 } else { 0 };
        *pixel = Rgba([red, green, blue, 255]); // Red color in RGBA format
    }
    return img;
}
