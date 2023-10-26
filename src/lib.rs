mod context;
mod gl;
mod size;
pub mod view;

use std::error::Error;


use context::Context;
use gl::init_gl;
use winit::{
    event_loop::{EventLoop, self},
    window::WindowBuilder,
};

pub fn create_window_context() -> Result<(Context, EventLoop<()>), Box<dyn Error>> {
    let el = EventLoop::new().unwrap();
    let window_builder =
        WindowBuilder::new().with_inner_size(winit::dpi::LogicalSize::new(700.0, 300.0));
    let env = init_gl(&el, window_builder)?;

    Ok((Context::new(env, ), el))
}
