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
/// <textPath>, <text>, <tspan>
pub trait TextElement {}
/// <textPath>, <tspan>
pub trait TextElementChild {}

pub trait BaseElement: Visit + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_id(&self) -> Option<&str>;
}

pub trait Shape {
    fn into_element(self) -> Element<Self>
    where
        Self: Sized + Visit + ElementKind,
    {
        Element::new(self)
    }
}

pub trait Animate {}

pub trait RootElement {}

pub trait ElementKind {}

pub trait Hx {}

///<a>, <circle>, <ellipse>, <foreignObject>, <g>, <image>, <line>, <path>, <polygon>, <polyline>, <rect>, <svg>, <switch>, <symbol>, <text>, <textPath>, <tspan>, <use>
pub trait Renderable {}

///<a>, <defs>, <g>, <marker>, <mask>, <pattern>, <svg>, <switch>, <symbol>
pub trait ContainerElement {}
