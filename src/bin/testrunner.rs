use svg_maker::{
    Shape, Svg,
    color::Color,
    element::Element,
    shapes::path::Path,
    style::{LineCap, LineJoin},
    units::{Percent, Px},
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

    let v = [100, 200, 130, 400];
    let _ = barchart(&v).debug(0);
}

fn barchart(values: &[i32]) -> Svg {
    let len = values.len();
    const SIZE: u32 = 400;
    let max_bar_width = 100;
    let padding = Percent(2);
    let mut s = Svg::new();
    let mut paths = vec![];
    for (i, v) in values.iter().enumerate() {
        let p = Path::new()
            .into_element()
            .class("hover")
            .stroke(Color::Red)
            .stroke_width(2)
            .stroke_dasharray(vec![Percent(9), 3])
            .stroke_linejoin(LineJoin::Round)
            .move_to((i as f64 + 25.) + (i * 75) as f64, 400)
            .vertical_line_relative(-1 * v)
            .horizontal_line_relative(50)
            .vertical_line_relative(*v as u32);

        paths.push(p);
    }
    s.css(
        r#"
        path.hover {
            fill: black;
            transition: fill 0.3s ease-in-out;
        }
        path.hover:hover {
            fill: red;
        }
    "#,
    )
    .version("2")
    .push_vec(paths)
    .viewbox(0, 0, 400, 400)
}
