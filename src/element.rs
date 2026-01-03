use std::{
    any::Any,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    Options,
    buffer::Buffer,
    color::Color,
    marker_traits::*,
    style::{FillRule, LineCap, LineJoin, Style},
    units::Length,
    visit::Visit,
};

#[derive(Debug)]
pub struct Element<T> {
    pub id: Option<String>,
    class: Option<String>,
    /// NOTE: this style object contains all possible styles, and some might not be applicable to
    /// the current element,
    pub(crate) style: Style,
    transforms: Option<Vec<Transform>>,
    hx: Option<HxData>,
    pub(crate) kind: T,
    children: Vec<Box<dyn ChildOf<T>>>,
}

impl<T: ElementKind + Visit> From<T> for Element<T> {
    fn from(value: T) -> Self {
        Element::new(value)
    }
}

impl<T: Visit + 'static + ElementKind> BaseElement for Element<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

impl<T> Deref for Element<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<T> DerefMut for Element<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl<T: Visit + ElementKind> Visit for Element<T> {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_tag(T::TAG);
        self.kind.visit(buffer);
        buffer.push_attr_opt("id", &self.id);
        buffer.push_attr_opt("class", &self.class);
        self.hx.visit(buffer);
        buffer.push_attr_opt("transform", &self.transforms);
        self.style.visit(buffer);
        //TODO: if the element has child elementes like animations, include them before closing, if not do a
        //selfclose tag
        if self.children.is_empty() {
            buffer.push_tag_self_close();
        } else {
            buffer.push_tag_end();
            for child in &self.children {
                child.visit(buffer);
            }
            buffer.push_tag_close(T::TAG);
        }
    }
}

impl<T> Element<T>
where
    Self: Visit,
    T: Visit + ElementKind + Any + 'static,
{
    pub fn get_element_by_id_mut<U: ChildOf<T>>(&mut self, _id: &str) -> Option<&mut U> {
        eprintln!("{:#?}", self.children);
        for child in &mut self.children {
            let c = child as &mut dyn Any;
            let ch = c.downcast_mut::<U>();
            // dbg!(&ch);
            if let Some(b) = ch
            // && b.id == Some(id.to_string())
            {
                return Some(b);
            }
        }
        None
    }
}

impl<T: ElementKind + Visit> Element<T> {
    pub fn new(kind: T) -> Self {
        Self {
            id: None,
            class: None,
            style: Style::default(),
            transforms: None,
            hx: None,
            kind,
            children: vec![],
        }
    }

    pub fn push<U>(mut self, value: U) -> Self
    where
        U: ChildOf<T> + Visit + 'static,
    {
        self.children.push(Box::new(value));
        self
    }

    pub fn push_iter<U>(mut self, value: impl IntoIterator<Item = U>) -> Self
    where
        U: ChildOf<T> + Visit + 'static,
    {
        for v in value {
            self.children.push(Box::new(v) as Box<dyn ChildOf<T>>);
        }
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        if let Some(ref mut c) = self.class {
            c.push(' ');
            c.push_str(class);
        } else {
            self.class = Some(class.to_string());
        }
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn render(&self, opts: Option<Options>) -> String {
        let mut buffer = Buffer::with_capacity(100);
        if let Some(opt) = opts {
            buffer.opts = opt;
        }
        self.visit(&mut buffer);
        buffer.str().to_string()
    }

    /// Returns the number of children the element have
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.children.len()
    }
}

impl<T: BaseStyle> Element<T> {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn transform(mut self, transform: Transform) -> Self {
        if let Some(ref mut transforms) = self.transforms {
            transforms.push(transform);
        } else {
            self.transforms = Some(vec![transform]);
        }
        self
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.style.fill = Some(color);
        self
    }

    pub fn fill_opacity(mut self, opacity: f32) -> Self {
        self.style.fill_opacity = Some(opacity);
        self
    }

    pub fn fill_rule(mut self, rule: FillRule) -> Self {
        self.style.fill_rule = Some(rule);
        self
    }

    pub fn stroke(mut self, color: Color) -> Self {
        self.style.stroke = Some(color);
        self
    }

    pub fn stroke_linejoin(mut self, join: LineJoin) -> Self {
        self.style.stroke_linejoin = Some(join);
        self
    }

    pub fn stroke_width(mut self, width: impl Into<Length>) -> Self {
        self.style.stroke_width = Some(width.into());
        self
    }

    pub fn stroke_dasharray<C, L>(mut self, array: C) -> Self
    where
        C: IntoIterator<Item = L>,
        L: Into<Length>,
    {
        let arr = array.into_iter().map(|e| e.into()).collect::<Vec<Length>>();
        self.style.stroke_dasharray = Some(arr);
        self
    }

    pub fn stroke_dashoffset(mut self, length: impl Into<Length>) -> Self {
        self.style.stroke_dashoffset = Some(length.into());
        self
    }

    pub fn stroke_opacity(mut self, opacity: f32) -> Self {
        self.style.stroke_opacity = Some(opacity);
        self
    }

    pub fn stroke_miterlimit(mut self, limit: f32) -> Self {
        self.style.stroke_miterlimit = Some(limit);
        self
    }
}

impl<T: OpenEndedShape> Element<T> {
    pub fn stroke_linecap(mut self, linecap: LineCap) -> Self {
        self.style.stroke_linecap = Some(linecap);
        self
    }
}

impl<T: Hx> Element<T> {
    pub fn hx_ext(mut self) -> Self {
        if let Some(ref mut hx) = self.hx {
            hx.ext = Some("sse".to_string());
        } else {
            self.hx = Some(HxData::ext("sse"));
        }
        self
    }

    pub fn hx_sse_connect(mut self, endpoint: &str) -> Self {
        if let Some(ref mut hx) = self.hx {
            hx.connect = Some(endpoint.to_string());
        } else {
            self.hx = Some(HxData::connect(endpoint));
        }
        self
    }

    pub fn hx_sse_swap(mut self, event_name: &str) -> Self {
        if let Some(ref mut hx) = self.hx {
            hx.swap = Some(event_name.to_string());
        } else {
            self.hx = Some(HxData::swap(event_name));
        }
        self
    }
}

#[derive(Default, Debug)]
struct HxData {
    ext: Option<String>,
    connect: Option<String>,
    swap: Option<String>,
}

impl HxData {
    pub fn ext(value: &str) -> Self {
        Self {
            ext: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn connect(value: &str) -> Self {
        Self {
            connect: Some(value.to_string()),
            ..Default::default()
        }
    }

    pub fn swap(value: &str) -> Self {
        Self {
            swap: Some(value.to_string()),
            ..Default::default()
        }
    }
}

impl Visit for Option<HxData> {
    fn visit(&self, buffer: &mut Buffer) {
        if let Some(hx) = self {
            hx.visit(buffer);
        }
    }
}

impl Visit for HxData {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_attr_opt("hx-ext", &self.ext);
        buffer.push_attr_opt("hx-connect", &self.connect);
        buffer.push_attr_opt("hx-swap", &self.swap);
    }
}

#[derive(Debug)]
pub enum Transform {
    // x,y y is assumed 0 if leftout
    Translate(f64, f64),
    TranslateX(f64),
    TranslateY(f64),
    // x,y y is assumed same as x if leftout
    Scale(f64, f64),
    ScaleX(f64),
    ScaleY(f64),
    ScaleXY(f64),
    // angle, x, y x and y can be leftout then the rotateion is around the origin of the current
    // corrd system
    RotateXY(f64, f64, f64),
    Rotate(f64),
    // angle
    SkewX(f64),
    // angle
    SkewY(f64),

    // TODO: ..
    Matrix(i32, i32, i32, i32, i32, i32),
}

impl Visit for Transform {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            Transform::Translate(x, y) => format!("translate({} {})", x, y),
            Transform::TranslateX(x) => format!("translate({})", x),
            Transform::TranslateY(y) => format!("translate(0 {})", y),
            Transform::Scale(x, y) => format!("scale({} {})", x, y),
            Transform::ScaleX(x) => format!("scale({} 1)", x),
            Transform::ScaleY(y) => format!("scale(1 {})", y),
            Transform::ScaleXY(xy) => format!("scale({})", xy),
            _ => todo!("not yet implemented"),
        };

        buffer.push_str(&str);
    }
}
