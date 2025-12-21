use svg_maker_derive::{BaseStyle, Hx, Shape};

use crate::{
    buffer::Buffer,
    marker_traits::{BaseStyle, Hx, Shape},
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
    pub fn new(x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            ..Default::default()
        }
    }

    pub fn x<T: Into<XCoord>>(&mut self, x: T) -> &mut Self {
        self.x = x.into();
        self
    }

    pub fn y<T: Into<YCoord>>(&mut self, y: T) -> &mut Self {
        self.y = y.into();
        self
    }

    pub fn height<H: Into<Length>>(&mut self, height: H) -> &mut Self {
        self.height = height.into();
        self
    }

    pub fn width<H: Into<Length>>(&mut self, width: H) -> &mut Self {
        self.width = width.into();
        self
    }

    pub fn href(&mut self, target: &str) -> &mut Self {
        if target.starts_with('#') {
            self.href = target.to_string();
        } else {
            self.href = "#".to_string() + target;
        }
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
