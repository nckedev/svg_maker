use svg_maker_derive::{BaseStyle, OpenEndedShape};

use crate::{
    buffer::Buffer, element::Element, marker_traits::ElementKind, units::Coord, visit::Visit,
};

#[derive(Debug, BaseStyle, OpenEndedShape)]
pub struct Polyline {
    points: Vec<Coord>,
}

impl Element<Polyline> {
    pub fn polyline() -> Self {
        let polyline = Polyline { points: vec![] };
        Element::new(polyline)
    }

    /// Appends a vector of points to the list
    pub fn add_points<P: Into<Coord>>(mut self, points: Vec<P>) -> Self {
        let mut vec = vec![];
        for p in points {
            vec.push(p.into());
        }
        self.points.append(&mut vec);
        self
    }

    /// Appends a point to the list
    pub fn add_point<P: Into<Coord>>(mut self, point: P) -> Self {
        self.points.push(point.into());
        self
    }
}

impl ElementKind for Polyline {
    const TAG: &'static str = "polyline";
}

impl Visit for Polyline {
    fn visit(&self, buffer: &mut Buffer) {
        if !self.points.is_empty() {
            buffer.push_attr("points", &self.points);
        }
    }
}
