use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use crate::{
    BaseStyle, Command, Line, Path, Raw, Shape,
    buffer::Buffer,
    color::Color,
    marker_traits::{BaseElement, OpenEndedShape},
    style::{LineCap, Style},
    units::{Coord, Length, XCoord, YCoord},
    visit::Visit,
};

pub struct ElementBuilder;
impl ElementBuilder {
    pub fn path() -> Element<Path> {
        // ElementBuilder::bootstrap(Path::new())
        Element::new(Path::new())
    }

    pub fn line(
        x1: impl Into<Length>,
        x2: impl Into<Length>,
        y1: impl Into<Length>,
        y2: impl Into<Length>,
    ) -> Element<Line> {
        Element::new(Line::new(x1, x2, y1, y2))
    }
    // pub fn line_from_coord(start: Coord, end: Coord) -> Element<Line> {}
    //
    pub fn raw(str: &str) -> Element<Raw> {
        Element::new(Raw {
            inner: str.to_string(),
        })
    }
}

pub struct Element<T: Sized> {
    pub id: Option<String>,
    class: Option<String>,
    /// NOTE: this style object contains all possible styles, and some might not be applicable to
    /// the current element,
    pub(crate) style: Style,
    transforms: Option<Vec<Transform>>,
    kind: T,
}

impl<T: Shape + Visit> From<T> for Element<T> {
    fn from(value: T) -> Self {
        Element::new(value)
    }
}

impl<T: Visit + 'static> BaseElement for Element<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn from_shape<S>(shape: S) -> Element<S>
    where
        S: Sized + Visit + crate::marker_traits::Shape,
    {
        Element::from(shape)
    }
}

impl Element<Path> {
    pub fn push(mut self, command: Command) -> Self {
        self.kind.push(command);
        self
    }

    pub fn push_commands(mut self, commands: &mut Vec<Command>) -> Self {
        self.kind.append(commands);
        self
    }

    pub fn move_to(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.kind.push(Command::MoveTo(Coord(x.into(), y.into())));
        self
    }

    pub fn move_to_relative(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.kind
            .push(Command::MoveToRelative(Coord(x.into(), y.into())));
        self
    }

    pub fn line(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.kind.push(Command::Line(Coord(x.into(), y.into())));
        self
    }

    pub fn line_relative(mut self, x: impl Into<XCoord>, y: impl Into<YCoord>) -> Self {
        self.kind
            .push(Command::LineRelative(Coord(x.into(), y.into())));
        self
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

impl<T: Visit> Visit for Element<T> {
    fn visit(&self, buffer: &mut Buffer) {
        self.kind.visit(buffer);
        buffer.push_attr_opt("id", &self.id);
        buffer.push_attr_opt("class", &self.class);
        buffer.push_attr_opt("transform", &self.transforms);
        self.style.visit(buffer);
        //TODO: if the element has child elementes like animations, include them before closing, if not do a
        //selfclose tag
        buffer.push_str("/>\n");
    }
}

impl<T: Visit> Element<T> {
    pub fn new(kind: T) -> Self {
        Self {
            id: None,
            class: None,
            style: Style::default(),
            transforms: None,
            kind,
        }
    }

    pub fn inner(&self) -> &T {
        &self.kind
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.kind
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

    pub fn stroke(mut self, color: Color) -> Self {
        self.style.stroke = Some(color);
        self
    }
}

impl<T: OpenEndedShape> Element<T> {
    pub fn stroke_linecap(mut self, linecap: LineCap) -> Self {
        self.style.stroke_linecap = Some(linecap);
        self
    }
}

pub enum Transform {
    // x,y y is assumed 0 if leftout
    Translate(i32, i32),
    TranslateX(i32),
    TranslateY(i32),
    // x,y y is assumed same as x if leftout
    Scale(i32, i32),
    ScaleX(i32),
    ScaleY(i32),
    ScaleXY(i32),
    // angle, x, y x and y can be leftout then the rotateion is around the origin of the current
    // corrd system
    RotateXY(f32, i32, i32),
    Rotate(f32),
    // angle
    SkewX(f32),
    // angle
    SkewY(f32),

    Matrix(i32, i32, i32, i32, i32, i32),
}

impl Visit for Transform {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            Transform::Translate(x, y) => format!("tranlate({} {}) ", x, y),
            Transform::TranslateX(x) => format!("tranlate({}) ", x),
            Transform::TranslateY(y) => format!("tranlate(0 {}) ", y),
            Transform::Scale(x, y) => format!("scale({} {}) ", x, y),
            Transform::ScaleX(x) => format!("scale({} 1) ", x),
            Transform::ScaleY(y) => format!("scale(1 {}) ", y),
            Transform::ScaleXY(xy) => format!("scale({}) ", xy),
            _ => todo!("not yet implemented"),
        };

        buffer.push_str(&str);
    }
}
