use std::collections::HashMap;

use crate::{Visit, buffer::Buffer, color::Color, units::Length};

#[derive(Default, Debug)]
pub struct Style {
    pub fill: Option<Color>,
    pub fill_opacity: Option<f32>,
    pub fill_rule: Option<FillRule>,
    pub stroke: Option<Color>,

    pub stroke_dasharray: Option<Vec<Length>>,
    pub stroke_dashoffset: Option<Length>,

    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub stroke_miterlimit: Option<f32>,
    pub stroke_opacity: Option<f32>,

    pub stroke_width: Option<Length>,

    pub kv: HashMap<String, String>,
}

impl Visit for Style {
    fn visit(&self, buffer: &mut Buffer) {
        macro_rules! visit_if_not_none {
            ($ident:ident, $str:literal) => {
                if let Some($ident) = &self.$ident {
                    let str = format!("{}: ", $str);
                    buffer.push_str(&str);
                    $ident.visit(buffer);
                    buffer.push_str("; ");
                }
            };
        }

        // do nothing if there is no styling option set.
        if let Style {
            fill: None,
            fill_opacity: None,
            fill_rule: None,
            stroke: None,
            stroke_dasharray: None,
            stroke_dashoffset: None,
            stroke_linecap: None,
            stroke_linejoin: None,
            stroke_miterlimit: None,
            stroke_opacity: None,
            stroke_width: None,
            kv,
        } = &self
            && kv.is_empty()
        {
            return;
        }

        buffer.push_str(r##" style=""##);

        visit_if_not_none!(fill, "fill");
        visit_if_not_none!(fill_opacity, "fill-opacity");
        visit_if_not_none!(fill_rule, "fill-rule");
        visit_if_not_none!(stroke, "stroke");
        visit_if_not_none!(stroke_width, "stroke-width");
        visit_if_not_none!(stroke_linejoin, "stroke-linejoin");
        visit_if_not_none!(stroke_dasharray, "stroke-dasharray");
        visit_if_not_none!(stroke_dashoffset, "stroke-dashoffset");
        visit_if_not_none!(stroke_linecap, "stroke-linecap");
        visit_if_not_none!(stroke_miterlimit, "stroke-miterlimit");
        visit_if_not_none!(stroke_opacity, "stroke_opacity");
        for (k, v) in &self.kv {
            buffer.push_str(&format!("{}:{}; ", k, v));
        }

        buffer.pop(); //remove the last whitespace
        buffer.push_str(r##"""##);
    }
}

#[derive(Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

impl Visit for FillRule {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            FillRule::NonZero => "nonzero",
            FillRule::EvenOdd => "evenodd",
        };
        buffer.push_str(str);
    }
}

#[derive(Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Visit for LineCap {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        };
        buffer.push_str(str);
    }
}

#[derive(Debug)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl Visit for LineJoin {
    fn visit(&self, buffer: &mut Buffer) {
        let str = match self {
            LineJoin::Miter => "miter",
            LineJoin::Round => "round",
            LineJoin::Bevel => "bevel",
        };
        buffer.push_str(str);
    }
}
