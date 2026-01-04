extern crate svg_maker;
use svg_maker::element::Element;

#[test]
fn testin() {
    let s = Element::svg().render(None);
    eprintln!("{s}");
    assert!(s.contains("<svg"))
}
