// Image handling
use image::GrayImage;
use image::buffer::ConvertBuffer;
use image::imageops::resize;
use image::imageops::FilterType;

// Error handling
use fehler::throws;
type Error = anyhow::Error;

const COLORS: u8 = 5;
const CHUNK: u8 = u8::MAX / COLORS;

#[throws]
fn main() {
    let image = image::open("sickcunt.jpg")?.to_rgba8();
    let gray_image: GrayImage = image.convert();
    let (width, height) = gray_image.dimensions();
    let width = width as f32 / (height as f32 / 500f32);
    let mut scaled = resize(&gray_image, width as _, 500, FilterType::CatmullRom);
    for pixel in scaled.iter_mut() {
        if *pixel < CHUNK {
            *pixel = 0;
        } else {
            for color in (1..COLORS).map(|color| u8::MAX / color).rev() {
                if *pixel < color {
                    *pixel = color;
                    break;
                }
            }
            
        }
    }
    scaled.save("output.png")?;
}
