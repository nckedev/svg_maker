use std::fmt::Debug;

use crate::{
    buffer::Buffer,
    element::Element,
    marker_traits::{ChildOf, ElementKind},
    shapes::text::LengthAdjust,
    units::Length,
    visit::Visit,
};

//need to implement this manually since String is not an Element<T>
impl ChildOf<Tspan> for String {
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn get_z_index(&self) -> Option<i32> {
        None
    }

    fn get_id(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Default)]
pub struct Tspan {
    x: Vec<Length>,
    y: Vec<Length>,
    dx: Vec<Length>,
    dy: Vec<Length>,
    rotate: Option<Vec<f64>>,
    length_adjust: Option<LengthAdjust>,
    text_length: Option<Length>,
}

impl Element<Tspan> {
    pub fn tspan<X, Y>(x: X, y: Y) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
    {
        let tspan = Tspan {
            x: vec![x.into()],
            y: vec![y.into()],
            ..Default::default()
        };
        Element::new(tspan)
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

impl Visit for Tspan {
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

impl ElementKind for Tspan {
    const TAG: &'static str = "tspan";
}

#[cfg(test)]
mod tests {
    use crate::{Options, Parent};

    use super::*;

    #[test]
    fn tspan() {
        let _t = Element::<Tspan>::tspan(1, 2).push("test".to_string());
        let mut opts = Options::default();
        opts.optimizations.remove_newline = true;
        opts.optimizations.remove_indent = true;
        let expected = r#"<tspan x="1" y="2">test</tspan>"#;
        let rendered = _t.render(Some(opts));
        assert_eq!(rendered, expected);
    }
}
