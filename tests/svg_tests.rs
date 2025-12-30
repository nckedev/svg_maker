extern crate svg_maker;
use svg_maker::Svg;
#[test]
fn testin() {
    let s = Svg::new().render();
    assert_eq!(s, "".to_string())
}
