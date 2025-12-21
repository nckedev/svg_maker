#![allow(dead_code)]
use std::any::Any;

use crate::{Visit, element::Element};

/// maker trait fro elements that can hava a style attribute.
/// implementors of this trait will get access to the base style attributes.
pub trait BaseStyle {}

/// maker trait for shapes that have an line ending
/// implementors of this trait gets access to "stroke-linecap" etc..
/// should be implemented for
/// shape, polyline, line, text, textpath, tspan
pub trait OpenEndedShape {}

pub trait ClosedShape {}

/// marker trait for elements that contains text
/// text, textpath, tspan
pub trait TextElement {}

pub trait BaseElement: Visit + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_id(&self) -> Option<&str>;
    fn from_shape<S>(shape: S) -> Element<S>
    where
        S: Sized + Visit + Shape,
        Self: Sized;
}

pub trait Shape {
    fn into_element(self) -> Element<Self>
    where
        Self: Sized + Visit,
    {
        Element::new(self)
    }
}

pub trait Animate {}

pub trait RootElement {}

pub trait Hx {}
