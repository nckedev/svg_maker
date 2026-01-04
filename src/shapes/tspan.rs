use std::ops::Deref;

use crate::shapes::text::Text;

pub struct Tspan(Text);

impl Deref for Tspan {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    // use super::*;

    #[test]
    fn tspan() {}
}
