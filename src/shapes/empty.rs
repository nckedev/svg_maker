use crate::{ElementKind, element::Element, visit::Visit};

pub struct Empty;

impl Element<Empty> {
    pub fn empty() -> Self {
        Element::new(Empty)
    }
}

impl ElementKind for Empty {
    const TAG: &'static str = "EMPTY";
}

impl Visit for Empty {
    fn visit(&self, _buffer: &mut crate::buffer::Buffer) {
        //
    }
}
