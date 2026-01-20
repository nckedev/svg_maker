use std::fmt::Debug;

use svg_maker_derive::*;

use crate::{
    buffer::Buffer,
    shapes::{
        circle::Circle, foreign_object::ForeignObject, group::Group, line::Line, path::Path,
        polygon::Polygon, rect::Rect, svg::Svg, text::Text, tspan::Tspan, use_href::Use,
    },
    units::{AlignAspectRatio, MeetOrSlice},
    visit::Visit,
};

pub use crate::marker_traits::*;

pub mod animations;
mod buffer;
pub mod color;
pub mod element;
pub mod marker_traits;
mod measure;
mod path_parser;
pub mod shapes;
pub mod style;
pub mod units;
pub mod visit;

/// first argument is the parent elementkind, rest of the arguments are the children,
/// ```impl_parent_of(<PARENT>, <CHILD>, <CHILD>, ...); ```
/// ```rust.ignore
/// impl_parent_of(Svg, Line, Rect);
/// ```
/// will result in
/// ```rust.ignore
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
/// ```rust.ignore
/// impl_child_of(Line, Svg, Group);
/// ```
/// will result in
/// ```rust.ignore
/// impl Childof<Svg> for Element<Line> {}
/// impl Childof<Group> for Element<Line> {}
/// ```
#[macro_export]
macro_rules! impl_child_of {
    ($child:ty, $($parent:ty),+ ) => {
        $(impl $crate::marker_traits::ChildOf<$parent> for $crate::element::Element<$child> {} )+
    };
}

#[macro_export]
macro_rules! impl_parent_child {
    ($parent:ty, $($child:ty),+ ) => {
        $(
        impl $crate::marker_traits::ChildOf<$parent> for $crate::element::Element<$child> {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
            fn get_z_index(&self) -> Option<i32> {
                self.z_index
            }
            fn get_id(&self) -> Option<&str> {
                self.id.as_ref().map(|s| s.as_str())
            }
        }
        // impl $crate::marker_traits::ParentOf<$crate::element::Element<$child>> for $parent { }
        )+

    };
}

impl_parent_child!(
    Svg,
    Use,
    Line,
    Rect,
    Circle,
    Text,
    Polygon,
    Path,
    Group,
    ForeignObject
);
impl_parent_child!(
    Group,
    Group,
    Use,
    Line,
    Rect,
    Circle,
    Text,
    Polygon,
    Path,
    ForeignObject
);
impl_parent_child!(Text, Tspan, String);

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
    /// collapses path commands of the same kind into one
    /// # Example
    /// d="L10,20 L30,40" => d="L10,20,30,40"
    pub collapse_same_path_command: bool,
}

impl Optimizations {
    pub fn all() -> Self {
        Self {
            remove_unit_for_px: true,
            remove_unit_for_deg: true,
            convert_ms_to_s_if_shorter: true,
            remove_newline: true,
            remove_indent: true,
            collapse_same_path_command: true,
        }
    }
}

impl Default for Optimizations {
    /// Default configuration for optimizations
    /// ```rust.ignore
    /// remove_unit_for_px: true,
    /// remove_unit_for_deg: true,
    /// convert_ms_to_s_if_shorter: false,
    /// remove_newline: false,
    /// remove_indent: false,
    /// collapse_same_path_command: false,
    /// ```
    fn default() -> Self {
        Self {
            remove_unit_for_px: true,
            remove_unit_for_deg: true,
            convert_ms_to_s_if_shorter: false,
            remove_newline: false,
            remove_indent: false,
            collapse_same_path_command: false,
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

#[derive(Debug)]
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

mod tester {
    use crate::shapes::svg::Svg;

    /// renders the svgs to a html page
    fn run(svgs: &[Svg]) {
        // TODO: render the svgs in a grid
    }
}

#[cfg(test)]
mod tests {
    use crate::{element::Element, shapes::path::Path};

    use super::*;

    #[test]
    fn get_element_by_id() {
        let mut s = Element::svg().push(Element::path().id("test_id"));
        let path = s.get_element_by_id_mut::<Element<Path>>("test_id");
        assert!(path.is_some());
        assert_eq!(path.unwrap().id.as_ref().unwrap(), &"test_id".to_string());

        let path = s.get_element_by_id_mut::<Element<Path>>("this_id_doesnt_exist");
        assert!(path.is_none());
    }
}
