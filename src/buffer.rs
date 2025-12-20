use crate::visit::Visit;

pub struct Buffer {
    inner: String,
}

impl Buffer {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: String::with_capacity(cap),
        }
    }

    pub fn push_tag(&mut self, tag: &str) {
        self.inner.push('<');
        self.inner.push_str(tag);
        self.inner.push(' ');
    }

    pub fn push_tag_end(&mut self) {
        self.inner.push('>');
        self.push_newline();
    }

    pub fn push_tag_close(&mut self, tag: &str) {
        self.inner.push_str("</");
        self.inner.push_str(tag);
        self.inner.push('>');
    }

    pub fn push_self_close(&mut self) {
        self.inner.push_str("/>");
    }

    pub fn push_attr(&mut self, attr: &str, value: &impl Visit) {
        self.inner.push_str(attr);
        self.inner.push_str("=\"");
        value.visit(self);
        self.inner.push_str("\" ");
    }

    pub fn push_attr_opt(&mut self, attr: &str, value: &Option<impl Visit>) {
        if let Some(v) = value {
            self.push_attr(attr, v);
        }
    }

    pub fn push_str(&mut self, str: &str) {
        self.inner.push_str(str);
    }

    pub fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    pub fn push_tab(&mut self) {
        self.inner.push('\t');
    }
    pub fn push_newline(&mut self) {
        self.inner.push('\n');
    }
    pub fn push_space(&mut self) {
        self.inner.push(' ');
    }

    pub fn str(&self) -> &str {
        &self.inner
    }
}
