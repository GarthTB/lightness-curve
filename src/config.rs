use crate::output_generator;
use anyhow::{Context, Error, anyhow};
use std::env::current_exe;
use std::fs::{File, read_to_string, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

/// 从config.toml中读取的配置信息及其相关操作
#[derive(serde::Deserialize)]
pub(crate) struct Config {
    /// 输入图像目录或视频文件的绝对路径
    input_path: String,
    /// 输入图像的重排序方式；对视频无效，可为None；0：文件名，1：创建时间，2：修改时间
    order_by: Option<u8>,
    /// 是否按降序重排序；对视频无效，可为None
    descending: Option<bool>,
    /// 被测视频片段的起始帧号；对图像无效；若无需截取视频片段，可为None
    start_frame: Option<usize>,
    /// 被测视频片段的帧数；对图像无效；若无需截取视频片段，可为None
    frame_count: Option<usize>,
    /// 被测通道；0：RGB总和，1：R，2：G，3：B
    channel: u8,
    /// ROI的左上角x坐标；若无需截取ROI，可为None
    top_left_x: Option<usize>,
    /// ROI的左上角y坐标；若无需截取ROI，可为None
    top_left_y: Option<usize>,
    /// ROI的宽度；若无需截取ROI，可为None
    width: Option<usize>,
    /// ROI的高度；若无需截取ROI，可为None
    height: Option<usize>,
    /// 输出数据文件的绝对路径；若无需输出数据文件，可为None
    output_data_path: Option<String>,
    /// 输出折线图的绝对路径；若无需输出折线图，可为None
    output_plot_path: Option<String>,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let config_path = exe_path.with_file_name("config.toml");
        let config_str = read_to_string(&config_path).context("无法读取配置文件")?;
        toml::from_str(&config_str).context("配置文件格式错误")
    }

    pub(crate) fn get_ordered_input_paths(&self) -> Result<Vec<PathBuf>, Error> {
        let path = Path::new(&self.input_path);
        if path.is_file() {
            Ok(vec![path.to_path_buf()])
        } else if path.is_dir() {
            let read_dir = path.read_dir().context("无法读取输入目录")?;
            let mut paths: Vec<PathBuf> = read_dir.map(|entry| entry.unwrap().path()).collect();
            match (self.order_by, self.descending) {
                (Some(order_by), Some(descending)) => {
                    match order_by {
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
                        _ => return Err(anyhow!("未指定有效的排序方式")),
                    }
                    if descending {
                        paths.reverse();
                    }
                    Ok(paths)
                }
                _ => Err(anyhow!("未同时指定排序方式以及是否降序")),
            }
        } else {
            Err(anyhow!("输入路径错误"))
        }
    }

    pub(crate) fn output_values(&self, lightness_values: Vec<f32>) -> Result<(), Error> {
        match &self.output_data_path {
            None => println!("未指定输出数据文件路径，无需输出。"),
            Some(path) => {
                println!("输出数据文件...");
                let report = output_generator::gen_report(&lightness_values);
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
                let chart = output_generator::gen_chart(lightness_values)?;
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
