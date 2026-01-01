use svg_maker_derive::{BaseStyle, ClosedShape};

use crate::{buffer::Buffer, element::Element, units::Length, visit::Visit};

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

impl Element<Rect> {
    pub fn rect<X, Y, W, H>(x: X, y: Y, w: W, h: H) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
        W: Into<Length>,
        H: Into<Length>,
    {
        let r = Rect {
            x: x.into(),
            y: y.into(),
            width: w.into(),
            height: h.into(),
            rx: None,
            ry: None,
        };
        Element::new(r)
    }

    /// Sets the conrner radius of rx, if ry is not specified it will be the same as rx
    /// if you want to have different radiuses for x and y use [`corner_radius_xy(..)`]
    pub fn corner_radius<R: Into<Length>>(mut self, radius: R) -> Self {
        self.rx = Some(radius.into());
        self
    }

    pub fn corner_radius_xy<Rx: Into<Length>, Ry: Into<Length>>(mut self, rx: Rx, ry: Ry) -> Self {
        self.rx = Some(rx.into());
        self.ry = Some(ry.into());
        self
    }
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

#[cfg(test)]
mod tests {
    use crate::units::{Percent, Px};

    use super::*;

    #[test]
    fn rect() {
        let rect = Element::rect(10, Percent(20), Px(20.), 20)
            .corner_radius(10.)
            .render(None);
        let expected =
            r#"<rect x="10px" y="20%" width="20px" height="20px" rx="10px"/>"#.to_string() + "\n";
        assert_eq!(rect, expected)
    }
}
