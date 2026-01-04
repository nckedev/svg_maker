use std::{error::Error, fmt::Debug, fs::File, io::Write};

use svg_maker_derive::*;

use crate::{
    PreserveAspectRatio, Viewbox,
    buffer::Buffer,
    element::Element,
    marker_traits::{BaseElement, ChildOf, ElementKind},
    units::{AlignAspectRatio, Length, MeetOrSlice},
    visit::Visit,
};

#[derive(BaseStyle, ContainerElement)]
pub struct Svg {
    w: Option<Length>,
    h: Option<Length>,
    viewbox: Option<Viewbox>,
    version: String,
    preserve_aspect_ratio: Option<PreserveAspectRatio>,
    namespace: String,
    css: Option<String>,
    defs: Vec<Box<dyn ChildOf<Self>>>,
}
//
impl<T: ElementKind + Visit + 'static + Debug> ChildOf<Svg> for Element<T> {}
// impl ChildOf<Svg> for Element<Rect> {}
// impl ChildOf<Svg> for Element<Path> {}
// impl ChildOf<Svg> for Element<Use> {}

impl Element<Svg> {
    pub fn svg() -> Self {
        Element::new(Svg::default())
    }

    /// Sets the size of the svg, if width or height is less or equal to zero it will be ignored.
    /// ```
    /// # use svg_maker::shapes::svg::Svg;
    /// # use svg_maker::element::Element;
    /// use svg_maker::units::Percent;
    /// let svg = Element::svg().size(Percent(20), 10);
    /// let rendered = svg.render(None);
    /// assert!(rendered.contains(r#"width="20%""#));
    /// assert!(rendered.contains(r#"height="10""#));
    /// ```
    pub fn size<W, H>(mut self, w: W, h: H) -> Self
    where
        W: Into<Length>,
        H: Into<Length>,
    {
        let w = w.into();
        let h = h.into();
        if w.is_greater_than_zero() {
            self.w = Some(w);
        }
        if h.is_greater_than_zero() {
            self.h = Some(h);
        }
        self
    }

    /// Sets the svg version
    /// ```
    /// # use svg_maker::shapes::svg::Svg;
    /// # use svg_maker::element::Element;
    /// let svg = Element::svg().version("2");
    /// let rendered = svg.render(None);
    /// assert!(rendered.contains(r#"version="2""#));
    /// ```
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn viewbox<T: Into<f64>>(mut self, x: T, y: T, w: T, h: T) -> Self {
        self.viewbox = Some(Viewbox {
            x: x.into(),
            y: y.into(),
            w: w.into(),
            h: h.into(),
        });
        self
    }

    pub fn preserv_aspect_ratio(
        mut self,
        alignment: AlignAspectRatio,
        meet_or_slice: MeetOrSlice,
    ) -> Self {
        self.preserve_aspect_ratio = Some(PreserveAspectRatio {
            alignment,
            meet_or_slice,
        });
        self
    }

    pub fn css(mut self, css: &str) -> Self {
        self.css = Some(css.to_string());
        self
    }

    /// Add a reusable symbol, a symbol has its own viewbox
    pub fn symbol(self) -> Self {
        self
    }

    pub fn def<E>(mut self, el: Element<E>) -> Self
    where
        Element<E>: ChildOf<Svg> + BaseElement,
        E: ElementKind + 'static,
    {
        debug_assert!(
            el.get_id().is_some(),
            "a definition is useless without an id"
        );
        self.defs.push(Box::new(el));
        self
    }

    pub fn defs(mut self, elements: Vec<Box<dyn ChildOf<Svg>>>) -> Self {
        // debug_assert!(
        //     elements.iter().all(|e| e.get_id().is_some()),
        //     "a element definition is useless without an id"
        // );
        for element in elements {
            self.defs.push(element);
        }
        self
    }

    pub fn render_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut f = File::create(path)?;
        f.write_all(self.render(None).as_bytes())?;
        Ok(())
    }

    pub fn debug(&self, refresh_sec: u32) -> Result<(), Box<dyn Error>> {
        let mut f = File::create("test.html")?;
        let meta = if refresh_sec > 0 {
            format!(r#"<meta http-equiv="refresh" content="{}" />"#, refresh_sec)
        } else {
            "".to_string()
        };
        let buf = format!(
            r##"
        <html>
  <head>
    <title>SVG MAKER DEBUG</title>
    {meta}
  </head>
  <body style="background:black;">
    {}
    <div>
        <input type="range" min="0" max="100" id="slider_l" />
        <label for="slider_l">Lightness</label>
        <input type="range" min="0" max="360" id="slider_c" />
        <label for="slider_c">Chroma</label>
        <input type="range" min="0" max="360" id="slider_primary_h" />
        <label for="slider_primary_h">Primary</label>
        <input type="range" min="0" max="360" id="slider_secondary_h" />
        <label for="slider_secondary_h">Secodnary</label>
        <input type="range" min="0" max="360" id="slider_stroke_h" />
        <label for="slider_stroke_h">Stroke</label>
    </div>
    <script>

        function create_slider(name, target, f) {{
            const slider = document.getElementById(name);
            const root = document.documentElement;
            slider.addEventListener("input", (e) => {{
                const r = f !== null ? f(e.target.value) : e.target.value;
                root.style.setProperty(target, r);
            }});
        }}

        create_slider("slider_l", "--lightness", (v) => {{ return v + "%"; }} );
        create_slider("slider_c", "--chroma", (v) => {{ return v / 1000}});
        create_slider("slider_primary_h", "--primary_hue", null);
        create_slider("slider_secondary_h", "--secondary_hue", null);
        create_slider("slider_stroke_h", "--stroke_hue", null);
    </script>
  </body>
</html>
        "##,
            self.render(None)
        );
        f.write_all(buf.as_bytes())?;
        Ok(())
    }
}

impl ElementKind for Svg {
    const TAG: &'static str = "svg";
}

impl Visit for Svg {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.opts.optimizations.remove_unit_for_px = true;
        buffer.push_attr_opt("width", &self.w);
        buffer.push_attr_opt("height", &self.h);
        buffer.push_attr_opt("viewbox", &self.viewbox);
        if let Some(PreserveAspectRatio { alignment, .. }) = &self.preserve_aspect_ratio
            && *alignment != AlignAspectRatio::None
        {
            buffer.push_attr_opt("preserveAspectRatio", &self.preserve_aspect_ratio);
        }
        buffer.push_attr("version", &self.version);
        buffer.push_attr("xmlns", &self.namespace);
        buffer.push_tag_end();
        if let Some(css) = &self.css {
            buffer.push_tag("style");
            buffer.push_tag_end();
            buffer.push_str(css);
            buffer.push_tag_close("style");
        }
    }
}

impl Svg {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn render(&self) -> String {
        let mut buffer = Buffer::with_capacity(100);
        self.visit(&mut buffer);
        buffer.str().to_string()
    }
}

impl Default for Svg {
    fn default() -> Self {
        Self {
            w: None,
            h: None,
            version: "1.1".to_string(),
            namespace: "http://www.w3.org/2000/svg".to_string(),
            viewbox: Some(Viewbox {
                x: 0.,
                y: 0.,
                w: 100.,
                h: 100.,
            }),
            preserve_aspect_ratio: None,
            css: None,
            defs: Vec::new(),
        }
    }
}
