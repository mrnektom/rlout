use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::{
    gl::Env,
    view::{
        layout::MeasureSpec,
        view::{draw_view, View},
    },
};

pub struct Context<'a> {
    content: Option<Box<dyn View<'a>>>,
    env: Env,
}

impl<'a> Context<'a> {
    pub(crate) fn new(env: Env) -> Self {
        Context { content: None, env }
    }

    pub fn set_content(&mut self, content: Box<dyn View<'a>>) {
        self.content = Some(content);
        self.redraw();
    }

    fn redraw(&mut self) {
        self.env.draw(|canvas, size| {
            if let Some(view) = &mut self.content {
                view.on_measure(
                    MeasureSpec::Fixed(size.width),
                    MeasureSpec::Fixed(size.height),
                );
                draw_view(canvas, view);
            }
        });
    }

    pub fn launch_window(mut self, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
        event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } if window_id == self.env.window.id() => {
                match event {
                    WindowEvent::Resized(size) => self.env.on_resized(size),
                    WindowEvent::RedrawRequested => self.redraw(),
                    WindowEvent::CloseRequested => elwt.exit(),
                    _ => (),
                }
            }
            _ => (),
        })
    }
}
