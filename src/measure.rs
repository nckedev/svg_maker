// ===== Measurment ============================================================

use std::ops::{Add, Deref};

#[derive(Debug, Default, Clone, Copy)]
pub struct Measurement {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl Measurement {
    pub fn approx_eq(a: Self, b: Self) -> bool {
        const EPS: f64 = 1e-12;
        (a.width - b.width).abs() < EPS && (a.height - b.height).abs() < EPS
    }
}

impl From<(f64, f64)> for Measurement {
    fn from(value: (f64, f64)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

impl Add for Measurement {
    type Output = Measurement;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Deref for Measurement {
    type Target = Measurement;

    fn deref(&self) -> &Self::Target {
        self
    }
}

pub fn check_relative_path(
    max: &mut Measurement,
    current: &mut Measurement,
    value: impl Into<Measurement>,
) {
    let t = *current + *value.into();
    *current = t;
    if current.width.abs() >= max.width {
        max.width = current.width
    }

    if current.height.abs() >= max.height {
        max.height = current.height;
    }
}

pub fn check_absolute_path(
    max: &mut Measurement,
    current: &mut Measurement,
    value: impl Into<Measurement>,
) {
    *current = value.into();
    if current.width.abs() >= max.width {
        max.width = current.width;
    }
}
