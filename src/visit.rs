use crate::buffer::Buffer;

/// trait for visitning a node and append the buffer.
/// the appended string should always include a space as last character, except for the primitive
/// types u32, f32, etc,
pub trait Visit {
    fn visit(&self, buffer: &mut Buffer);
    fn visit_extra(&self, buffer: &mut Buffer, prefix: &str, suffix: &str) {
        buffer.push_str(prefix);
        self.visit(buffer);
        buffer.push_str(suffix);
    }
    fn visit_prefix(&self, buffer: &mut Buffer, prefix: &str) {
        buffer.push_str(prefix);
        self.visit(buffer);
    }

    fn vist_suffix(&self, buffer: &mut Buffer, suffix: &str) {
        self.visit(buffer);
        buffer.push_str(suffix);
    }
}

impl Visit for String {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(self);
    }
}

impl Visit for &str {
    fn visit(&self, buffer: &mut Buffer) {
        buffer.push_str(self);
    }
}

impl<T: Visit> Visit for Vec<T> {
    fn visit(&self, buffer: &mut Buffer) {
        for (i, x) in self.iter().enumerate() {
            if i == self.len().saturating_sub(1) {
                x.visit(buffer);
            } else {
                x.vist_suffix(buffer, " ");
            }
        }
    }
}

macro_rules! impl_visit {
    ($($t:ty),*) => {
        $(
            impl Visit for $t {
                fn visit(&self, buffer: &mut Buffer) {
                    buffer.push_str(&self.to_string());
                }
            }
        )*
    };
}

impl_visit!(f64, f32, u64, u32, u16, u8, i64, i32, i16, i8, usize, isize);
