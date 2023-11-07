use std::error::Error;

use std::{ffi::CString, num::NonZeroU32};

use gl::types::*;
use gl_rs as gl;
use glutin::prelude::NotCurrentGlContext;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::{ISize, Size};
use winit::dpi::PhysicalSize;

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use skia_safe::{
    gpu::{self, backend_render_targets, gl::FramebufferInfo, SurfaceOrigin},
    Color, ColorType, Surface,
};

use crate::view::Canvas;

pub(crate) fn init_gl(
    event_loop: &EventLoop<()>,
    window_builder: WindowBuilder,
) -> Result<Env, Box<dyn Error>> {
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    let (window, gl_config) = display_builder.build(event_loop, template, |configs| {
        configs
            .reduce(|accum, config| {
                let transparecy_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);
                if transparecy_check || config.num_samples() > accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .unwrap()
    })?;
    let window = window.expect("Lol, window is not created");
    let raw_window_handle = window.raw_window_handle();
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    let not_current_gl_context = unsafe {
        gl_config
            .display()
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config
                    .display()
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    };

    let (width, height): (u32, u32) = window.inner_size().into();

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    );

    let gl_surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .expect("Could not create gl window surface")
    };

    let gl_context = not_current_gl_context
        .make_current(&gl_surface)
        .expect("Could not make GL context current when setting up skia renderer");

    gl::load_with(|s| {
        gl_config
            .display()
            .get_proc_address(CString::new(s).unwrap().as_c_str())
    });
    let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
        if name == "eglGetCurrentDisplay" {
            return std::ptr::null();
        }
        gl_config
            .display()
            .get_proc_address(CString::new(name).unwrap().as_c_str())
    })
    .expect("Could not create interface");

    let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
        .expect("Could not create direct context");

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
    };

    let num_samples = gl_config.num_samples() as usize;
    let stencil_size = gl_config.stencil_size() as usize;

    let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

    let env = Env {
        surface,
        gl_surface,
        gl_context,
        gr_context,
        window,
        fb_info,
        num_samples,
        stencil_size,
    };

    Ok(env)
}

pub(crate) struct Env {
    pub(crate) surface: Surface,
    gl_surface: GlutinSurface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    pub(crate) window: Window,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
}

impl Env {
    pub(crate) fn on_resized(&mut self, physical_size: PhysicalSize<u32>) {
        self.surface = create_surface(
            &self.window,
            self.fb_info,
            &mut self.gr_context,
            self.num_samples,
            self.stencil_size,
        );
        /* First resize the opengl drawable */
        let (width, height): (u32, u32) = physical_size.into();

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );
    }

    pub(crate) fn draw<F>(&mut self, on_draw: F)
    where
        F: FnOnce(&Canvas, Size),
    {
        let size = Size::from((
            self.gl_surface.width().expect("") as i32,
            self.gl_surface.height().expect("") as i32,
        ));

        let canvas = self.surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        on_draw(canvas, size);
        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
}

fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();

    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
