use std::fmt::Debug;

use svg_maker_derive::*;

use crate::{
    buffer::Buffer,
    units::{AlignAspectRatio, MeetOrSlice},
    visit::Visit,
};

pub use crate::marker_traits::Shape;

pub mod animations;
mod buffer;
pub mod color;
pub mod element;
mod marker_traits;
pub mod shapes;
pub mod style;
pub mod units;
pub mod visit;

/// first argument is the parent elementkind, rest of the arguments are the children,
/// ```impl_parent_of(<PARENT>, <CHILD>, <CHILD>, ...); ```
/// ```
/// impl_parent_of(Svg, Line, Rect);
/// ```
/// will result in
/// ```
/// impl Childof<Svg> for Element<Line> {}
/// impl Childof<Svg> for Element<Rect> {}
/// ```
#[macro_export]
macro_rules! impl_parent_of {
    ($parent:ty, $($child:ty),+ ) => {
        $(impl $crate::marker_traits::ChildOf<$parent> for $crate::element::Element<$child> {} )+
    };
}

/// first argument is the child elementkind, rest of the arguments are the parents,
/// ```impl_child_of(<CHILD>, <PARENT>, <PARENT>, ...); ```
/// ```
/// impl_child_of(Line, Svg, Group);
/// ```
/// will result in
/// ```
/// impl Childof<Svg> for Element<Line> {}
/// impl Childof<Group> for Element<Line> {}
/// ```
#[macro_export]
macro_rules! impl_child_of {
    ($child:ty, $($parent:ty),+ ) => {
        $(impl $crate::marker_traits::ChildOf<$parent> for $crate::element::Element<$child> {} )+
    };
}

#[derive(Clone, Copy, Debug, Default)]
struct Viewbox {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Visit for Viewbox {
    fn visit(&self, buffer: &mut Buffer) {
        let Viewbox { x, y, w, h } = self;
        buffer.push_str(&format!("{} {} {} {}", x, y, w, h));
    }
}

#[derive(Default, Debug)]
pub struct Options {
    pub invert_y: bool,
    pub optimizations: Optimizations,
}

#[derive(Debug)]
pub struct Optimizations {
    pub remove_unit_for_px: bool,
    pub remove_unit_for_deg: bool,
    pub convert_ms_to_s_if_shorter: bool,
    pub remove_newline: bool,
    pub remove_indent: bool,
}

impl Optimizations {
    pub fn all() -> Self {
        Self {
            remove_unit_for_px: true,
            remove_unit_for_deg: true,
            convert_ms_to_s_if_shorter: true,
            remove_newline: true,
            remove_indent: true,
        }
    }
}

impl Default for Optimizations {
    /// default configuration for optimizations
    /// remove_unit_for_px: true,
    /// remove_unit_for_deg: true,
    /// convert_ms_to_s_if_shorter: false,
    /// remove_newline: false,
    /// remove_indent: false,
    fn default() -> Self {
        Self {
            remove_unit_for_px: true,
            remove_unit_for_deg: true,
            convert_ms_to_s_if_shorter: false,
            remove_newline: false,
            remove_indent: false,
        }
    }
}

// Raw ======================================

#[derive(BaseStyle)]
pub struct Raw {
    inner: String,
}

impl Visit for Raw {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(&self.inner);
    }
}

struct PreserveAspectRatio {
    alignment: AlignAspectRatio,
    meet_or_slice: MeetOrSlice,
}

impl Visit for PreserveAspectRatio {
    fn visit(&self, buffer: &mut Buffer) {
        self.alignment.visit(buffer);
        self.meet_or_slice.visit(buffer);
    }
}

#[cfg(test)]
mod tests {
    use crate::{element::Element, shapes::path::Path};

    // use super::*;

    #[test]
    fn get_element_by_id() {
        let mut s = Element::svg().push(Element::path().id("test_id"));
        let path = s.get_element_by_id_mut::<Element<Path>>("test_id");
        assert!(path.is_some());
        assert_eq!(path.unwrap().id.as_ref().unwrap(), &"test_id".to_string());

        // let path = s.get_element_by_id_mut::<Path>("this_id_doesnt_exist");
        // assert!(path.is_none());
    }
}
