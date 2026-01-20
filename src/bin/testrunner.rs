#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;

use svg_maker::{
    Parent,
    color::{Color, Oklch},
    element::{Element, Transform},
    shapes::{group::Group, svg::Svg, text::Text},
    units::{AlignAspectRatio, MeetOrSlice, Percent, TextAnchor},
    visit::Visit,
};

fn main() {
    let e = Element::line(10, 10, 100, 100);
    let c = Element::circle(1, 1, 1);

    let s = Element::svg()
        .viewbox(0, 0, 400, 400)
        .css(
            r#"
            svg { 
                fill: black;
            }
            path {
                fill: red; 
            } 
            "#,
        )
        // .def(
        //     Element::path()
        //         .move_to(10, 400)
        //         .line_relative(0, -300)
        //         .line_relative(50, 0)
        //         .line_relative(0, 300)
        //         .id("new")
        //         .stroke(Color::Red)
        //         .stroke_width(4)
        //         .stroke_linejoin(LineJoin::Round)
        //         .fill(Color::Black),
        // )
        // .def(Element::line(12, 23, 123, 12).id("id"))
        .push(
            Element::use_href(1, 2)
                .height(400)
                .fill(Color::Black)
                .href("new"),
        );
    // .push(
    //     Element::line(Px(1.) + Percent(2) + Px(3.) + Px(2.), Percent(1), 100, 100)
    //         .id("myid")
    //         .stroke_width(Percent(50))
    //         .hx_ext()
    //         .hx_sse_connect("test")
    //         .stroke(Color::Black)
    //         .stroke_linecap(LineCap::Butt)
    //         .fill(Color::Red),
    // );

    let v = vec![
        BarchartValue::new("Mån", 100.),
        BarchartValue::new("Tis", 5.),
        BarchartValue::new("Ons", 112.),
        BarchartValue::new("Tors", 350.),
        BarchartValue::new("Fre", 501.),
    ];
    let opts = BarChartOpts {
        width: 400.,
        height: 400.,
        ..Default::default()
    };
    println!(
        "barchart: \n {}",
        barchart("barcharts", &v, &opts, &Theme::default())
            .unwrap()
            .render(None)
    );

    let _ = barchart("barcharts", &v, &opts, &Theme::default())
        .unwrap()
        .debug(1);
}

pub(crate) struct Theme {
    pub(crate) palette: Palette,
    profile: Profile,
    font: String,
    stroke: Color,
}

impl Default for Theme {
    fn default() -> Self {
        let lightness = 0.75;
        let chroma = 0.15;
        let neutral_variance = 0.15;
        let primary_hue_variance = 10;
        let a = [2, 2, 2];
        let b = a[1];

        let neutral_base = Oklch::new(0.60, 0., 60);
        let neutrals = Oklch::generate_from_with_lightness::<3>(neutral_base, 0.15);
        let primary_base = Oklch::new(lightness, chroma, 60);
        let primaries = Oklch::generate_from_with_hue::<3>(primary_base, 10);

        Self {
            palette: Palette {
                black: Color::Black,
                neutral1: neutrals[0].into(),
                neutral2: neutrals[1].into(),
                neutral3: neutrals[2].into(),
                white: Color::White,
                primary: primaries[0].into(),
                seconday: Oklch::new(lightness, chroma, 225).into(),
                good: Oklch::new(lightness, chroma, 150).into(),
                bad: Oklch::new(lightness, chroma, 30).into(),
            },
            profile: Profile::SoftEdges,
            font: "".to_string(),
            stroke: Color::White,
        }
    }
}

pub(crate) struct Palette {
    pub(crate) black: Color,
    pub(crate) neutral1: Color,
    pub(crate) neutral2: Color,
    pub(crate) neutral3: Color,
    pub(crate) white: Color,

    pub(crate) primary: Color,
    pub(crate) seconday: Color,
    pub(crate) good: Color,
    pub(crate) bad: Color,
}

enum Profile {
    HardEdges,
    SoftEdges,
}

struct BarChartOpts {
    corner_radius_y: u32,
    corner_radius_x: u32,
    good_threshold: Option<f64>,
    bad_threshold: Option<f64>,
    height: f64,
    width: f64,
    legend_pos: LegendPos,
    show_axis_line: bool,
    show_labels: bool,
    show_title: bool,
    min_y_label_value: f64,
}

impl Default for BarChartOpts {
    fn default() -> Self {
        Self {
            corner_radius_y: 20,
            corner_radius_x: 10,
            good_threshold: None,
            bad_threshold: None,
            height: 100.,
            width: 100.,
            legend_pos: LegendPos::default(),
            show_axis_line: true,
            show_labels: true,
            show_title: true,
            min_y_label_value: 0_f64,
        }
    }
}

#[derive(Default)]
pub enum LegendPos {
    Left,
    Right,
    #[default]
    None,
}

struct Size {
    w: f64,
    h: f64,
}

impl From<(f64, f64)> for Size {
    fn from(value: (f64, f64)) -> Self {
        Self {
            w: value.0,
            h: value.1,
        }
    }
}

struct BarchartValue {
    value: f64,
    label: String,
    order: Option<u32>,
}
impl BarchartValue {
    fn new(label: &str, value: impl Into<f64>) -> Self {
        Self {
            value: value.into(),
            label: label.to_string(),
            order: None,
        }
    }
}

fn barchart(
    title: &str,
    values: &[BarchartValue],
    opts: &BarChartOpts,
    theme: &Theme,
) -> Result<Element<Svg>, Box<dyn Error>> {
    let padding = 20.;
    let availible_height = opts.height - (padding * 2.);
    let availible_width = opts.width - (padding * 2.);

    let title_height = 20.;
    let title_offset_y = padding;
    let bars_offset_y = padding + if opts.show_title { title_height } else { 0. };
    let bars_offset_x = padding;
    let bars_width = opts.width - (padding * 2.);
    let bars_height =
        opts.height - (padding * 2.) - if opts.show_title { title_height } else { 0. };

    let css = &{
        let primary = theme.palette.primary.visit_return();
        let secondary = theme.palette.seconday.visit_return();
        let neutral = theme.palette.neutral2.visit_return();
        format!(
            r#"
        :root {{
              font-family: Inter, sans-serif;
              font-feature-settings: 'liga' 1, 'calt' 1; /* fix for Chrome */
            --primary_hue: {primary};
            --secondary_hue: 24;
            --stroke_hue: 0;
            --lightness: 50%;
            --chroma: 0.2;
            --primary: {primary};
            --secondary: {secondary};
            --stroke: {neutral};
            --black: oklch(40% 0.13 80);
        }}
        @supports (font-variation-settings: normal) {{
          :root {{ font-family: InterVariable, sans-serif; }}
        }}
        svg {{
            border: 2px solid red;
        }}
        path.hover {{
            fill: var(--primary);
            // stroke: var(--stroke);
            transition: fill 0.3s ease-in-out;
        }}
        path.hover:hover {{
            fill: var(--secondary);
        }}
        text {{
            font-size: 24px;
            fill: White;
            stroke: White;
        }}

        text.label_x {{
            font-size: 12px;
            font-weight: 100;
            stroke-width: 1;
        }}
    "#
        )
    };

    let title = generate_title("title", (100., 10.).into());
    let bars = generate_bars(values, bars_width, bars_height);

    let s = Element::svg()
        .css(css)
        .version("2")
        .push_if(
            opts.show_axis_line,
            Element::group().id("lines").push(
                Element::path()
                    .move_to(0, 0)
                    .vertical_line_relative(360)
                    .horizontal_line_relative(360)
                    .fill(Color::Transparent)
                    .stroke(Color::White)
                    .transform(Transform::Translate(20., 20.)),
            ),
        )
        .push(
            Element::foreign_object(20, 10)
                .id("div")
                .width(30)
                .height(20)
                .push("<div style=\"background: red\">test</div>".to_string()),
        )
        .push(
            title
                .transform(Transform::Translate(0., title_offset_y))
                .stroke(Color::White),
        )
        .push(
            bars.id("bars")
                .transform(Transform::Translate(bars_offset_x, bars_offset_y)),
        )
        // .size(400, 400)
        .preserv_aspect_ratio(AlignAspectRatio::XMidYMid, MeetOrSlice::Meet)
        .viewbox(0, 0, 400, 400);

    Ok(s)
}

fn generate_title(title: &str, size: Size) -> Element<Text> {
    Element::text(Percent(50), 10)
        .id("title")
        .stroke_linejoin(svg_maker::style::LineJoin::Miter)
        .stroke_miterlimit(0.)
        .text_anchor(TextAnchor::Middle)
        .push("My awsome chart")
}

fn generate_bars(values: &[BarchartValue], w: f64, h: f64) -> Element<Group> {
    // height of the highest bar
    let max = {
        let max_entry = values.iter().max_by(|a, b| a.value.total_cmp(&b.value));
        if let Some(max) = max_entry {
            max.value
        } else {
            0.
        }
    };

    // set the scale factor if the the highest (max) bar is higher than the
    // avalible height (h)
    let max_barvalue =
        find_closest_above(&values.iter().map(|b| b.value).collect::<Vec<_>>(), 500.);
    let scale_factor = if max > h { h / max_barvalue } else { 1. };
    let len = values.len();
    let spacing = {
        let total_width = (len * 50) as f64;
        let remaining_space = w - total_width;
        remaining_space / (len - 1) as f64
    };
    let offset_x = 20.; // radius of the corner
    let offset_y = 20.; // radius of the corner
    let topbar = 10.; // width of the top of the bar that isnt curved 

    let mut paths = vec![];

    for (i, v) in values.iter().enumerate() {
        let offset_y = if offset_y > v.value * scale_factor {
            0.
        } else {
            offset_y
        };

        let bar_height = (v.value * scale_factor) - offset_x;
        eprintln!("max {} , barh: {}", max_barvalue, bar_height);
        let p = Element::group()
            .push(
                Element::path()
                    .id(&format!("bar{}", i))
                    .class("hover")
                    // .stroke_dasharray(&[Percent(9), 2])
                    // .move_to(0. + ((spacing + (offset_x * 2. + topbar)) * i as f64), h)
                    .move_to(0, 0)
                    .vertical_line_relative(-bar_height)
                    .cubic_bezier_relative((0., -offset_y), (0., -offset_y), (offset_x, -offset_y))
                    .horizontal_line_relative(topbar)
                    .cubic_bezier_relative((offset_x, 0.), (offset_x, 0.), (offset_x, offset_y))
                    .vertical_line_relative(bar_height),
            )
            .push(
                Element::text(offset_x + (topbar / 2.), 15)
                    .class("label_x")
                    .stroke_linejoin(svg_maker::style::LineJoin::Round)
                    .text_anchor(TextAnchor::Middle)
                    .stroke(Color::White)
                    // PERF: clone here...
                    .push(v.label.clone()),
            )
            .transform(Transform::Translate(
                0. + ((spacing + (offset_x * 2. + topbar)) * i as f64),
                h,
            ));

        paths.push(p);
    }

    Element::group().push_iter(paths)
}

/// finds the closest whole number above the max value of the array.
/// ie 972 => 1000, 11 => 20
/// return the biggest number of the calculated number above and min.
fn find_closest_above(values: &[f64], min: f64) -> f64 {
    let Some(max) = values.iter().max_by(|a, b| a.total_cmp(b)) else {
        return 0.0;
    };

    if *max <= 0.0 {
        return 0.0;
    }
    let exp = max.log10().floor();
    let base = 10_f64.powf(exp);
    let contender = (max / base).ceil() * base;
    if contender > min { contender } else { min }
}

fn fmt_with_suffix(value: f64, decimals: usize) -> String {
    const K: f64 = 1e3;
    const M: f64 = 1e6;
    const B: f64 = 1e9;
    const T: f64 = 1e12;

    let value = value.abs();

    if value >= T {
        format!("{:.*}T", decimals, value / T)
    } else if value >= B {
        format!("{:.*}B", decimals, value / B)
    } else if value >= M {
        format!("{:.*}M", decimals, value / M)
    } else if value >= K {
        format!("{:.*}K", decimals, value / K)
    } else {
        format!("{value}")
    }
}

#[cfg(test)]
mod tests {
    use crate::fmt_with_suffix;

    use super::*;

    #[test]
    fn test_fmt_with_suffix() {
        let a = 1001.;
        let a = find_closest_above(&[2000_f64], 0.);
        let b = fmt_with_suffix(a, 0);
        assert_eq!(b, "2K".to_owned());
    }
}
