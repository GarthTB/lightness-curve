use anyhow::{Context, Error};
use charts_rs::{LineChart, Series, THEME_ANT};

pub(crate) fn gen_report(values: &Vec<f32>) -> String {
    values
        .into_iter()
        .enumerate()
        .map(|(i, v)| format!("{}\t{v}", i + 1))
        .collect::<Vec<String>>()
        .join("\n")
}

pub(crate) fn gen_chart(values: Vec<f32>) -> Result<String, Error> {
    let max_value = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_value = values.iter().cloned().fold(f32::INFINITY, f32::min);
    let range = max_value - min_value;
    let width = 1440.max(values.len() * 6).min(8000);

    let indexes: Vec<String> = (1..=values.len()).map(|i| format!("{i}")).collect();
    let series = Series::new("Value".to_string(), values);
    let mut chart = LineChart::new_with_theme(vec![series], indexes, THEME_ANT);

    chart.width = width as f32;
    chart.height = 960.0;
    chart.margin = charts_rs::Box::from((20.0, 12.0, 24.0, 8.0));
    chart.font_family = "Roboto".to_string();
    chart.title_text = "Value Curve".to_string();
    chart.legend_show = Some(false);
    chart.x_boundary_gap = Some(false);
    chart.y_axis_configs[0].axis_max = Some(max_value + range * 0.1);
    chart.y_axis_configs[0].axis_min = Some(min_value - range * 0.1);

    chart.svg().context("无法生成 SVG 图")
}
