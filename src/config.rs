use anyhow::{Context, Error};
use std::env::current_exe;
use std::fs::read_to_string;

/// 从config.toml中读取的配置信息及其相关操作
#[derive(serde::Deserialize)]
pub(crate) struct Config {
    /// 输入图像目录或视频文件的绝对路径
    input_path: String,
    /// 输入图像的重排序方式；对视频无效（可为None）；0：文件名，1：创建时间，2：修改时间
    order_by: Option<u8>,
    /// 是否按倒序重排序；不重排序时无效（可为None）
    descending: Option<bool>,
    /// 待测视频片段的起始帧号；对图像无效；若无需截取视频片段，可为None
    start_frame: Option<usize>,
    /// 待测视频片段的帧数；对图像无效；若无需截取视频片段，可为None
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
    /// 输出曲线图像的绝对路径；若无需输出曲线图像，可为None
    output_plot_path: Option<String>,
    /// 输出数据文件的绝对路径；若无需输出数据文件，可为None
    output_data_path: Option<String>,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let config_path = exe_path.with_file_name("config.toml");
        let config_str = read_to_string(&config_path).context("无法读取配置文件")?;
        toml::from_str(&config_str).context("配置文件格式错误")
    }
}
