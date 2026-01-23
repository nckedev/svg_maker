use svg_maker_derive::{BaseStyle, ClosedShape};

use crate::{ElementKind, Shape, element::Element, units::Length, visit::Visit};

#[derive(BaseStyle, ClosedShape, Debug)]
pub struct Ellipse {
    cx: Length,
    cy: Length,
    rx: Length,
    ry: Length,
}

impl ElementKind for Ellipse {
    const TAG: &'static str = "ellipse";
}

impl Visit for Ellipse {
    fn visit(&self, buffer: &mut crate::buffer::Buffer) {
        buffer.push_attr("cx", &self.cx);
        buffer.push_attr("cy", &self.cy);
        buffer.push_attr("rx", &self.rx);
        buffer.push_attr("ry", &self.ry);
    }
}

impl Element<Ellipse> {
    pub fn ellipse<X, Y, RX, RY>(center_x: X, center_y: Y, radius_x: RX, radius_y: RY) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
        RX: Into<Length>,
        RY: Into<Length>,
    {
        Ellipse {
            cx: center_x.into(),
            cy: center_y.into(),
            rx: radius_x.into(),
            ry: radius_y.into(),
        }
        .into_element()
    }

    pub fn radius_y<R: Into<Length>>(mut self, radius: R) -> Self {
        self.ry = radius.into();
        self
    }

    pub fn radius_x<R: Into<Length>>(mut self, radius: R) -> Self {
        self.rx = radius.into();
        self
    }
}
