#![allow(dead_code)]
use std::{any::Any, fmt::Debug};

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
    // fn get_id(&self) -> Option<&str>;
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
pub trait Descriptive {}

pub trait ChildOf<T>: Any
where
    Self: Visit + Debug,
{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_z_index(&self) -> Option<i32>;
    fn get_id(&self) -> Option<&str>;
}

pub trait Parent<P>
where
    Self: Visit + Debug,
{
    fn push<C: ChildOf<P>>(self, value: C) -> Self;
    fn push_if<C>(self, pred: bool, value: C) -> Self
    where
        C: ChildOf<P>,
        Self: Sized,
    {
        if pred { self.push(value) } else { self }
    }
    fn push_iter<C: ChildOf<P>>(self, values: impl IntoIterator<Item = C>) -> Self;
    fn push_vec<C: ChildOf<P>>(self, values: Vec<C>) -> Self;
}

pub trait RootElement {}

pub trait ElementKind: Visit {
    const TAG: &'static str;
}

impl ElementKind for String {
    const TAG: &'static str = "STRING";
}

pub trait Hx {}

///<a>, <circle>, <ellipse>, <foreignObject>, <g>, <image>, <line>, <path>, <polygon>, <polyline>, <rect>, <svg>, <switch>, <symbol>, <text>, <textPath>, <tspan>, <use>
pub trait Renderable {}

///<a>, <defs>, <g>, <marker>, <mask>, <pattern>, <svg>, <switch>, <symbol>
pub trait ContainerElement: Visit + ElementKind {}
