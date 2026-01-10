use crate::{buffer::Buffer, visit::Visit};

#[allow(clippy::enum_variant_names)] // warns on currentcolor otherwise
#[derive(Debug)]
pub enum Color {
    Red,
    Black,
    White,
    CssName(String),
    Transparent,
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hex(String),
    //lightness, chroma, hue / alpha
    Oklch(Oklch),
    CssVar(String),
    CurrentColor,
    Url(String),
    // Test(Oklch<f32, u32>),
}

impl Visit for Color {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            Color::Red => "red",
            Color::Black => "black",
            Color::White => "white",
            Color::CssName(name) => name,
            Color::Transparent => "transparent",
            Color::Rgb(r, g, b) => &format!("rgb({} {} {})", r, g, b),
            Color::Rgba(_, _, _, _) => todo!(),
            Color::Hex(s) => s,
            Color::Oklch(color) => &color.visit_return(),
            Color::CssVar(var) => {
                if var.starts_with("--") {
                    &format!("var({})", var)
                } else {
                    &format!("var(--{})", var)
                }
            }
            Color::CurrentColor => "currentColor",
            Color::Url(s) => {
                if s.starts_with("#") {
                    &format!("url({s})")
                } else {
                    &format!("url(#{s})")
                }
            }
        };
        buffer.push_str(str);
    }
}

impl From<Oklch> for Color {
    fn from(value: Oklch) -> Self {
        Color::Oklch(value)
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            x if x.starts_with("#") => Color::Hex(x.to_string()),
            x if x.starts_with("--") => Color::CssVar(x.to_string()),
            x => Color::CssName(x.to_string()),
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Oklch {
    lightness: f64,
    chroma: f64,
    hue: u16,
    alpha: Option<f64>,
}

impl Oklch {
    pub fn new(lum: f64, chroma: f64, hue: u16) -> Self {
        Self {
            lightness: lum,
            chroma,
            hue,
            alpha: None,
        }
    }
    pub fn with_aplha(mut self, alpha: f64) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn clone_with_hue(&self, hue: u16) -> Self {
        let mut color = *self;
        color.hue = hue;
        color
    }

    pub fn generate_from_with_lightness<const N: usize>(
        from: Oklch,
        lightness_variance: f64,
    ) -> [Oklch; N] {
        debug_assert!(lightness_variance <= 1., "lightness variance should be < 1");
        let mut list = [Oklch::default(); N];
        (0..N).for_each(|x| {
            list[x] = Oklch {
                lightness: from.lightness + (x as f64 * lightness_variance),
                chroma: from.chroma,
                hue: from.hue,
                alpha: from.alpha,
            };
        });
        list
    }

    pub fn generate_from_with_hue<const N: usize>(from: Oklch, hue_variance: u16) -> [Oklch; N] {
        let mut list = [from; N];
        (0..N).for_each(|x| {
            list[x] = Oklch {
                lightness: from.lightness,
                chroma: from.chroma,
                hue: from.hue + (x as u16 * (hue_variance % 360)),
                alpha: from.alpha,
            }
        });
        list
    }
}

impl Visit for Oklch {
    fn visit(&self, buffer: &mut Buffer) {
        match self {
            Oklch {
                lightness: lum,
                chroma,
                hue,
                alpha: Some(a),
            } => buffer.push_str(&format!("oklch({} {} {} / {})", lum, chroma, hue, a)),
            Oklch {
                lightness: lum,
                chroma,
                hue,
                alpha: None,
            } => buffer.push_str(&format!("oklch({} {} {})", lum, chroma, hue)),
        }
    }
}
