use std::ops::Deref;

use crate::shapes::text::Text;

struct Tspan(Text);

impl Deref for Tspan {
    type Target = Text;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
