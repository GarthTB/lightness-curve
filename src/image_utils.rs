use crate::config::Config;
use anyhow::{Context, Error, anyhow};
use image::DynamicImage::*;
use rayon::prelude::*;
use std::path::PathBuf;

pub(crate) fn get_mean_vals(config: &Config, image_paths: Vec<PathBuf>) -> Result<Vec<f32>, Error> {
    println!("计算{}张图像中，请稍候...", image_paths.len());
    image_paths
        .par_iter()
        .map(|path| get_mean_val(config, path))
        .collect()
}

fn get_mean_val(config: &Config, image_path: &PathBuf) -> Result<f32, Error> {
    let image = image::open(image_path).context("无法打开图像")?;
    match config.get_image_roi(image) {
        ImageLuma8(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf.iter().map(|&p| p as f32 / 255.0).sum();
            Ok(sum / buf.len() as f32)
        }
        ImageLumaA8(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(2)
                .map(|p| p[0] as f32 / 255.0 * p[1] as f32 / 255.0)
                .sum();
            Ok(sum / (buf.len() / 2) as f32)
        }
        ImageRgb8(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(3)
                .map(|p| {
                    let r = p[0] as f32 / 255.0;
                    let g = p[1] as f32 / 255.0;
                    let b = p[2] as f32 / 255.0;
                    config.rgb_to_lightness(r, g, b)
                })
                .sum();
            Ok(sum / (buf.len() / 3) as f32)
        }
        ImageRgba8(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(4)
                .map(|p| {
                    let r = p[0] as f32 / 255.0;
                    let g = p[1] as f32 / 255.0;
                    let b = p[2] as f32 / 255.0;
                    config.rgb_to_lightness(r, g, b) * p[3] as f32 / 255.0
                })
                .sum();
            Ok(sum / (buf.len() / 4) as f32)
        }
        ImageLuma16(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf.iter().map(|&p| p as f32 / 65535.0).sum();
            Ok(sum / buf.len() as f32)
        }
        ImageLumaA16(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(2)
                .map(|p| p[0] as f32 / 65535.0 * p[1] as f32 / 65535.0)
                .sum();
            Ok(sum / (buf.len() / 2) as f32)
        }
        ImageRgb16(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(3)
                .map(|p| {
                    let r = p[0] as f32 / 65535.0;
                    let g = p[1] as f32 / 65535.0;
                    let b = p[2] as f32 / 65535.0;
                    config.rgb_to_lightness(r, g, b)
                })
                .sum();
            Ok(sum / (buf.len() / 3) as f32)
        }
        ImageRgba16(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(4)
                .map(|p| {
                    let r = p[0] as f32 / 65535.0;
                    let g = p[1] as f32 / 65535.0;
                    let b = p[2] as f32 / 65535.0;
                    config.rgb_to_lightness(r, g, b) * p[3] as f32 / 65535.0
                })
                .sum();
            Ok(sum / (buf.len() / 4) as f32)
        }
        ImageRgb32F(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(3)
                .map(|p| config.rgb_to_lightness(p[0], p[1], p[2]))
                .sum();
            Ok(sum / (buf.len() / 3) as f32)
        }
        ImageRgba32F(ref roi) => {
            let buf = roi.as_raw();
            let sum: f32 = buf
                .chunks_exact(4)
                .map(|p| config.rgb_to_lightness(p[0], p[1], p[2]) * p[3])
                .sum();
            Ok(sum / (buf.len() / 4) as f32)
        }
        _ => Err(anyhow!("不支持此图像格式")),
    }
}
