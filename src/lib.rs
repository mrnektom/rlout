mod context;
mod gl;
mod size;
pub mod view;
pub mod applcation;

use std::error::Error;


use context::Context;
use gl::init_gl;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub fn create_window_context<'a>() -> Result<(Context<'a>, EventLoop<()>), Box<dyn Error>> {
    let el = EventLoop::new().unwrap();
    let window_builder =
        WindowBuilder::new().with_inner_size(winit::dpi::LogicalSize::new(700.0, 300.0));
    let env = init_gl(&el, window_builder)?;

    Ok((Context::new(env, ), el))
}
