use crate::{
    buffer::Buffer,
    element::Element,
    marker_traits::ElementKind,
    measure::{self, Measurement},
    path_parser,
    units::{Coord, CubicArgs, XCoord, YCoord},
    visit::Visit,
};
use svg_maker_derive::*;

#[derive(Debug, BaseStyle, Shape)]
pub struct Path {
    pub path: Vec<Command>,
}

impl Path {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    /// EXPERIMETNAL
    pub fn measure(&self) -> Measurement {
        // TODO: only works on relative paths

        let mut max = Measurement::default();
        let mut current = Measurement::default();
        // Skip the first element if it is a moveto or a movetoreelative, since we just
        // want ot measure the actual width
        let skip = if let Some(&Command::MoveTo(_)) | Some(&Command::MoveToRelative(_)) =
            self.path.first()
        {
            1
        } else {
            0
        };

        let _iter = self
            .path
            .iter()
            .skip_while(|p| matches!(p, &&Command::MoveTo(_) | &&Command::MoveToRelative(_)));

        // TODO: should also skip move to it is the last command, since no drawing occures after
        // that the new location should not be counted to the total width
        for p in self.path.iter().skip(skip) {
            let v = match p {
                Command::MoveTo(coord) => (coord.0.0, coord.1.0, false),
                Command::MoveToRelative(coord) => (coord.0.0, coord.1.0, true),
                Command::Line(coord) => (coord.0.0, coord.1.0, false),
                Command::LineRelative(coord) => (coord.0.0, coord.1.0, true),
                Command::VerticalLine(ycoord) => (0., ycoord.0, false),
                Command::VerticalLineRelative(ycoord) => (0., ycoord.0, true),
                Command::HorizontalLine(xcoord) => (xcoord.0, 0., false),
                Command::HorizontalLineRelative(xcoord) => (xcoord.0, 0., true),
                Command::CubicBezier(args) => (args.end.0.0, args.end.1.0, false),
                Command::CubicBezierRelative(args) => (args.end.0.0, args.end.1.0, true),
                Command::CubicBezierExtended => todo!(),
                Command::CubicBezierExtendedRelaitve => todo!(),
                Command::QuadraticBezier => todo!(),
                Command::QuadraticBezierRelative => todo!(),
                Command::QuadraticBezierExtended => todo!(),
                Command::QuadraticBezierExtendedRelaitve => todo!(),
                Command::Arc => todo!(),
                Command::ArcRelative => todo!(),
                Command::Raw(_) => todo!(),
                Command::ClosePath => todo!(),
                Command::Invalid => todo!(),
            };
            match v {
                (x, y, true) => measure::check_relative_path(&mut max, &mut current, (x, y)),
                (x, y, false) => measure::check_absolute_path(&mut max, &mut current, (x, y)),
            }
            // check(&mut max_x, &mut current_x, v);
        }

        max
    }
}

impl Element<Path> {
    pub fn path() -> Self {
        Element::from(Path::new())
    }

    pub fn push_path(mut self, command: Command) -> Self {
        self.path.push(command);
        self
    }

    /// appends a path or sub path to the path
    pub fn path_from_str(mut self, path: &str, scale: f64) -> Self {
        // TODO:
        // 1. parse to commands
        // 2. scale to appropriate scale
        // 3. append to path
        let _paths = path_parser::parse(path);
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

    /// Creates a cubic bezier curve.
    /// `(f64, f64)` (x, y) implements `Into<Coord>`
    pub fn cubic_bezier<P>(mut self, p1: P, p2: P, end: P) -> Self
    where
        P: Into<Coord>,
    {
        self.path.push(Command::CubicBezier(CubicArgs {
            p1: p1.into(),
            p2: p2.into(),
            end: end.into(),
        }));
        self
    }

    /// Creates a cubic bezier curve where all the values are relative to current pos.
    /// `(f64, f64)` (x, y) implements `Into<Coord>`
    pub fn cubic_bezier_relative<P>(mut self, p1: P, p2: P, end: P) -> Self
    where
        P: Into<Coord>,
    {
        self.path.push(Command::CubicBezierRelative(CubicArgs {
            p1: p1.into(),
            p2: p2.into(),
            end: end.into(),
        }));
        self
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl ElementKind for Path {
    const TAG: &'static str = "path";
}

impl Visit for Path {
    fn visit(&self, buffer: &mut Buffer) {
        if let Some(&Command::MoveTo(_)) = self.path.first() {
            buffer
                .warnings
                .push("Path must start with a MoveTo command".to_string());
        }
        buffer.push_attr("d", &self.path);
    }
}

#[derive(Debug, PartialEq)]
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
    CubicBezier(CubicArgs),
    CubicBezierRelative(CubicArgs),
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
    Invalid,
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
            Command::CubicBezier(args) => args.visit_prefix(buffer, "C"),
            Command::CubicBezierRelative(args) => args.visit_prefix(buffer, "c"),
            // TODO: check the end of s and add a space.
            Command::Raw(s) => buffer.push_str(s),
            Command::ClosePath => buffer.push_str("Z"),
            _ => todo!("not implemented yet"),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::Options;

    use super::*;

    #[test]
    fn test_name() {
        let e = Element::path()
            .move_to(10, 10)
            .horizontal_line_relative(10)
            .line_relative(20, 90)
            .horizontal_line_relative(-10)
            .cubic_bezier((9, 9), (1, 1), (2, 3))
            .horizontal_line_relative(100);

        assert_eq!(e.render(Some(Options::default())), "2");
    }
}
