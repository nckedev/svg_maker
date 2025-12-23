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

    pub fn move_to_relative(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
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
            Command::Raw(s) => s.clone(),
            Command::ClosePath => "Z ".to_string(),
        };
        buffer.push_str(&str);
    }
}
