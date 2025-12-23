use std::{
    collections::VecDeque,
    ops::{Add, Sub},
    rc::Rc,
};

use derive_more::Display;
use num_traits::Num;

use crate::{buffer::Buffer, visit::Visit};

pub struct Coord(pub XCoord, pub YCoord);

#[derive(Display, Debug, Default)]
pub struct XCoord(pub f64);

impl<T: Num + Into<f64>> From<T> for XCoord {
    fn from(value: T) -> Self {
        XCoord(value.into())
    }
}

impl Visit for XCoord {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.0.to_string());
    }
}

#[derive(Display, Debug, Default)]
pub struct YCoord(pub f64);

impl<T: Num + Into<f64>> From<T> for YCoord {
    fn from(value: T) -> Self {
        YCoord(value.into())
    }
}

impl Visit for YCoord {
    fn visit(&self, buffer: &mut Buffer) {
        if buffer.opts.invert_y {
            let v = buffer.opts.container_size - self.0;
            buffer.push_str(&v.to_string());
        } else {
            buffer.push_str(&self.0.to_string());
        }
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
            Length::Px(px) => Px(height as f64 - px.0).into(),
            // TODO: need to walk the expr tree to invert all individual Lenght's
            Length::Expr(_) => panic!("cannot invert an expression yet "),
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

#[derive(Display, Debug)]
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
