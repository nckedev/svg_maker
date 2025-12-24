use std::{error::Error, fs::File, io::Write};

use svg_maker_derive::*;

use crate::{buffer::Buffer, element::Element, marker_traits::BaseElement, visit::Visit};

pub use crate::marker_traits::Shape;

pub mod animations;
mod buffer;
pub mod color;
pub mod element;
mod marker_traits;
pub mod shapes;
pub mod style;
pub mod units;
pub mod visit;

#[derive(BaseStyle)]
pub struct Svg {
    w: u16,
    h: u16,
    viewbox: Option<Viewbox>,
    version: String,
    namespace: String,
    css: Option<String>,
    defs: Vec<Box<dyn BaseElement>>,
    children: Vec<Box<dyn BaseElement>>,
}

impl Svg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, w: u16, h: u16) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn viewbox(mut self, x: u32, y: u32, w: u32, h: u32) -> Self {
        self.viewbox = Some(Viewbox { x, y, w, h });
        self
    }

    pub fn css(mut self, css: &str) -> Self {
        self.css = Some(css.to_string());
        self
    }

    pub fn symbol(mut self) -> Self {
        self
    }

    pub fn def<E, S>(mut self, el: E) -> Self
    where
        E: Into<Element<S>> + BaseElement,
        S: Shape + Sized + Visit + 'static,
    {
        debug_assert!(
            el.get_id().is_some(),
            "a definition is useless without an id"
        );
        self.defs.push(Box::new(el));
        self
    }

    #[must_use]
    pub fn push<E, S>(mut self, el: E) -> Self
    where
        E: Into<Element<S>> + BaseElement,
        S: Shape + Sized + Visit + 'static,
    {
        let e: Element<S> = el.into();
        self.children.push(Box::new(e));
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let mut buffer = Buffer::with_capacity(100);
        buffer.opts.optimizations.remove_unit_for_px = true;
        buffer.push_tag("svg");
        buffer.push_attr("width", &self.w);
        buffer.push_attr("height", &self.h);
        buffer.push_attr_opt("viewbox", &self.viewbox);
        buffer.push_attr("version", &self.version);
        buffer.push_attr("xmlns", &self.namespace);
        buffer.push_tag_end();
        if let Some(css) = &self.css {
            buffer.push_tag("style");
            buffer.push_tag_end();
            buffer.push_str(css);
            buffer.push_tag_close("style");
        }
        if !self.defs.is_empty() {
            buffer.push_tag("defs");
            buffer.push_tag_end();
            for def in &self.defs {
                def.visit(&mut buffer);
            }
            buffer.push_tag_close("defs");
        }
        for element in &self.children {
            element.visit(&mut buffer);
        }
        buffer.push_tag_close("svg");
        eprintln!(" buffer:\n {}", buffer.str());
        buffer.str().to_string()
    }

    pub fn render_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut f = File::create(path)?;
        f.write_all(self.render().as_bytes())?;
        Ok(())
    }

    pub fn get_element_by_id<T: Visit + 'static>(&mut self, id: &str) -> Option<&mut Element<T>> {
        // TODO: include defs here?
        for el in &mut self.children {
            if let Some(el_ref) = el.as_any_mut().downcast_mut::<Element<T>>()
                && Some(id) == el_ref.get_id()
            {
                return Some(el_ref);
            }
        }
        None
    }
}

impl Default for Svg {
    fn default() -> Self {
        Self {
            w: 100,
            h: 100,
            version: "1.1".to_string(),
            namespace: "http://www.w3.org/2000/svg".to_string(),
            viewbox: Some(Viewbox {
                x: 0,
                y: 0,
                w: 100,
                h: 100,
            }),
            css: None,
            defs: Vec::new(),
            children: Vec::new(),
        }
    }
}

struct Viewbox {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Visit for Viewbox {
    fn visit(&self, buffer: &mut Buffer) {
        let Viewbox { x, y, w, h } = self;
        buffer.push_str(&format!("{} {} {} {}", x, y, w, h));
    }
}

#[derive(Default, Debug)]
pub(crate) struct Options {
    pub(crate) invert_y: bool,
    pub(crate) optimizations: Optimizations,
    pub(crate) container_size: f64,
}

#[derive(Default, Debug)]
pub(crate) struct Optimizations {
    pub(crate) remove_unit_for_px: bool,
    pub(crate) remove_newline: bool,
    pub(crate) remove_indent: bool,
}

// Raw ======================================

#[derive(BaseStyle)]
pub struct Raw {
    inner: String,
}

impl Visit for Raw {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.inner);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        shapes::{line::Line, path::Path},
        units::{Percent, Px},
    };

    use super::*;

    #[test]
    fn it_works() {
        let mut s = Svg::new().size(100, 100).push(
            Path::new()
                .into_element()
                .class("testing")
                .class("testing22")
                .transform(element::Transform::Scale(2, 2))
                // .push(Command::MoveTo(Coord(10, 10)))
                .move_to(10, 9)
                .stroke(Color::Red)
                .fill(Color::Black),
        );
        // .push(ElementBuilder::raw("testing"));
        let _ = s.render();
        let x = s.get_element_by_id::<Line>("myid");
        if let Some(x) = x {
            x.style.fill = Some(Color::Black);
        }

        let _ = s.render();

        assert_eq!(6, 4);
    }

    #[test]
    fn test_expr_reducing() {
        let a = Px(2.);
        let b = Px(3.);
        let c = a + b + Percent(4) + 3;
        let mut buf = Buffer::with_capacity(10);
        c.visit(&mut buf);
        eprintln!("expr:{}", buf.str());
        assert_eq!(1, 2);
    }
}
