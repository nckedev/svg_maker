use std::rc::Rc;

use crate::{buffer::Buffer, visit::Visit};

struct InternalColor {
    lum: u8,
    chroma: u8,
    hue: u16,
    alpha: Option<u8>,
}

impl InternalColor {
    fn new(lum: u8, chroma: u8, hue: u16, alpha: Option<u8>) -> Self {
        Self {
            lum,
            chroma,
            hue,
            alpha,
        }
    }
}

impl From<Color> for InternalColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Red => InternalColor::new(50, 100, 50, None),
            Color::Black => todo!(),
            Color::White => todo!(),
            Color::Transparent => todo!(),
            Color::Rgb(_r, _g, _b) => todo!(),
            Color::Rgba(_r, _g, _b, _a) => todo!(),
            Color::Hex(_) => todo!(),
            Color::Oklch(l, c, h) => InternalColor::new(l, c, h, None),
            Color::OklchAlpha(l, c, h, a) => InternalColor::new(l, c, h, Some(a)),
            _ => todo!("catch all fallback"),
        }
    }
}

impl From<&Color> for InternalColor {
    fn from(value: &Color) -> Self {
        match value {
            Color::Black => todo!(),
            Color::White => todo!(),
            Color::Transparent => todo!(),
            Color::Rgb(_, _, _) => todo!(),
            Color::Rgba(_, _, _, _) => todo!(),
            Color::Hex(_) => todo!(),
            Color::Oklch(_, _, _) => todo!(),
            Color::OklchAlpha(_, _, _, _) => todo!(),
            // NOTE: this does not work on all browsers for tvs.
            Color::OklchFrom(_color, _, _, _, _) => todo!(),
            Color::CssVar(_) => todo!(),
            _ => todo!("fallback"),
        }
    }
}

impl From<InternalColor> for String {
    fn from(value: InternalColor) -> Self {
        match value {
            InternalColor {
                lum,
                chroma,
                hue,
                alpha: Some(a),
            } => format!("oklch({} {} {} / {})", lum, chroma, hue, a),
            InternalColor {
                lum,
                chroma,
                hue,
                alpha: None,
            } => format!("oklch({} {} {})", lum, chroma, hue),
        }
    }
}

#[allow(clippy::enum_variant_names)] // warns on currentcolor otherwise
pub enum Color {
    Red,
    Black,
    White,
    Transparent,
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hex(String),
    //lightness, chroma, hue / alpha
    Oklch(u8, u8, u16),
    OklchAlpha(u8, u8, u16, u8),
    OklchFrom(Rc<Color>, u8, u8, u16, u8),
    CssVar(String),
    CurrentColor,
    // Test(Oklch<f32, u32>),
}

impl Visit for Color {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            Color::Red => "red",
            Color::Black => "black",
            Color::White => "white",
            Color::Transparent => todo!(),
            Color::Rgb(r, g, b) => &format!("rgb({} {} {})", r, g, b),
            Color::Rgba(_, _, _, _) => todo!(),
            Color::Hex(_) => todo!(),
            Color::Oklch(l, c, h) => &format!("oklch({} {} {})", l, c, h),
            Color::OklchAlpha(l, c, h, a) => &format!("oklch({} {} {} {})", l, c, h, a),
            Color::OklchFrom(_color, _, _, _, _) => todo!(),
            Color::CssVar(_) => todo!(),
            Color::CurrentColor => "currentColor",
        };
        buffer.push_str(str);
    }
}

struct Oklch<L, C> {
    lum: L,
    chroma: C,
    hue: u16,
}
