struct HorizontalLayout {
    padding: u32,
}

trait Layout {
    fn measure_x(&self) -> usize {
        0
    }
    fn measure_y(&self) -> usize {
        0
    }
}
