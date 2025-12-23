use svg_maker_derive::*;

use crate::{buffer::Buffer, element::Element, units::Length, visit::Visit};

#[derive(Default, BaseStyle, OpenEndedShape, Hx)]
pub struct Line {
    x1: Length,
    y1: Length,
    x2: Length,
    y2: Length,
}

impl Line {
    pub fn new(
        x1: impl Into<Length>,
        y1: impl Into<Length>,
        x2: impl Into<Length>,
        y2: impl Into<Length>,
    ) -> Self {
        Self {
            x1: x1.into(),
            y1: y1.into(),
            x2: x2.into(),
            y2: y2.into(),
        }
    }
}

impl Element<Line> {
    pub fn line<X, Y, W, H>(x: X, y: Y, w: W, h: H) -> Element<Line>
    where
        X: Into<Length>,
        Y: Into<Length>,
        W: Into<Length>,
        H: Into<Length>,
    {
        let l = Line::new(x.into(), y.into(), w.into(), h.into());
        Element::new(l)
    }
}
impl Visit for Line {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_tag("line");
        buffer.push_attr("x1", &self.x1);
        buffer.push_attr("y1", &self.y1);
        buffer.push_attr("x2", &self.x2);
        buffer.push_attr("y2", &self.y2);
    }
}
