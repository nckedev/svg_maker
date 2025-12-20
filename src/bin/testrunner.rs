use svg_maker::{
    Svg,
    color::Color,
    element::ElementBuilder,
    style::LineCap,
    units::{Percent, Px},
};

fn main() {
    let s = Svg::new().size(100, 100).push(
        ElementBuilder::line(Px(1) + Percent(2) + Px(3) + Px(2), Percent(5), 6, 7)
            .id("myid")
            .stroke_linecap(LineCap::Butt)
            .fill(Color::Red),
    );

    println!("{}", s.render());
}
