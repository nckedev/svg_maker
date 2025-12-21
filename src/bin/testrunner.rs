use svg_maker::{
    Line, Shape, Svg,
    color::Color,
    element::ElementBuilder,
    shapes::use_href::Use,
    style::LineCap,
    units::{Percent, Px},
};

fn main() {
    let s = Svg::new()
        .size(100, 100)
        .def(
            Line::new(1, 1, 40, 40)
                .into_element()
                .id("new")
                .stroke(Color::Red),
        )
        .push(Use::new(1, 2).into_element())
        .push(
            ElementBuilder::line(Px(1) + Percent(2) + Px(3) + Px(2), Percent(5), 100, 100)
                .id("myid")
                .stroke(Color::Black)
                .stroke_width(Percent(50))
                .hx_ext()
                .hx_sse_connect("test")
                .stroke_linecap(LineCap::Butt)
                .fill(Color::Red),
        );

    println!("{}", s.render());
    let _ = s.render_to_file("test2.svg");
}
