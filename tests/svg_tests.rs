extern crate svg_maker;
use svg_maker::shapes::svg::Svg;

#[test]
fn testin() {
    let s = Svg::new().render();
    assert!(s.contains("<svg"))
}
