#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;

use svg_maker::{
    Shape, Svg,
    color::Color,
    element::Element,
    shapes::path::Path,
    style::LineJoin,
    units::{Percent, Px},
    visit::Visit,
};

fn main() {
    let e = Element::line(10, 10, 100, 100);

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
        .def(
            Element::path()
                .move_to(10, 400)
                .line_relative(0, -300)
                .line_relative(50, 0)
                .line_relative(0, 300)
                .id("new")
                .stroke(Color::Red)
                .stroke_width(4)
                .stroke_linejoin(LineJoin::Round)
                .fill(Color::Black),
        )
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

    let v = [100, 200, 130, 350, 40];
    println!(
        "{}",
        barchart(&v, &BarChartOpts::default(), &Theme::default())
            .unwrap()
            .render(None)
    );
    let _ = barchart(&v, &BarChartOpts::default(), &Theme::default())
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
        Self {
            palette: Palette {
                black: Color::Black,
                neutral1: Color::Oklch(0.60, 0., 60),
                neutral2: Color::Oklch(0.75, 0., 60),
                neutral3: Color::Oklch(0.90, 0., 60),
                white: Color::White,
                primary: Color::Oklch(lightness, chroma, 60),
                seconday: Color::Oklch(lightness, chroma, 225),
                good: Color::Oklch(lightness, chroma, 150),
                bad: Color::Oklch(lightness, chroma, 30),
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
        }
    }
}

fn barchart(
    values: &[i32],
    opts: &BarChartOpts,
    theme: &Theme,
) -> Result<Element<Svg>, Box<dyn Error>> {
    let len = values.len() as u32;
    const SIZE: u32 = 400;
    let max_bar_width = 100;
    // if SIZE / len < max_bar_width {
    //     return Err("min bar width".into());
    // }
    let padding = Px::from(20);
    let mut s = Element::svg();
    let mut paths = vec![];
    for (i, v) in values.iter().enumerate() {
        let start = (i as f64 + 25.) + (i as f64 * 75.);
        let height = v - 20;
        let p = Path::new()
            .into_element()
            .id(&format!("bar{}", i))
            .class("hover")
            .stroke_width(2)
            // .stroke_dasharray(&[Percent(9), 2])
            .stroke_linejoin(LineJoin::Round)
            .move_to(start, 400)
            .vertical_line_relative(-height)
            .cubic_bezier_relative((0, -20), (0, -20), (20, -20))
            .horizontal_line_relative(10)
            .cubic_bezier_relative((20, 0), (20, 0), (20, 20))
            .vertical_line_relative(height as u32);

        paths.push(p);
    }
    let s = s
        .css(&{
            let primary = theme.palette.primary.visit_return();
            let secondary = theme.palette.seconday.visit_return();
            let neutral = theme.palette.neutral2.visit_return();
            format!(
                r#"
        :root {{
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
        path.hover {{
            fill: var(--primary);
            // stroke: var(--stroke);
            transition: fill 0.3s ease-in-out;
        }}
        path.hover:hover {{
            fill: var(--secondary);
        }}
    "#
            )
        })
        .version("2")
        .push_vec(paths)
        .size(Percent(20), Percent(20))
        .preserv_aspect_ratio(
            svg_maker::units::AlignAspectRatio::XMidYMid,
            svg_maker::units::MeetOrSlice::Meet,
        )
        .viewbox(0, 0, 400, 400);

    Ok(s)
}
