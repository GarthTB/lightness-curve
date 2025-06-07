use crate::config::Config;
use crate::image_utils;
use anyhow::Error;
use std::time::Instant;

pub(crate) fn run() -> Result<f32, Error> {
    let time = Instant::now();
    println!("载入配置文件...");
    let config = Config::load()?;
    println!("载入成功！");
    let image_paths = config.get_ordered_image_paths()?;
    println!("共{}张图像。开始计算...", image_paths.len());
    let values = image_utils::get_mean_vals(&config, image_paths)?;
    println!("计算完成！");
    config.output_values(values)?;
    println!("输出完成！");
    Ok(time.elapsed().as_secs_f32())
}
