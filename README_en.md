# 📸 Lightness Curve 图像亮度变化测量工具 📊

[![README English](https://img.shields.io/badge/README-English-blue)](https://github.com/GarthTB/lightness-curve/blob/master/README_en.md)
[![用前必读 中文](https://img.shields.io/badge/用前必读-中文-red)](https://github.com/GarthTB/lightness-curve/blob/master/README.md)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-brown)](https://www.rust-lang.org)
[![Latest Release 0.1.0](https://img.shields.io/badge/Latest%20Release-0.1.0-brightgreen)](https://github.com/GarthTB/lightness-curve/releases/latest)
[![License Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-royalblue)](https://www.apache.org/licenses/LICENSE-2.0)

A lightweight, high-performance tool built with Rust for analyzing lightness changes in an ROI or the entire frame
across an image sequence. Supports multiple color space metrics, with results exportable to CSV data or SVG line charts.

## Features

- ⚡️ Superior performance: utilizes parallel processing techniques to process tens of thousands of 4K8-bit TIFF images
  in under a minute
- 📦 Zero-runtime dependencies: a single statically-compiled executable file
- 🖼️ Wide format support: supports JPEG, PNG, BMP, TIFF, and WebP formats
- 🔍 ROI support: can analyze specific regions or the entire image
- 🎚️ Multiple metrics support:
    - RGB average (RGB Avg)
    - Visual weighted luminance (Luminance)
    - Individual channels (R, G, B)
    - HSV color space (Hue, Saturation, Value)
- 📊 Flexible output: outputs CSV tables or SVG vector charts
- 🤖 User-friendly automation: no interactive interface, all configuration driven

## Getting Started

### 1. Get the program

Download the pre-compiled binary from the [Release page](https://github.com/GarthTB/lightness-curve/releases/latest)

Or compile from source using Cargo:

```bash
cargo install lightness-curve
```

### 2. Configuration file

Create a `config.toml` file in the same directory as the program, with the following example configuration:

```toml
image_dir = "C:/test/input_images"  # Directory containing the input images
order_by = 0  # Order of the input images; 0: filename, 1: creation time, 2: modification time
descending = false  # Whether to sort the input images in descending order
mode = 0  # The metric to be analyzed; 0: RGB sum, 1: visual weighted luminance, 2: R, 3: G, 4: B, 5: H, 6: S, 7: V
# top_left_x = 0  # The x coordinate of the top-left corner of the ROI; if no ROI is needed, comment out this line
# top_left_y = 0  # The y coordinate of the top-left corner of the ROI; if no ROI is needed, comment out this line
# width = 0  # The width of the ROI; if no ROI is needed, comment out this line
# height = 0  # The height of the ROI; if no ROI is needed, comment out this line
output_data_path = "C:/test/output_data.csv"  # The path of the output data file; if no data file is needed, comment out this line
output_plot_path = "C:/test/output_plot.svg"  # The path of the output plot file; if no plot is needed, comment out this line
```

### 3. Run the program

```bash
./lightness-curve
```

## Performance Testing

Testing environment: Intel(R) Core(TM) i5-12500H 2.50 GHz / 16GB RAM / Windows 11 26100.4061

Image information: 3840x2160, 8-bit RGB 3-channel TIFF, ZIP compressed, each image is around 38MB

| Image count |  ROI size  | Metric  | Processing time (seconds) |
|:-----------:|:----------:|:-------:|:-------------------------:|
|     100     | Full image | RGB sum |           3.28            |
|     100     | Full image |    H    |           3.35            |
|     100     | 1920x1080  | RGB sum |           3.21            |
|     624     | Full image | RGB sum |           22.70           |
|     624     | 1920x1080  | RGB sum |           23.67           |

## Notes

- The precision of the y-axis of the SVG plot cannot be adjusted, and it only displays to one decimal place
- Cannot process RAW format images, must be converted to common formats first

## Changelog

### v0.1.0 (2025-06-07)

- Initial release
