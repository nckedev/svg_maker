use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Add, Sub},
    rc::Rc,
};

use derive_more::Display;
use num_traits::Num;

use crate::{buffer::Buffer, visit::Visit};

#[derive(Debug, PartialEq)]
pub struct Coord(pub XCoord, pub YCoord);

impl Visit for Coord {
    fn visit(&self, buffer: &mut Buffer) {
        self.0.visit(buffer);
        buffer.push_str(",");
        self.1.visit(buffer);
    }
}

impl<T, U> From<(T, U)> for Coord
where
    T: Num + Into<XCoord>,
    U: Num + Into<YCoord>,
{
    fn from(value: (T, U)) -> Self {
        Coord(value.0.into(), value.1.into())
    }
}

impl From<[f64; 2]> for Coord {
    fn from(value: [f64; 2]) -> Self {
        Coord::from((value[0], value[1]))
    }
}

#[derive(Display, Debug, Default, PartialEq)]
pub struct XCoord(pub f64);

impl<T: Num + Into<f64>> From<T> for XCoord {
    fn from(value: T) -> Self {
        XCoord(value.into())
    }
}

impl Visit for XCoord {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&format!("{}", Truncated(self.0)));
    }
}

#[derive(Display, Debug, Default, PartialEq)]
pub struct YCoord(pub f64);

impl<T: Num + Into<f64>> From<T> for YCoord {
    fn from(value: T) -> Self {
        YCoord(value.into())
    }
}

impl Visit for YCoord {
    fn visit(&self, buffer: &mut Buffer) {
        let value = if buffer.opts.invert_y {
            debug_assert!(
                buffer.viewbox.y != 0.,
                "viewbox must be set for invert_y to work"
            );
            buffer.viewbox.y - self.0
        } else {
            self.0
        };
        buffer.push_str(&format!("{}", Truncated(value)));
    }
}

#[derive(Debug, PartialEq)]
pub struct CubicArgs {
    pub p1: Coord,
    pub p2: Coord,
    pub end: Coord,
}

impl From<[f64; 6]> for CubicArgs {
    fn from(value: [f64; 6]) -> Self {
        Self {
            p1: Coord::from((value[0], value[1])),
            p2: Coord::from((value[2], value[3])),
            end: Coord::from((value[4], value[5])),
        }
    }
}

impl Visit for CubicArgs {
    fn visit(&self, buffer: &mut Buffer) {
        self.p1.visit(buffer);
        buffer.push_space();
        self.p2.visit(buffer);
        buffer.push_space();
        self.end.visit(buffer);
    }
}

// NOTE: the struct needs to have the same name as the enum varuant for the macro to work
#[derive(Debug, Clone)]
pub enum Length {
    Percent(Percent),
    Px(Px),
    Expr(Expr),
}

impl Length {
    pub fn invert(&self, height: u32) -> Length {
        match self {
            Length::Percent(percent) => Percent(100 - percent.0).into(),
            Length::Px(px) => Px(height as f64 - px.0).into(),
            // TODO: need to walk the expr tree to invert all individual Lenght's
            Length::Expr(_) => panic!("cannot invert an expression yet "),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Length::Percent(percent) => percent.0 == 0,
            Length::Px(px) => px.0 == 0.,
            Length::Expr(_) => false,
        }
    }

    pub fn is_greater_than_zero(&self) -> bool {
        match self {
            Length::Percent(percent) => percent.0 > 0,
            Length::Px(px) => px.0 > 0.,
            Length::Expr(_) => true,
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::Px(Px(0.))
    }
}

impl Visit for Length {
    fn visit(&self, buffer: &mut Buffer) {
        match self {
            Length::Percent(percent) => percent.visit(buffer),
            Length::Px(px) => px.visit(buffer),
            Length::Expr(expr) => {
                buffer.push_str("calc(");
                expr.visit(buffer);
                buffer.push(')');
            }
        }
    }
}

impl From<Percent> for Length {
    fn from(value: Percent) -> Self {
        Self::Percent(value)
    }
}

impl From<Px> for Length {
    fn from(value: Px) -> Self {
        Self::Px(value)
    }
}

impl From<Expr> for Length {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl From<u32> for Length {
    fn from(value: u32) -> Self {
        Self::Px(Px(value.into()))
    }
}

impl From<f64> for Length {
    fn from(value: f64) -> Self {
        Self::Px(Px(value))
    }
}

impl<T: Into<Length>> Sub<T> for Length {
    type Output = Length;

    fn sub(self, _rhs: T) -> Self::Output {
        // TODO: implement
        self
    }
}

impl<T: Into<Length>> Add<T> for Length {
    type Output = Length;

    fn add(self, rhs: T) -> Self::Output {
        match (self, rhs.into()) {
            (Length::Percent(lhs), Length::Percent(rhs)) => Length::Percent(Percent(lhs.0 + rhs.0)),
            (Length::Px(lhs), Length::Px(rhs)) => Length::Px(Px(lhs.0 + rhs.0)),
            (Length::Expr(_expr1), Length::Expr(_expr2)) => todo!("expr + expr"),
            (Length::Expr(mut lhs), rhs) => {
                // £TODO: Check if we can reduce the expression here, ie lhs.rhs has same type as
                // rhs then just add them
                lhs.push_rhs(
                    "+",
                    Expr {
                        lhs: Rc::new(rhs),
                        op: None,
                        rhs: None,
                    },
                );
                Length::Expr(lhs)
            }
            (lhs, rhs) => Length::Expr(Expr {
                lhs: Rc::new(lhs),
                op: Some("+".to_string()),
                rhs: Some(Box::new(Expr {
                    lhs: Rc::new(rhs),
                    op: None,
                    rhs: None,
                })),
            }),
        }
    }
}

#[derive(Display, Debug, Clone, Copy)]
pub struct Percent(pub u32);

impl Visit for Percent {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.0.to_string());
        buffer.push('%');
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Px(pub f64);

impl<T: Num + Into<f64>> From<T> for Px {
    fn from(value: T) -> Self {
        Px(value.into())
    }
}

impl Visit for Px {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.0.to_string());
        if !buffer.opts.optimizations.remove_unit_for_px {
            buffer.push_str("px");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    lhs: Rc<Length>,
    op: Option<String>,
    rhs: Option<Box<Expr>>,
}

impl Expr {
    fn push_rhs(&mut self, op: &str, e: Expr) {
        let mut que: VecDeque<&mut Box<Expr>> = VecDeque::new();
        if let Some(first) = &mut self.rhs {
            que.push_back(first);
        }

        while let Some(x) = que.pop_front() {
            match x.rhs {
                None => {
                    x.op = Some(op.to_string());
                    x.rhs = Some(Box::new(e));
                    return;
                }
                Some(ref mut v) => que.push_back(v),
            }
        }
    }
}

impl Visit for Expr {
    fn visit(&self, buffer: &mut Buffer) {
        self.lhs.visit(buffer);
        if let Some(op) = &self.op
            && let Some(rhs) = &self.rhs
        {
            buffer.push_str(op);
            rhs.visit(buffer);
        }
    }
}

macro_rules! impl_ops_for_lenght_units {
    ($($ident:ident),+) => {
        $(impl<T: Into<Length>> std::ops::Add<T> for $ident {
            type Output = Length;

            fn add(self, rhs: T) -> Self::Output {
                Length::from(self) + rhs
            }
        })+

        $(impl<T: Into<Length>> std::ops::Sub<T> for $ident {
            type Output = Length;

            fn sub(self, rhs: T) -> Self::Output {
                Length::from(self) - rhs
            }
        })+
    };
}

impl_ops_for_lenght_units!(Percent, Px);

// impl<T: Into<Length>> Add<T> for Px {
//     type Output = Length;
//
//     fn add(self, rhs: T) -> Self::Output {
//         Length::Px(self) + rhs
//     }
//     //self = px
//     //rhs = into Length
// }

pub enum Angle {
    Deg(f64),
    Grad(f64),
    Rad(f64),
}

impl<T: Num + Into<f64>> From<T> for Angle {
    fn from(value: T) -> Self {
        Angle::Deg(value.into())
    }
}

impl Visit for Angle {
    fn visit(&self, buffer: &mut Buffer) {
        match self {
            Angle::Deg(v) => {
                if buffer.opts.optimizations.remove_unit_for_deg {
                    buffer.push_str(&format!("{}", v))
                } else {
                    buffer.push_str(&format!("{}deg", v))
                }
            }
            Angle::Grad(v) => buffer.push_str(&format!("{}grad", v)),
            Angle::Rad(v) => buffer.push_str(&format!("{}rad", v)),
        };
    }
}

// ==== TIME =======================================================================
#[derive(Clone, Copy, Debug)]
pub enum Time {
    MilliSeconds(f64),
    Seconds(f64),
}

impl Time {
    pub(crate) fn to_seconds_if_shorter(self) -> Self {
        if let Time::MilliSeconds(ms) = self
            && ms % 1000. == 0.
        {
            Self::Seconds(ms / 1000.)
        } else {
            self
        }
    }
}

impl Visit for Time {
    fn visit(&self, buffer: &mut Buffer) {
        let mut t = *self;
        if buffer.opts.optimizations.convert_ms_to_s_if_shorter {
            t = self.to_seconds_if_shorter();
        };

        match t {
            Time::MilliSeconds(ms) => buffer.push_str(&format!("{}ms", ms)),
            Time::Seconds(s) => buffer.push_str(&format!("{}s", s)),
        }
    }
}

// ====== ALignAspectRatio ====================================================

#[derive(Debug, PartialEq, Eq)]
pub enum AlignAspectRatio {
    None,
    XMinYMin,
    XMinYMid,
    XMinYMax,
    XMidYMin,
    XMidYMid,
    XMidYMax,
    XMaxYMin,
    XMaxYMid,
    XMaxYMax,
}

impl Visit for AlignAspectRatio {
    fn visit(&self, buffer: &mut Buffer) {
        let v = match self {
            AlignAspectRatio::None => "",
            AlignAspectRatio::XMinYMin => "xMinYMin",
            AlignAspectRatio::XMinYMid => "xMinYMid",
            AlignAspectRatio::XMinYMax => "xMinYMax",
            AlignAspectRatio::XMidYMin => "xMidYMin",
            AlignAspectRatio::XMidYMid => "xMidYMid",
            AlignAspectRatio::XMidYMax => "xMidYMax",
            AlignAspectRatio::XMaxYMin => "xMaxYMin",
            AlignAspectRatio::XMaxYMid => "xMaxYMid",
            AlignAspectRatio::XMaxYMax => "xMaxYMax",
        };
        buffer.push_str(v);
    }
}

// ====== MeetOrSlice =========================================================

#[derive(Debug)]
pub enum MeetOrSlice {
    Meet,
    Slice,
}

impl Visit for MeetOrSlice {
    fn visit(&self, buffer: &mut Buffer) {
        let v = match self {
            MeetOrSlice::Meet => " meet",
            MeetOrSlice::Slice => " slice",
        };
        buffer.push_str(v);
    }
}

#[derive(Debug)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

impl Visit for TextAnchor {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            TextAnchor::Start => "start",
            TextAnchor::Middle => "middle",
            TextAnchor::End => "end",
        };
        buffer.push_str(str);
    }
}

// ===== Truncated ============================================================

struct Truncated(f64);

impl Display for Truncated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PRECISION: f64 = 1e3;
        let rounded = (self.0 * PRECISION).round() / PRECISION;
        write!(f, "{}", rounded)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    mod time {
        use rstest::rstest;

        use crate::{buffer::Buffer, units::Time, visit::Visit};

        #[rstest]
        #[case(Time::MilliSeconds(2000_f64), "2s")]
        #[case(Time::MilliSeconds(200_f64), "200ms")]
        #[case(Time::MilliSeconds(2550_f64), "2550ms")]
        #[case(Time::Seconds(20_f64), "20s")]
        fn time_shortening_optmization(#[case] before: Time, #[case] after: &str) {
            let mut buffer = Buffer::with_capacity(100);
            buffer.opts.optimizations.convert_ms_to_s_if_shorter = true;
            before.visit(&mut buffer);
            assert_eq!(buffer.str(), after);
        }
    }
}
