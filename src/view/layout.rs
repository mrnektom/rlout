pub mod column;

use std::fmt::Debug;

use skia_safe::Rect;

use super::{view::View, Size};

#[derive(Debug)]
pub struct SuperLayout<'a> {
    children: Vec<Box<dyn View<'a>>>,
}

impl<'a> SuperLayout<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

pub trait Layout<'a>: Debug {
    fn super_layout(&self) -> &SuperLayout<'a>;
    fn super_layout_mut(&mut self) -> &mut SuperLayout<'a>;

    fn on_measure(&self, width: MeasureSpec, height: MeasureSpec) -> Size {
        let width = match width {
            MeasureSpec::Fixed(width) => width,
            MeasureSpec::AtMost(_) => 0.0,
            MeasureSpec::Unspecified => 0.0,
        };
        let height = match height {
            MeasureSpec::Fixed(height) => height,
            MeasureSpec::AtMost(_) => 0.0,
            MeasureSpec::Unspecified => 0.0,
        };

        (width, height).into()
    }

    fn child_count(&self) -> usize {
        self.children().len()
    }

    fn children(&self) -> &Vec<Box<dyn View<'a>>> {
        &self.super_layout().children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn View<'a>>> {
        &mut self.super_layout_mut().children
    }

    fn add_child(&mut self, child: Box<dyn View<'a>>) {
        self.children_mut().push(child);
    }

    fn on_layout(&mut self) {}
}

trait LayoutParams {}

#[derive(Debug, Clone, Copy)]
pub enum MeasureSpec {
    Fixed(f32),
    AtMost(f32),
    Unspecified,
}
