use super::{Size, Canvas, layout::MeasureSpec};

pub trait View {
    #[allow(unused)]
    fn on_draw(&self, canvas: &Canvas) {}

    fn on_measure(&self, width: MeasureSpec, height: MeasureSpec) -> Size;
}
