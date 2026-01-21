use svg_maker_derive::{BaseStyle, ClosedShape};

use crate::{
    buffer::Buffer, element::Element, marker_traits::ElementKind, units::Coord, visit::Visit,
};

#[derive(Debug, BaseStyle, ClosedShape)]
pub struct Polygon {
    points: Vec<Coord>,
}

impl Element<Polygon> {
    pub fn polygon() -> Self {
        let polygon = Polygon { points: vec![] };
        Element::new(polygon)
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

impl ElementKind for Polygon {
    const TAG: &'static str = "polygon";
}

impl Visit for Polygon {
    fn visit(&self, buffer: &mut Buffer) {
        if !self.points.is_empty() {
            buffer.push_attr("points", &self.points);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn polygon() {
        let points = vec![
            Coord::from((10, 10)),
            Coord::from((20, 10)),
            Coord::from((20, 20)),
            Coord::from((10, 20)),
        ];
        let polygon = Element::polygon().add_points(points);
        let expected = r#"<polygon points="10,10 20,10 20,20 10,20"/>"#.to_string() + "\n";
        assert_eq!(polygon.render(None), expected);

        let polygon = polygon.add_point((1, 1));
        let expected = r#"<polygon points="10,10 20,10 20,20 10,20 1,1"/>"#.to_string() + "\n";
        assert_eq!(polygon.render(None), expected);
    }
}
