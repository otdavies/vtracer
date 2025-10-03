<div align="center">

  <img src="docs/images/visioncortex-banner.png">
  <h1>VTracer</h1>

  <p>
    <strong>Rust Library for Raster to Vector Graphics Conversion</strong>
  </p>

  <h3>
    <a href="https://www.visioncortex.org/vtracer-docs">Article</a>
  </h3>

  <sub>Built with 🦀 by <a href="https://www.visioncortex.org/">The Vision Cortex Research Group</a></sub>
</div>

## Introduction

visioncortex VTracer is an open source software to convert raster images (like jpg & png) into vector graphics (svg). It can vectorize graphics and photographs and trace the curves to output compact vector files.

Comparing to [Potrace](http://potrace.sourceforge.net/) which only accept binarized inputs (Black & White pixmap), VTracer has an image processing pipeline which can handle colored high resolution scans. tl;dr: Potrace uses a `O(n^2)` fitting algorithm, whereas `vtracer` is entirely `O(n)`.

Comparing to Adobe Illustrator's [Image Trace](https://helpx.adobe.com/illustrator/using/image-trace.html), VTracer's output is much more compact (less shapes) as we adopt a stacking strategy and avoid producing shapes with holes.

VTracer is originally designed for processing high resolution scans of historic blueprints up to gigapixels. At the same time, VTracer can also handle low resolution pixel art, simulating `image-rendering: pixelated` for retro game artworks.

Technical descriptions of the [tracing algorithm](https://www.visioncortex.org/vtracer-docs) and [clustering algorithm](https://www.visioncortex.org/impression-docs).

## Library Usage

This is a pure Rust library for converting raster images into vector graphics. It can be used in Rust applications and compiles to WASM without any environment variable dependencies.

### Installation

Add `vtracer` to your `Cargo.toml`:

```toml
[dependencies]
vtracer = { path = "cmdapp" }
```

### Basic Example

```rust
use vtracer::{convert, ColorImage, Config, Preset};

fn main() {
    // Create an image
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
            println!("SVG output:\n{}", svg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

### Configuration Options

The library provides three preset configurations:

- `Preset::Bw` - For binary (black and white) images
- `Preset::Poster` - For color images with moderate detail
- `Preset::Photo` - For photographs with high detail

You can also create custom configurations using the `Config` struct:

```rust
use vtracer::{Config, ColorMode, Hierarchical};
use visioncortex::PathSimplifyMode;

let config = Config {
    color_mode: ColorMode::Color,
    hierarchical: Hierarchical::Stacked,
    filter_speckle: 4,
    color_precision: 6,
    layer_difference: 16,
    mode: PathSimplifyMode::Spline,
    corner_threshold: 60,
    length_threshold: 4.0,
    max_iterations: 10,
    splice_threshold: 45,
    path_precision: Some(2),
};
```

### WASM Compatibility

This library has no environment variable dependencies and can be compiled to WebAssembly:

```sh
cargo build --target wasm32-unknown-unknown
```

## In the wild

VTracer is used by the following products (open a PR to add yours):

<table>
  <tbody>
    <tr>
      <td><a href="https://logo.aliyun.com/logo#/name"><img src="docs/images/aliyun-logo.png" width="250"/></a>
      <br>Smart logo design
      </td>
      <td></td>
    </tr>
  </tbody>
</table>

## Citations

VTracer has since been cited by a few academic papers in computer graphics / vision research. Please kindly let us know if you have cited our work:

+ SKILL 2023 [Framework to Vectorize Digital Artworks for Physical Fabrication based on Geometric Stylization Techniques](https://www.researchgate.net/publication/374448489_Framework_to_Vectorize_Digital_Artworks_for_Physical_Fabrication_based_on_Geometric_Stylization_Techniques)
+ arXiv 2023 [Image Vectorization: a Review](https://arxiv.org/abs/2306.06441)
+ arXiv 2023 [StarVector: Generating Scalable Vector Graphics Code from Images](https://arxiv.org/abs/2312.11556)
+ arXiv 2024 [Text-Based Reasoning About Vector Graphics](https://arxiv.org/abs/2404.06479)
+ arXiv 2024 [Delving into LLMs' visual understanding ability using SVG to bridge image and text](https://openreview.net/pdf?id=pwlm6Po61I)

## How did VTracer come about?

> The following content is an excerpt from my [unpublished memoir](https://github.com/visioncortex/memoir).

At my teenage, two open source projects in the vector graphics space inspired me the most: Potrace and Anti-Grain Geometry (AGG).

Many years later, in 2020, I was developing a video processing engine. And it became evident that it requires way more investment to be commercially viable. So before abandoning the project, I wanted to publish *something* as open-source for posterity. At that time, I already developed a prototype vector graphics tracer. It can convert high-resolution scans of hand-drawn blueprints into vectors. But it can only process black and white images, and can only output polygons, not splines.

The plan was to fully develop the vectorizer: to handle color images and output splines. I recruited a very talented intern, [@shpun817](https://github.com/shpun817), to work on VTracer. I grafted the frontend of the video processing engine - the ["The Clustering Algorithm"](https://www.visioncortex.org/impression-docs#the-clustering-algorithm) as the pre-processor.

Three months later, we published the first version on Reddit. Out of my surprise, the response of such an underwhelming project was overwhelming.

## What's next?

There are several things in my mind:

1. Path simplification. Implement a post-process filter to the output paths to further reduce the number of splines.

2. Perfect cut-out mode. Right now in cut-out mode, the shapes do not share boundaries perfectly, but have seams.

3. Pencil tracing. Instead of tracing shapes as closed paths, may be we can attempt to skeletonize the shapes as open paths. The output would be clean, fixed width strokes.

4. Image cleaning. Right now the tracer works best on losslessly compressed pngs. If an image suffered from jpeg noises, it could impact the tracing quality. We might be able to develop a pre-filtering pass that denoises the input.

If you are interested in working on them or willing to sponsor its development, feel free to get in touch.
