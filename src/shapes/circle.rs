use svg_maker_derive::*;

use crate::{Shape, element::Element, marker_traits::ElementKind, units::Length, visit::Visit};

#[derive(BaseStyle, Shape)]
pub struct Circle {
    cx: Length,
    cy: Length,
    radius: Length,
}

impl ElementKind for Circle {
    const TAG: &'static str = "circle";
}

impl Element<Circle> {
    pub fn circle<X, Y, R>(center_x: X, center_y: Y, radius: R) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
        R: Into<Length>,
    {
        let c = Circle {
            cx: center_x.into(),
            cy: center_y.into(),
            radius: radius.into(),
        };
        c.into_element()
    }
}

impl Visit for Circle {
    fn visit(&self, buffer: &mut crate::buffer::Buffer) {
        buffer.push_attr("cx", &self.cx);
        buffer.push_attr("cy", &self.cy);
        buffer.push_attr("r", &self.radius);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::{Percent, Px};

    use super::*;

    #[test]
    fn circle() {
        let c = Element::circle(10, Percent(10), Px(10.)).render(None);
        let expected = r#"<circle cx="10" cy="10%" r="10"/>"#.to_string() + "\n";
        assert_eq!(c, expected);
    }
}
