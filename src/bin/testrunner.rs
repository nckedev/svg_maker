#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;

use svg_maker::{
    Parent, Shape,
    color::{Color, Oklch},
    element::{Element, Transform},
    shapes::{path::Path, svg::Svg},
    units::{AlignAspectRatio, MeetOrSlice},
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

    let v = [100, 100, 130, 350, 40];
    println!(
        "barchart: \n {}",
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
    let padding = 20;
    let availible_height = 400 - (padding * 2);
    let availible_width = 400 - (padding * 2);
    let start_x = padding;
    let start_y = 400 - padding;
    let scale_factor = 1;
    // TODO: check if any bar is higher tham availible height, if so scale them donw with some
    // factor
    let spacing = {
        let total_width = len * 50;
        let remaining_space = availible_width - total_width;
        remaining_space / (len - 1)
    };
    let mut paths = vec![];
    let offset = 20;
    let topbar = 10; // width of the top of the bar that isnt curved 
    for (i, v) in values.iter().enumerate() {
        let bar_height = v - offset;
        let p = Path::new()
            .into_element()
            .id(&format!("bar{}", i))
            .class("hover")
            // .stroke_dasharray(&[Percent(9), 2])
            .move_to(
                start_x + ((spacing + (offset * 2 + topbar) as u32) * i as u32),
                start_y,
            )
            .vertical_line_relative(-bar_height)
            .cubic_bezier_relative((0, -offset), (0, -offset), (offset, -offset))
            .horizontal_line_relative(topbar)
            .cubic_bezier_relative((offset, 0), (offset, 0), (offset, offset))
            .vertical_line_relative(bar_height);

        paths.push(p);
    }
    let css = &{
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
        svg {{
            background: red;
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
    };

    let s = Element::svg()
        .css(css)
        .version("2")
        .push(
            Element::group()
                .id("bars")
                .push_iter(paths)
                .transform(Transform::Translate(60., 0.)),
        )
        .size(400, 400)
        .preserv_aspect_ratio(AlignAspectRatio::XMidYMid, MeetOrSlice::Meet)
        .viewbox(0, 0, 400, 400);

    Ok(s)
}
