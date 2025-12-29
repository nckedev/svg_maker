use svg_maker::{
    Shape, Svg,
    color::Color,
    element::Element,
    shapes::path::Path,
    style::{LineCap, LineJoin},
    units::{Percent, Px},
    visit::Visit,
};

fn main() {
    let e = Element::line(10, 10, 100, 100);

    let s = Svg::new()
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

    let v = [100, 200, 130, 350];
    println!(
        "{}",
        barchart(&v, BarChartOpts::default(), &Theme::default()).render()
    );
    let _ = barchart(&v, BarChartOpts::default(), &Theme::default()).debug(1);
}

struct Theme {
    primary: Color,
    secondary: Color,
    neutral: Color,
    text: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::CssVar("--primary".to_string()),
            secondary: Color::CssVar("--secondary".to_string()),
            neutral: Color::Black,
            text: Color::Black,
        }
    }
}

struct BarChartOpts {
    corner_radius_y: u32,
    corner_radius_x: u32,
    outline: bool,
}

impl Default for BarChartOpts {
    fn default() -> Self {
        Self {
            corner_radius_y: 20,
            corner_radius_x: 10,
            outline: true,
        }
    }
}

fn barchart(values: &[i32], opts: BarChartOpts, theme: &Theme) -> Svg {
    let len = values.len();
    const SIZE: u32 = 400;
    let max_bar_width = 100;
    let padding = Percent(2);
    let mut s = Svg::new();
    let mut paths = vec![];
    for (i, v) in values.iter().enumerate() {
        let start = (i as f64 + 25.) + (i as f64 * 75.);
        let height = v - 20;
        let p = Path::new()
            .into_element()
            .class("hover")
            .stroke_width(2)
            // .stroke_dasharray(&[Percent(9), 2])
            .stroke_linejoin(LineJoin::Round)
            .move_to(start, 400)
            .vertical_line_relative(-1 * height)
            .cubic_bezier_relative((0, -20), (0, -20), (20, -20))
            .horizontal_line_relative(10)
            .cubic_bezier_relative((20, 0), (20, 0), (20, 20))
            .vertical_line_relative(height as u32);

        paths.push(p);
    }
    s.css(&{
        let primary = theme.primary.visit_return();
        format!(
            r#"
        :root {{
            --primary_hue: {primary};
            --secondary_hue: 24;
            --stroke_hue: 0;
            --lightness: 50%;
            --chroma: 0.2;
            --primary: oklch(var(--lightness) var(--chroma) var(--primary_hue));
            --secondary: oklch(var(--lightness) var(--chroma) var(--secondary_hue));
            --stroke: oklch(100% 0 0);
            --black: oklch(40% 0.13 80);
        }}
        path.hover {{
            fill: var(--primary);
            stroke: var(--stroke);
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
    .viewbox(0, 0, 400, 400)
}
