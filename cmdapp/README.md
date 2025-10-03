# VTracer - Rust Library

A pure Rust library for converting raster images into vector graphics (SVG).

## Features

- Convert raster images (ColorImage) to SVG format
- Support for both color and binary images
- Configurable tracing parameters
- No environment variable dependencies
- WASM-compatible (no std::env usage)

## Core API

### Main Function

```rust
pub fn convert(img: ColorImage, config: Config) -> Result<SvgFile, String>
```

Converts an in-memory `ColorImage` into an in-memory `SvgFile`.

### Configuration

Use `Config` struct to control the conversion behavior:

- `color_mode`: Choose between `ColorMode::Color` or `ColorMode::Binary`
- `hierarchical`: Choose between `Hierarchical::Stacked` or `Hierarchical::Cutout`
- `filter_speckle`: Discard patches smaller than X×X pixels
- `color_precision`: Number of significant bits in RGB channels (1-8)
- `layer_difference`: Color difference between gradient layers (0-255)
- `mode`: Path simplification mode (Spline, Polygon, or None)
- `corner_threshold`: Minimum angle in degrees to be considered a corner (0-180)
- `length_threshold`: Maximum segment length (3.5-10.0)
- `max_iterations`: Maximum iterations for curve fitting
- `splice_threshold`: Minimum angle in degrees to splice a spline (0-180)
- `path_precision`: Number of decimal places in SVG path strings

### Presets

Three convenient presets are available:

- `Config::from_preset(Preset::Bw)` - For binary images
- `Config::from_preset(Preset::Poster)` - For color posters
- `Config::from_preset(Preset::Photo)` - For photographs

## Example

See `examples/basic_usage.rs` for a complete example.

```rust
use vtracer::{convert, ColorImage, Config, Preset};

let img = ColorImage {
    pixels: vec![255, 0, 0, 255],  // RGBA pixels
    width: 1,
    height: 1,
};

let config = Config::from_preset(Preset::Poster);
let svg = convert(img, config)?;
println!("{}", svg);
```

## Building

```sh
cargo build --lib
```

## Running Examples

```sh
cargo run --example basic_usage
```
