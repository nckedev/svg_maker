// Categories:	Graphics element, Text content element
//
// Permitted content:
// Character data and any number of the following elements, in any order:
// Animation elements
// Descriptive elements
// Text content child elements
// <a>

use crate::{
    buffer::Buffer,
    element::Element,
    impl_childof,
    marker_traits::{ChildOf, ElementKind},
    units::Length,
    visit::Visit,
};

//need to implement this manually since String is not an Element<T>
impl ChildOf<Text> for String {}

#[derive(Debug, Default)]
pub struct Text {
    x: Vec<Length>,
    y: Vec<Length>,
    dx: Vec<Length>,
    dy: Vec<Length>,
    rotate: Option<Vec<f64>>,
    length_adjust: Option<LengthAdjust>,
    text_length: Option<Length>,
}

impl Element<Text> {
    // TODO: a constructor for each xy, xy: lenght, xy: Vec<lenght>, dxy: length, dxy : vec<lenght>
    pub fn text<X, Y>(x: X, y: Y) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
    {
        let text = Text {
            x: vec![x.into()],
            y: vec![y.into()],
            ..Default::default()
        };
        Element::new(text)
    }

    pub fn rotate(mut self, angle: Vec<f64>) -> Self {
        self.rotate = Some(angle);
        self
    }

    pub fn lengt_adjust(mut self, value: LengthAdjust) -> Self {
        self.length_adjust = Some(value);
        self
    }

    pub fn text_length(mut self, value: impl Into<Length>) -> Self {
        self.text_length = Some(value.into());
        self
    }
}

impl Visit for Text {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_attr("x", &self.x);
        buffer.push_attr("y", &self.y);
        if !self.dx.is_empty() {
            buffer.push_attr("dx", &self.dx);
        }
        if !self.dy.is_empty() {
            buffer.push_attr("dy", &self.dy);
        }

        buffer.push_attr_opt("rotate", &self.rotate);
        buffer.push_attr_opt("lenghtAdjust", &self.length_adjust);
        buffer.push_attr_opt("textLenght", &self.text_length);
    }
}

impl ElementKind for Text {
    const TAG: &'static str = "text";
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum LengthAdjust {
    #[default]
    Spacing,
    SpacingAndGlyphs,
}

impl Visit for LengthAdjust {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            LengthAdjust::Spacing => "spacing",
            LengthAdjust::SpacingAndGlyphs => "spacingAndGlyphs",
        };
        buffer.push_str(str);
    }
}

#[cfg(test)]
mod tests {
    use crate::Options;

    use super::*;

    #[test]
    fn text() {
        let _t = Element::text(1, 2).push("test".to_string());
        let mut opts = Options::default();
        opts.optimizations.remove_newline = true;
        opts.optimizations.remove_indent = true;
        let expected = r#"<text x="1" y="2">test</text>"#;
        let rendered = _t.render(Some(opts));
        assert_eq!(rendered, expected);
    }
}
