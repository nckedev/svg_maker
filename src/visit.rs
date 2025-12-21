use crate::buffer::Buffer;

/// trait for visitning a node and append the buffer.
/// the appended string should always include a space as last character, except for the primitive
/// types u32, f32, etc,
pub trait Visit {
    fn visit(&self, buffer: &mut Buffer);
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
        for x in self {
            x.visit(buffer);
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

impl_visit!(f32, u64, u32, u16, u8);
