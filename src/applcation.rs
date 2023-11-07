use skia_safe::{canvas, Rect};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{
    gl::{init_gl, Env},
    view::{
        layout::MeasureSpec,
        view::{draw_view, measure_view, View, layout_view},
    },
};

pub struct Application<'a> {
    content: Option<Box<dyn View<'a>>>,
    env: Env,
    event_loop: Option<EventLoop<()>>,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Self::from_window_builder(WindowBuilder::default())
    }

    pub fn from_window_builder(window_builder: WindowBuilder) -> Self {
        let event_loop = EventLoop::new().unwrap();
        Self {
            content: None,
            env: init_gl(&event_loop, window_builder).unwrap(),
            event_loop: Some(event_loop),
        }
    }

    pub fn set_content(&mut self, view: Box<dyn View<'a>>) {
        self.content = Some(view);
    }

    fn redraw(&mut self) {
        self.env.draw(|canvas, size| {
            if let Some(content) = &mut self.content {
                measure_view(
                    content,
                    MeasureSpec::Fixed(size.width),
                    MeasureSpec::Fixed(size.height),
                );
                layout_view(content);
                draw_view(canvas, content);
            }
        });
    }

    pub fn launch(mut self) {
        let mut event_loop = Option::<EventLoop<()>>::None;

        std::mem::swap(&mut event_loop, &mut self.event_loop);

        event_loop
            .unwrap()
            .run(move |event, elwt| match event {
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
            .unwrap();
    }
}
