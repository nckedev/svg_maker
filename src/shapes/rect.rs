use svg_maker_derive::{BaseStyle, ClosedShape};

use crate::{buffer::Buffer, units::Length, visit::Visit};

#[derive(Default, BaseStyle, ClosedShape)]
pub struct Rect {
    x: Length,
    y: Length,
    width: Length,
    height: Length,
    rx: Option<Length>,
    ry: Option<Length>,
    // TODO:
    //pathLenght
}

impl Visit for Rect {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_tag("rect");
        buffer.push_attr("x", &self.x);
        buffer.push_attr("y", &self.y);
        buffer.push_attr("width", &self.width);
        buffer.push_attr("height", &self.height);
        buffer.push_attr_opt("rx", &self.rx);
        buffer.push_attr_opt("ry", &self.ry);
    }
}
