use std::{any::Any, fmt::Debug};

use rlout_runtime::MaybeDerivedFrom;
use skia_safe::{Color, Paint, Rect};

use super::{
    layout::{self, Layout, MeasureSpec},
    Canvas, Size,
};

#[derive(Debug, Clone)]
pub struct ViewStyles {
    background: Color,
}

impl ViewStyles {
    fn new() -> Self {
        Self {
            background: Color::TRANSPARENT,
        }
    }

    pub fn set_background(&mut self, background: Color) -> &mut Self {
        self.background = background;
        self
    }

    pub fn background(&self) -> Color {
        self.background
    }
}

#[derive(Debug, Clone)]
pub struct SuperView {
    view_styles: ViewStyles,
    bounds: Rect,
}

impl SuperView {
    pub fn new() -> Self {
        Self {
            view_styles: ViewStyles::new(),
            bounds: Rect::new_empty(),
        }
    }

    pub fn set_background(&mut self, background: Color) -> &mut Self {
        self.view_styles.background = background;
        self
    }

    pub fn layout(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}

pub trait View<'a>:
    Debug + MaybeDerivedFrom<'a, dyn Layout<'a> + 'a> + MaybeDerivedFrom<'a, dyn Draw + 'a>
{
    fn super_view(&self) -> &SuperView;
    fn super_view_mut(&mut self) -> &mut SuperView;

    fn on_measure(
        &mut self,
        width: super::layout::MeasureSpec,
        height: super::layout::MeasureSpec,
    ) -> super::Size;

    fn layout(&mut self, bounds: Rect) {
        self.super_view_mut().bounds = bounds;
    }
    fn size(&self) -> Size {
        self.super_view().bounds.size()
    }
}

pub trait Draw {
    fn draw(&self, canvas: &Canvas);
}

pub(crate) fn draw_view<'a>(canvas: &Canvas, view: &Box<dyn View<'a>>) {
    let mut paint = Paint::default();

    canvas.save();
    let SuperView {
        view_styles: ViewStyles { background },
        bounds,
    } = view.super_view();
    canvas.clip_rect(bounds, None, None);
    canvas.translate((bounds.left, bounds.top));
    println!("draw view: bounds: {bounds:?}, background: {background:?}");
    paint.set_color(background.clone());
    canvas.draw_rect(Rect::from_wh(bounds.width(), bounds.height()), &paint);

    {
        let layout: Option<&dyn Layout> = view.try_as();
        println!("layout : {layout:?}");
        if let Some(layout) = layout {
            for view in layout.children() {
                draw_view(canvas, view);
            }
        }
    }

    {
        let draw: Option<&dyn Draw> = view.try_as();
        if let Some(draw) = draw {
            draw.draw(canvas);
        }
    }

    canvas.restore();
}

pub(crate) fn measure_view(
    view: &mut Box<dyn View>,
    width: MeasureSpec,
    height: MeasureSpec,
) -> Size {
    let size = view.on_measure(width, height);
    let bounds = &mut view.super_view_mut().bounds;
    bounds.right = bounds.left + size.width;
    bounds.bottom = bounds.top + size.height;
    size
}

pub fn layout_view(view: &mut Box<dyn View>) -> Option<()> {
    let layout: &mut dyn Layout = view.try_as_mut()?;
    layout.on_layout();
    Some(())
}
