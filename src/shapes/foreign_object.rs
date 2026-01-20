use crate::{ChildOf, ElementKind, buffer::Buffer, element::Element, units::Length, visit::Visit};

impl ChildOf<ForeignObject> for String {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_z_index(&self) -> Option<i32> {
        Some(0)
    }

    fn get_id(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Default)]
pub struct ForeignObject {
    x: Length,
    y: Length,
    width: Option<Length>,
    height: Option<Length>,
}

impl Element<ForeignObject> {
    pub fn foreign_object<X, Y>(x: X, y: Y) -> Self
    where
        X: Into<Length>,
        Y: Into<Length>,
    {
        let fo = ForeignObject {
            x: x.into(),
            y: y.into(),
            ..Default::default()
        };
        Element::new(fo)
    }

    pub fn width<W>(mut self, width: W) -> Self
    where
        W: Into<Length>,
    {
        self.width = Some(width.into());
        self
    }

    pub fn height<H>(mut self, height: H) -> Self
    where
        H: Into<Length>,
    {
        self.height = Some(height.into());
        self
    }
}

impl ElementKind for ForeignObject {
    const TAG: &'static str = "foreignObject";
}

impl Visit for ForeignObject {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_attr("x", &self.x);
        buffer.push_attr("y", &self.y);
        buffer.push_attr_opt("width", &self.width);
        buffer.push_attr_opt("height", &self.height);
    }
}
#[cfg(test)]
mod tests {
    use crate::Parent;

    use super::*;
    #[test]
    fn foreign_object() {
        let fo = Element::foreign_object(12, 12)
            .push("<div>asdaf</div>".to_string())
            .render(None);
        let expected = r#"<foreignObject x="0" y="0" width="0" height="0"/>"#.to_string() + "\n";
        assert_eq!(fo, expected)
    }
}
