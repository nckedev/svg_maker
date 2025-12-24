use crate::{
    buffer::Buffer,
    element::Element,
    units::{Coord, XCoord, YCoord},
    visit::Visit,
};
use svg_maker_derive::*;

#[derive(BaseStyle, Shape)]
pub struct Path {
    pub path: Vec<Command>,
}

impl Path {
    pub fn new() -> Self {
        Self { path: vec![] }
    }
}

impl Element<Path> {
    pub fn path() -> Self {
        Element::from(Path::new())
    }

    pub fn push(mut self, command: Command) -> Self {
        self.path.push(command);
        self
    }

    pub fn push_commands(mut self, commands: &mut Vec<Command>) -> Self {
        self.path.append(commands);
        self
    }

    pub fn move_to<X, Y>(mut self, x: X, y: Y) -> Self
    where
        X: Into<XCoord>,
        Y: Into<YCoord>,
    {
        self.path.push(Command::MoveTo(Coord(x.into(), y.into())));
        self
    }

    pub fn move_to_relative<X, Y>(mut self, x: X, y: Y) -> Self
    where
        X: Into<XCoord>,
        Y: Into<YCoord>,
    {
        self.path
            .push(Command::MoveToRelative(Coord(x.into(), y.into())));
        self
    }

    pub fn line_path(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.path.push(Command::Line(Coord(x.into(), y.into())));
        self
    }

    pub fn line_relative(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.path
            .push(Command::LineRelative(Coord(x.into(), y.into())));
        self
    }

    pub fn vertical_line<Y>(mut self, y: Y) -> Self
    where
        Y: Into<YCoord>,
    {
        self.path.push(Command::VerticalLine(y.into()));
        self
    }

    pub fn vertical_line_relative<Y>(mut self, y: Y) -> Self
    where
        Y: Into<YCoord>,
    {
        self.path.push(Command::VerticalLineRelative(y.into()));
        self
    }

    pub fn horizontal_line<X>(mut self, x: X) -> Self
    where
        X: Into<XCoord>,
    {
        self.path.push(Command::HorizontalLine(x.into()));
        self
    }

    pub fn horizontal_line_relative<X>(mut self, x: X) -> Self
    where
        X: Into<XCoord>,
    {
        self.path.push(Command::HorizontalLineRelative(x.into()));
        self
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Visit for Path {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_tag("path");
        buffer.push_attr("d", &self.path);
        buffer.push_tag_self_close();
    }
}

pub enum Command {
    MoveTo(Coord),
    MoveToRelative(Coord),
    Line(Coord),
    LineRelative(Coord),
    VerticalLine(YCoord),
    VerticalLineRelative(YCoord),
    HorizontalLine(XCoord),
    HorizontalLineRelative(XCoord),
    //C x1 y1 x2 y2 x y
    //c dx1 dy1 dx2 dy2 dx dy
    CubicBezier,
    CubicBezierRelative,
    //S x y
    //s dx dy
    CubicBezierExtended,
    CubicBezierExtendedRelaitve,
    //Q x1 y1 x y
    //q dx1 dy1 dx dy
    QuadraticBezier,
    QuadraticBezierRelative,
    //T x y
    //t dx dy
    QuadraticBezierExtended,
    QuadraticBezierExtendedRelaitve,
    //  A rx ry x-axis-rotation large-arc-flag sweep-flag x y
    // a rx ry x-axis-rotation large-arc-flag sweep-flag dx dy
    Arc,
    ArcRelative,
    Raw(String),
    ClosePath,
}

impl Visit for Command {
    fn visit(&self, buffer: &mut Buffer) {
        match self {
            Command::MoveTo(coord) => coord.visit_prefix(buffer, "M"),
            Command::MoveToRelative(coord) => coord.visit_prefix(buffer, "m"),
            Command::Line(coord) => coord.visit_prefix(buffer, "L"),
            Command::LineRelative(coord) => coord.visit_prefix(buffer, "l"),
            Command::VerticalLine(y) => y.visit_prefix(buffer, "V"),
            Command::VerticalLineRelative(dy) => dy.visit_prefix(buffer, "v"),
            Command::HorizontalLine(x) => x.visit_prefix(buffer, "H"),
            Command::HorizontalLineRelative(dx) => dx.visit_prefix(buffer, "h"),
            // TODO: check the end of s and add a space.
            Command::Raw(s) => buffer.push_str(s),
            Command::ClosePath => buffer.push_str("Z"),
            _ => todo!("not implemented yet"),
        };
    }
}
