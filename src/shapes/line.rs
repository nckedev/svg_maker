use crate::{BaseStyle, ElementKind, Hx, OpenEndedShape};

use crate::{buffer::Buffer, element::Element, units::Length, visit::Visit};

#[derive(Debug, Default, BaseStyle, OpenEndedShape, Hx)]
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
    pub fn line<X1, Y1, X2, Y2>(x1: X1, y1: Y1, x2: X2, y2: Y2) -> Element<Line>
    where
        X1: Into<Length>,
        Y1: Into<Length>,
        X2: Into<Length>,
        Y2: Into<Length>,
    {
        let l = Line::new(x1.into(), y1.into(), x2.into(), y2.into());
        Element::new(l)
    }
}

impl ElementKind for Line {
    const TAG: &'static str = "line";
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
