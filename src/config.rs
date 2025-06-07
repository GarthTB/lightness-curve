use crate::output_generator;
use anyhow::{Context, Error, anyhow};
use image::DynamicImage;
use std::env::current_exe;
use std::fs::{File, read_to_string, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

/// 从config.toml中读取的配置信息及其相关操作
#[derive(serde::Deserialize)]
pub(crate) struct Config {
    /// 待测图像所在文件夹
    image_dir: String,
    /// 输入图像的重排序方式；0：文件名，1：创建时间，2：修改时间
    order_by: u8,
    /// 是否按降序重排序
    descending: bool,
    /// 待测指标；0：RGB平均值，1：RGB视觉加权明度，2：R，3：G，4：B，5：H，6：S，7：V
    mode: u8,
    /// ROI的左上角x坐标；若无需截取ROI，可为None
    top_left_x: Option<u32>,
    /// ROI的左上角y坐标；若无需截取ROI，可为None
    top_left_y: Option<u32>,
    /// ROI的宽度；若无需截取ROI，可为None
    width: Option<u32>,
    /// ROI的高度；若无需截取ROI，可为None
    height: Option<u32>,
    /// 输出数据文件的路径；若无需输出数据文件，可为None
    output_data_path: Option<String>,
    /// 输出折线图的路径；若无需输出折线图，可为None
    output_plot_path: Option<String>,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let config_path = exe_path.with_file_name("config.toml");
        let config_str = read_to_string(&config_path).context("无法读取配置文件")?;
        toml::from_str(&config_str).context("配置文件格式错误")
    }

    pub(crate) fn get_ordered_image_paths(&self) -> Result<Vec<PathBuf>, Error> {
        let path = Path::new(&self.image_dir);
        if path.is_dir() {
            let read_dir = path.read_dir().context("无法读取图像目录")?;
            let mut paths: Vec<PathBuf> = read_dir
                .filter_map(|entry| match entry {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            Some(entry.path())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect();
            match self.order_by {
                0 => paths.sort_by(|a, b| a.file_name().cmp(&b.file_name())),
                1 => paths.sort_by(|a, b| {
                    let a_time = a.metadata().unwrap().created().unwrap();
                    let b_time = b.metadata().unwrap().created().unwrap();
                    a_time.cmp(&b_time)
                }),
                2 => paths.sort_by(|a, b| {
                    let a_time = a.metadata().unwrap().modified().unwrap();
                    let b_time = b.metadata().unwrap().modified().unwrap();
                    a_time.cmp(&b_time)
                }),
                _ => return Err(anyhow!("未指定有效的重排序方式")),
            }
            if self.descending {
                paths.reverse();
            }
            Ok(paths)
        } else {
            Err(anyhow!("输入路径不是目录"))
        }
    }

    pub(crate) fn get_image_roi(&self, image: DynamicImage) -> DynamicImage {
        match (self.top_left_x, self.top_left_y, self.width, self.height) {
            (Some(x), Some(y), Some(w), Some(h)) => image.crop_imm(x, y, w, h),
            _ => image,
        }
    }

    pub(crate) fn get_target_value(&self, r: f32, g: f32, b: f32) -> f32 {
        match self.mode {
            0 => (r + g + b) / 3.0,
            1 => r * 0.2126729 + g * 0.7151522 + b * 0.0721750,
            2 => r,
            3 => g,
            4 => b,
            5 => {
                let max = r.max(g).max(b);
                let min = r.min(g).min(b);
                let delta = max - min;
                let h = if delta == 0.0 {
                    0.0
                } else if max == r {
                    60.0 * (g - b) / delta
                } else if max == g {
                    60.0 * (b - r) / delta + 120.0
                } else {
                    60.0 * (r - g) / delta + 240.0
                };
                let h = if h < 0.0 { h + 360.0 } else { h };
                h / 360.0
            }
            6 => {
                let max = r.max(g).max(b);
                let min = r.min(g).min(b);
                let delta = max - min;
                if delta == 0.0 { 0.0 } else { delta / max }
            }
            7 => r.max(g).max(b),
            _ => panic!("未指定有效的通道"),
        }
    }

    pub(crate) fn output_values(&self, values: Vec<f32>) -> Result<(), Error> {
        match &self.output_data_path {
            None => println!("未指定输出数据文件路径，无需输出。"),
            Some(path) => {
                println!("输出数据文件...");
                let report = output_generator::gen_report(&values);
                if Path::new(path).exists() {
                    println!("指定的路径存在文件，将覆盖。");
                    remove_file(&path).context("无法删除该同名文件")?;
                }
                File::create(path)
                    .context("无法创建输出数据文件")?
                    .write_all(report.as_bytes())
                    .context("无法写入输出数据文件")?;
            }
        }
        match &self.output_plot_path {
            None => println!("未指定输出折线图路径，无需输出。"),
            Some(path) => {
                println!("输出折线图...");
                let chart = output_generator::gen_chart(values)?;
                if Path::new(path).exists() {
                    println!("指定的路径存在文件，将覆盖。");
                    remove_file(&path).context("无法删除该同名文件")?;
                }
                File::create(path)
                    .context("无法创建输出折线图文件")?
                    .write_all(chart.as_bytes())
                    .context("无法写入输出折线图文件")?;
            }
        }
        Ok(())
    }
}
