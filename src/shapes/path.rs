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
        buffer.push_str("<path d=\"");
        for command in &self.path {
            command.visit(buffer);
        }
        buffer.push_str("\" ");
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
        let str = match self {
            Command::MoveTo(coord) => format!("M{},{} ", coord.0, coord.1),
            Command::MoveToRelative(coord) => format!("m{},{} ", coord.0, coord.1),
            Command::Line(coord) => format!("L{},{} ", coord.0, coord.1),
            Command::LineRelative(coord) => format!("l{},{} ", coord.0, coord.1),
            Command::VerticalLine(y) => format!("V{} ", y.0),
            Command::VerticalLineRelative(dy) => format!("v{} ", dy.0),
            Command::HorizontalLine(x) => format!("H{} ", x.0),
            Command::HorizontalLineRelative(dx) => format!("h{} ", dx.0),
            // TODO: check the end of s and add a space.
            Command::Raw(s) => s.clone(),
            Command::ClosePath => "Z ".to_string(),
            _ => todo!("not implemented yet"),
        };
        buffer.push_str(&str);
    }
}
