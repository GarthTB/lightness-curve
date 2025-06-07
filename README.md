# 📸 Lightness Curve 图像亮度变化测量工具 📊

[![README English](https://img.shields.io/badge/README-English-blue)](https://github.com/GarthTB/lightness-curve/blob/master/README_en.md)
[![用前必读 中文](https://img.shields.io/badge/用前必读-中文-red)](https://github.com/GarthTB/lightness-curve/blob/master/README.md)
[![开发语言 Rust](https://img.shields.io/badge/开发语言-Rust-brown)](https://www.rust-lang.org)
[![最新版本 0.1.0](https://img.shields.io/badge/最新版本-0.1.0-brightgreen)](https://github.com/GarthTB/lightness-curve/releases/latest)
[![开源许可 Apache 2.0](https://img.shields.io/badge/开源许可-Apache%202.0-royalblue)](https://www.apache.org/licenses/LICENSE-2.0)

用Rust编写的轻量级高性能工具，用于分析图像序列中特定区域或全图的亮度变化，支持多种色彩空间指标，结果可导出为CSV数据或SVG折线图。

## 功能特性

- ⚡ 超高性能：使用并行处理技术，一分钟内处理上千张ZIP压缩的4K8位TIFF图像
- 📦 零运行时依赖：静态编译的单一可执行文件
- 🖼️ 广泛格式支持：JPEG、PNG、BMP、TIFF、WebP等
- 🔍 ROI支持：可分析特定区域或整张图像
- 🎚️ 多指标支持：
    - RGB平均值 (RGB Avg)
    - 视觉加权亮度 (Luminance)
    - 单独通道 (R, G, B)
    - HSV空间 (Hue, Saturation, Value)
- 📊 灵活输出：CSV表格或SVG矢量图表
- 🤖 自动化友好：无交互界面，全配置文件驱动

## 快速开始

### 1. 获取程序

从[Release页面](https://github.com/GarthTB/lightness-curve/releases/latest)下载预编译二进制文件

或使用Cargo编译：

```bash
cargo install lightness-curve
```

### 2. 配置文件

在程序同目录创建`config.toml`文件，完整配置示例如下：

```toml
image_dir = "C:/test/input_images"  # 待测图像所在文件夹
order_by = 0  # 输入图像的重排序方式；0：文件名，1：创建时间，2：修改时间
descending = false  # 是否按降序重排序
mode = 0  # 待测指标；0：RGB总和，1：RGB视觉加权明度，2：R，3：G，4：B，5：H，6：S，7：V
# top_left_x = 0  # ROI的左上角x坐标；若无需截取ROI，可注释掉
# top_left_y = 0  # ROI的左上角y坐标；若无需截取ROI，可注释掉
# width = 0  # ROI的宽度；若无需截取ROI，可注释掉
# height = 0  # ROI的高度；若无需截取ROI，可注释掉
output_data_path = "C:/test/output_data.csv"  # 输出数据文件的路径；若无需输出数据文件，可注释掉
output_plot_path = "C:/test/output_plot.svg"  # 输出折线图的路径；若无需输出折线图，可注释掉
```

### 3. 运行程序

```bash
./lightness-curve
```

## 性能测试

测试环境：Intel(R) Core(TM) i5-12500H 2.50 GHz / 16GB RAM / Windows 11 26100.4061

图像信息：3840x2160，8位RGB 3通道 TIFF，ZIP压缩，每张图像约38MB

| 图像数量 |   ROI尺寸   | 待测指标  | 处理时间（秒） |
|:----:|:---------:|:-----:|:-------:|
| 100  |    全图     | RGB总和 |  3.28   |
| 100  |    全图     |   H   |  3.35   |
| 100  | 1920x1080 | RGB总和 |  3.21   |
| 624  |    全图     | RGB总和 |  22.70  |
| 624  | 1920x1080 | RGB总和 |  23.67  |

## 注意事项

- 输出SVG折线图时，纵坐标的精度无法调节，只能显示至小数点后一位
- 不能处理RAW格式图像，需先转换为常用格式

## 更新日志

### v0.1.0 (2025-06-07)

- 初始版本
