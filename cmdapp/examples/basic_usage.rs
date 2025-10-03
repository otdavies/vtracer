// Example demonstrating basic usage of the vtracer library

use vtracer::{convert, ColorImage, Config, Preset};

fn main() {
    // Create a simple test image (10x10 pixels, red square)
    let width = 10;
    let height = 10;
    let mut pixels = Vec::new();
    
    for _y in 0..height {
        for _x in 0..width {
            pixels.push(255); // R
            pixels.push(0);   // G
            pixels.push(0);   // B
            pixels.push(255); // A
        }
    }
    
    let img = ColorImage {
        pixels,
        width,
        height,
    };
    
    // Use a preset configuration
    let config = Config::from_preset(Preset::Poster);
    
    // Convert the image to SVG
    match convert(img, config) {
        Ok(svg) => {
            println!("Successfully converted image to SVG!");
            println!("SVG output:\n{}", svg);
        }
        Err(e) => {
            eprintln!("Error converting image: {}", e);
        }
    }
}
