

pub(crate) trait LogicalSize<P> {
    fn dp_size(self) -> winit::dpi::LogicalSize<P>;
}

impl<P> LogicalSize<P> for (P, P) {
    fn dp_size(self) -> winit::dpi::LogicalSize<P> {
        winit::dpi::LogicalSize::new(self.0, self.1)
    }
}
