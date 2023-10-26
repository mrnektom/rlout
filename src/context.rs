use winit::{event_loop::EventLoop, error::EventLoopError, event::{Event, WindowEvent}};

use crate::{gl::Env, view::node::Node};

pub struct Context {
    root_node: Option<Node>,
    env: Env,
}

impl Context {
    pub(crate) fn new(env: Env) -> Self {
        Context {
            root_node: None,
            env,
        }
    }

    pub fn set_content(&mut self, node: Node) {
        self.root_node = Some(node);
        self.redraw();
    }

    fn redraw(&mut self) {
        self.env.draw(|canvas| {
            if let Some(node) = &self.root_node {
                node.draw(canvas);
            }
        });
    }

    pub fn launch_window(mut self, event_loop: EventLoop<()>) -> Result<(), EventLoopError> {
        event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } if window_id == self.env.window.id() => match event {
                WindowEvent::Resized(size) => self.env.on_resized(size),
                WindowEvent::RedrawRequested => self.redraw(),
                WindowEvent::CloseRequested => elwt.exit(),
                _ => (),
            },
            _ => (),
        })
    }
}
