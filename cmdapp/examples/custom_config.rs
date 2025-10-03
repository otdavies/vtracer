// Example demonstrating custom configuration options

use vtracer::{convert, ColorImage, Config, ColorMode, Hierarchical};
use visioncortex::PathSimplifyMode;

fn main() {
    // Create a test image with multiple colors (20x20 pixels)
    let width = 20;
    let height = 20;
    let mut pixels = Vec::new();
    
    for y in 0..height {
        for _x in 0..width {
            // Create a simple pattern: red top half, blue bottom half
            if y < height / 2 {
                pixels.push(255); // R
                pixels.push(0);   // G
                pixels.push(0);   // B
                pixels.push(255); // A
            } else {
                pixels.push(0);   // R
                pixels.push(0);   // G
                pixels.push(255); // B
                pixels.push(255); // A
            }
        }
    }
    
    let img = ColorImage {
        pixels,
        width,
        height,
    };
    
    // Create a custom configuration
    let config = Config {
        color_mode: ColorMode::Color,
        hierarchical: Hierarchical::Stacked,
        filter_speckle: 4,           // Discard small patches
        color_precision: 6,          // Medium color precision
        layer_difference: 16,        // Moderate gradient layers
        mode: PathSimplifyMode::Spline, // Use spline curves
        corner_threshold: 60,        // Corner detection threshold
        length_threshold: 4.0,       // Segment length threshold
        max_iterations: 10,          // Max curve fitting iterations
        splice_threshold: 45,        // Spline splice threshold
        path_precision: Some(2),     // 2 decimal places in paths
    };
    
    // Convert the image to SVG
    match convert(img, config) {
        Ok(svg) => {
            println!("Successfully converted image to SVG with custom config!");
            println!("\nSVG has {} paths", svg.paths.len());
            println!("Image dimensions: {}x{}", svg.width, svg.height);
            println!("\nFull SVG output:\n{}", svg);
        }
        Err(e) => {
            eprintln!("Error converting image: {}", e);
        }
    }
}
