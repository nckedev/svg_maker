use svg_maker::{
    Svg,
    color::Color,
    element::Element,
    shapes::use_href::Use,
    style::LineCap,
    units::{Percent, Px},
};

fn main() {
    let s = Svg::new()
        .size(100, 100)
        .css(
            r#"
            path {
                fill: black; 
            } 
            "#,
        )
        .def(
            Element::line(Px(1), 3, 49, Percent(30))
                .id("new")
                .stroke(Color::Red),
        )
        .push(Use::make_element(1, 2).fill(Color::Black).href("test"))
        .push(
            Element::line(Px(1) + Percent(2) + Px(3) + Px(2), Percent(5), 100, 100)
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
