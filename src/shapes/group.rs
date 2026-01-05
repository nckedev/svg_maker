use svg_maker_derive::BaseStyle;

use crate::{
    element::Element,
    marker_traits::{ChildOf, ElementKind},
    shapes::svg::Svg,
    visit::Visit,
};

impl ChildOf<Svg> for Element<Group> {}

#[derive(Debug, Default, BaseStyle)]
pub struct Group {}

impl Element<Group> {
    pub fn group() -> Self {
        Element::new(Group {})
    }
}

impl ElementKind for Group {
    const TAG: &'static str = "g";
}

impl Visit for Group {
    fn visit(&self, _buffer: &mut crate::buffer::Buffer) {}
}

#[cfg(test)]
mod tests {
    use crate::{Options, Parent};

    use super::*;

    #[test]
    fn group() {
        let e = Element::group()
            .push(Element::rect(1, 2, 3, 4))
            .push(Element::rect(2, 2, 2, 2));
        assert_eq!(e.len(), 2);
        let mut opt = Options::default();
        opt.optimizations.remove_newline = true;
        opt.optimizations.remove_indent = true;
        let rendered = e.render(Some(opt));
        let expected = r#"<g><rect x="1" y="2" width="3" height="4"/><rect x="2" y="2" width="2" height="2"/></g>"#;
        assert_eq!(rendered, expected);
    }
}
