use svg_maker_derive::{BaseStyle, ContainerElement};

use crate::{element::Element, marker_traits::ElementKind, visit::Visit};

#[derive(Default, ContainerElement, BaseStyle)]
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
