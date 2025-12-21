use std::{error::Error, fs::File, io::Write};

use svg_maker_derive::*;

use crate::{
    buffer::Buffer,
    element::Element,
    marker_traits::{BaseElement, BaseStyle, ClosedShape, Hx, OpenEndedShape},
    units::{Coord, Length},
    visit::Visit,
};

pub use crate::marker_traits::Shape;

mod buffer;
pub mod color;
pub mod element;
mod marker_traits;
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

    pub fn def(mut self, el: impl BaseElement + 'static) -> Self {
        debug_assert!(
            el.get_id().is_some(),
            "a definition is useless without an id"
        );
        self.defs.push(Box::new(el));
        self
    }

    #[must_use]
    pub fn push(mut self, el: impl BaseElement + 'static) -> Self {
        self.children.push(Box::new(el));
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

// Line ===============================================

#[derive(Default, BaseStyle, OpenEndedShape, Hx)]
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
impl Visit for Line {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_tag("line");
        buffer.push_attr("x1", &self.x1);
        buffer.push_attr("y1", &self.y1);
        buffer.push_attr("x2", &self.x2);
        buffer.push_attr("y2", &self.y2);
    }
}

impl Shape for Line {}

// Path =============================================

#[derive(BaseStyle)]
pub struct Path {
    pub path: Vec<Command>,
}

impl Path {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub(crate) fn push(&mut self, command: Command) {
        self.path.push(command);
    }

    pub fn append(&mut self, commands: &mut Vec<Command>) -> &mut Self {
        self.path.append(commands);
        self
    }

    pub fn path_only(&mut self) -> &mut Self {
        self
    }
}

impl Shape for Path {}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Visit for Path {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str("<path d=\"");
        for command in &self.path {
            command.visit(buffer);
        }
        buffer.push_str("\" ");
    }
}

// Rect ==============================================

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

pub enum Command {
    MoveTo(Coord),
    MoveToRelative(Coord),
    Line(Coord),
    LineRelative(Coord),
    VerticalLine(u16),
    VerticalLineRelative(u16),
    HorizontalLine(u16),
    HorizontalLineRelative(u16),
    Raw(String),
    ClosePath,
}

impl Visit for Command {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            Command::MoveTo(coord) => format!("M{},{} ", coord.0, coord.1),
            Command::MoveToRelative(coord) => format!("m{},{} ", coord.0, coord.1),
            Command::Line(coord) => format!("L{},{} ", coord.0, coord.1),
            Command::LineRelative(coord) => format!("l{},{} ", coord.0, coord.1),
            Command::VerticalLine(y) => format!("V{} ", y),
            Command::VerticalLineRelative(dy) => format!("v{} ", dy),
            Command::HorizontalLine(x) => format!("H{} ", x),
            Command::HorizontalLineRelative(dx) => format!("h{} ", dx),
            // TODO: check the end of s and add a space.
            Command::Raw(s) => s.to_string(),
            Command::ClosePath => "Z ".to_string(),
        };
        buffer.push_str(&str);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        element::ElementBuilder,
        style::LineCap,
        units::{Percent, Px},
    };

    use super::*;

    #[test]
    fn it_works() {
        let mut s = Svg::new()
            .size(100, 100)
            .push(
                Path::new()
                    .into_element()
                    .class("testing")
                    .class("testing22")
                    .transform(element::Transform::Scale(2, 2))
                    // .push(Command::MoveTo(Coord(10, 10)))
                    .move_to(10, 9)
                    .stroke(Color::Red)
                    .fill(Color::Black),
            )
            .push(
                ElementBuilder::line(Px(1) + Percent(2) + Px(3) + 4, Percent(5), 6, 7)
                    .id("myid")
                    .stroke_linecap(LineCap::Butt)
                    .fill(Color::Red),
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
        let a = Px(2);
        let b = Px(3);
        let c = a + b + Percent(4) + 3;
        let mut buf = Buffer::with_capacity(10);
        c.visit(&mut buf);
        eprintln!("expr:{}", buf.str());
        assert_eq!(1, 2);
    }
}
