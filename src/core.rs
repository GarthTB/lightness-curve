use crate::config::Config;
use crate::image_utils;
use anyhow::Error;
use std::path::PathBuf;
use std::time::Instant;

pub(crate) fn run() -> Result<f32, Error> {
    let time = Instant::now();
    println!("载入配置文件...");
    let config = Config::load()?;
    println!("载入成功！");
    let input_paths = config.get_ordered_input_paths()?;
    let lightness_values = match input_paths.len() {
        1 => get_vid_vals(&config, &input_paths[0])?,
        _ => image_utils::get_mean_vals(&config, input_paths)?,
    };
    println!("计算完成！");
    config.output_values(lightness_values)?;
    println!("输出完成！");
    Ok(time.elapsed().as_secs_f32())
}

fn get_vid_vals(config: &Config, video_path: &PathBuf) -> Result<Vec<f32>, Error> {
    todo!()
}
