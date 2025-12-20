use derive_more::Display;
use std::{
    collections::VecDeque,
    fmt::Debug,
    mem::discriminant,
    ops::{Add, Sub},
    rc::Rc,
};

use crate::{Visit, buffer::Buffer, color::Color};

#[derive(Default)]
pub struct Style {
    pub fill: Option<Color>,
    pub fill_opacity: Option<f32>,
    pub fill_rule: Option<FillRule>,
    pub stroke: Option<Color>,

    pub stroke_dasharray: Option<Vec<Length>>,
    pub stroke_dashoffset: Option<Length>,

    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub stroke_miterlimit: Option<u32>,
    pub stroke_opacity: Option<f32>,

    pub stroke_width: Option<Length>,
}

impl Visit for Style {
    fn visit(&self, buffer: &mut Buffer) {
        macro_rules! visit_if_not_none {
            ($ident:ident, $str:literal) => {
                if let Some($ident) = &self.$ident {
                    let str = format!(" {}: ", $str);
                    buffer.push_str(&str);
                    $ident.visit(buffer);
                    buffer.push(';');
                }
            };
        }

        // do nothing if there is no styling option set.
        if let Style {
            fill: None,
            stroke: None,
            fill_opacity: None,
            fill_rule: None,
            stroke_dasharray: None,
            stroke_dashoffset: None,
            stroke_linecap: None,
            stroke_linejoin: None,
            stroke_miterlimit: None,
            stroke_opacity: None,
            stroke_width: None,
        } = &self
        {
            return;
        }

        buffer.push_str(r##" style=""##);

        visit_if_not_none!(fill, "fill");
        visit_if_not_none!(stroke, "stroke");
        visit_if_not_none!(fill_opacity, "fill-opacity");
        visit_if_not_none!(fill_rule, "fill-rule");

        buffer.push_str(r##"" "##);
    }
}

pub enum FillRule {
    NonZero,
    EvenOdd,
}

impl Visit for FillRule {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            FillRule::NonZero => "nonzero",
            FillRule::EvenOdd => "evenodd",
        };
        buffer.push_str(str);
    }
}

pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Visit for LineCap {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        };
        buffer.push_str(str);
    }
}

pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl Visit for LineJoin {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            LineJoin::Miter => "miter",
            LineJoin::Round => "round",
            LineJoin::Bevel => "bevel",
        };
        buffer.push_str(str);
    }
}

// NOTE: the struct needs to have the same name as the enum varuant for the macro to work
#[derive(Debug)]
pub enum Length {
    Percent(Percent),
    Px(Px),
    Expr(Expr),
}

impl Length {
    pub fn invert(&self, height: u32) -> Length {
        match self {
            Length::Percent(percent) => Percent(100 - percent.0).into(),
            Length::Px(px) => Px(height - px.0).into(),
            // TODO: need to walk the expr tree to invert all individual Lenght's
            Length::Expr(_) => panic!("cannot invert an expression yet "),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::Px(Px(0))
    }
}

impl Visit for Length {
    fn visit(&self, buffer: &mut Buffer) {
        match self {
            Length::Percent(percent) => {
                percent.visit(buffer);
                buffer.push('%');
            }
            Length::Px(px) => {
                px.visit(buffer);
                buffer.push_str("px");
            }
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
        Self::Px(Px(value))
    }
}

#[derive(Display, Debug)]
pub struct Percent(pub u32);

impl Visit for Percent {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.0.to_string());
    }
}

// impl<T: LengthUnit + Display> Visit for T {
//     fn visit(&self, buffer: &mut Buffer) {
//         buffer.push_str(&format!("{}", self));
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct Px(pub u32);

impl Visit for Px {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.0.to_string());
    }
}

// trait LengthUnit {}

#[derive(Debug)]
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
//

impl<T: Into<Length>> Sub<T> for Length {
    type Output = Length;

    fn sub(self, rhs: T) -> Self::Output {
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
