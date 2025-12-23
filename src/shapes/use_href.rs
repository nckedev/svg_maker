use svg_maker_derive::{BaseStyle, Hx, Shape};

use crate::{
    buffer::Buffer,
    element::Element,
    units::{Length, XCoord, YCoord},
    visit::Visit,
};

#[derive(Default, Hx, Shape, BaseStyle)]
pub struct Use {
    x: XCoord,
    y: YCoord,
    width: Length,
    height: Length,
    href: String,
}

impl Use {
    pub fn make_element(x: impl Into<XCoord>, y: impl Into<YCoord>) -> Element<Self> {
        let s = Self {
            x: x.into(),
            y: y.into(),
            ..Default::default()
        };
        Element::new(s)
    }
}
impl Element<Use> {
    #[must_use]
    pub fn href(mut self, target: &str) -> Self {
        if target.starts_with('#') {
            self.href = target.to_string();
        } else {
            self.href = "#".to_string() + target;
        }
        self
    }

    #[must_use]
    pub fn x<T: Into<XCoord>>(mut self, x: T) -> Self {
        self.x = x.into();
        self
    }

    #[must_use]
    pub fn y<T: Into<YCoord>>(mut self, y: T) -> Self {
        self.y = y.into();
        self
    }

    #[must_use]
    pub fn height<H: Into<Length>>(mut self, height: H) -> Self {
        self.height = height.into();
        self
    }

    #[must_use]
    pub fn width<H: Into<Length>>(mut self, width: H) -> Self {
        self.width = width.into();
        self
    }
}

impl Visit for Use {
    fn visit(&self, buffer: &mut Buffer) {
        debug_assert!(!self.href.is_empty(), "Use without a href is useless");

        if self.href.is_empty() {
            return;
        }
        buffer.push_tag("Use");
        buffer.push_attr("x", &self.x);
        buffer.push_attr("y", &self.y);
        buffer.push_attr("width", &self.width);
        buffer.push_attr("height", &self.height);
        buffer.push_attr("href", &self.href);
    }
}
