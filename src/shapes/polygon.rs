use svg_maker_derive::{BaseStyle, ClosedShape};

use crate::{
    buffer::Buffer, element::Element, marker_traits::ElementKind, units::Coord, visit::Visit,
};

#[derive(Debug, BaseStyle, ClosedShape)]
pub struct Polygon {
    points: Vec<Coord>,
}

impl Element<Polygon> {
    pub fn polygon<P: Into<Coord>>(points: Vec<P>) -> Self {
        let mut polygon = Polygon { points: vec![] };
        for p in points {
            polygon.points.push(p.into());
        }

        Element::new(polygon)
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
        let polygon = Element::polygon(points).render(None);
        let expected = r#"<polygon points="10,10 20,10 20,20 10,20"/>"#.to_string() + "\n";
        assert_eq!(polygon, expected);
    }
}
