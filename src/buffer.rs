use crate::{Options, Viewbox, visit::Visit};

pub struct Buffer {
    inner: String,
    tabs: u32,
    pub(crate) opts: Options,
    pub(crate) warnings: Vec<String>,
    pub(crate) viewbox: Viewbox,
}

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: String::with_capacity(cap),
            opts: Options::default(),
            tabs: 0,
            warnings: Vec::new(),
            viewbox: Viewbox::default(),
        }
    }

    /// pushes the start of a tag: "<tag .."
    pub fn push_tag(&mut self, tag: &str) {
        self.indent();
        self.inner.push('<');
        self.inner.push_str(tag);
        self.tabs += 1;
    }

    /// append the end marker of a tag ">"
    pub fn push_tag_end(&mut self) {
        self.inner.push('>');
        self.push_newline();
    }

    /// appends a closing tag like: "</tag>"
    pub fn push_tag_close(&mut self, tag: &str) {
        self.tabs = self.tabs.saturating_sub(1);
        self.indent();
        self.inner.push_str("</");
        self.inner.push_str(tag);
        self.inner.push('>');
        self.push_newline();
    }

    /// appends "/>"
    pub fn push_tag_self_close(&mut self) {
        self.inner.push_str("/>");
        self.push_newline();
        self.tabs = self.tabs.saturating_sub(1);
    }

    pub fn push_attr(&mut self, attr: &str, value: &impl Visit) {
        self.inner.push(' ');
        self.inner.push_str(attr);
        self.inner.push_str("=\"");
        value.visit(self);
        self.inner.push('"');
    }

    pub fn push_attr_opt(&mut self, attr: &str, value: &Option<impl Visit>) {
        if let Some(v) = value {
            self.push_attr(attr, v);
        }
    }

    pub fn push_attr_if(&mut self, attr: &str, value: &impl Visit, pred: impl Fn() -> bool) {}

    pub fn push_str(&mut self, str: &str) {
        self.inner.push_str(str);
    }

    pub fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    fn indent(&mut self) {
        if !self.opts.optimizations.remove_indent {
            let count = self.tabs;
            eprintln!("tabs: {}", count);
            for _ in 0..count {
                self.inner.push('\t');
            }
        }
    }

    #[allow(clippy::bool_comparison)]
    fn push_newline(&mut self) {
        if self.opts.optimizations.remove_newline == false {
            self.inner.push('\n');
        }
    }

    pub fn push_space(&mut self) {
        self.inner.push(' ');
    }

    pub fn str(&self) -> &str {
        &self.inner
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }

    pub fn push_warning(&mut self, warning: &str) {
        self.warnings.push(format!(
            "warning file: {}, line {}, {}",
            file!(),
            line!(),
            warning
        ));
    }
}
