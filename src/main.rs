mod config;
mod core;
mod image_utils;
mod output_generator;

fn main() {
    println!(
        "欢迎使用 Lightness Curve 图像亮度变化测量工具！\n\
        版本号：0.1.0 (2025-06-07)\n\
        作者：Garth TB <g-art-h@outlook.com>\n\
        仓库地址：https://github.com/GarthTB/lightness-curve"
    );
    match core::run() {
        Ok(t) => println!("程序成功执行完毕！用时：{t} s"),
        Err(e) => println!("程序出错：{e:?}\n\n已中断！"),
    }
}
