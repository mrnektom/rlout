use rlout_runtime::derive;
use skia_safe::Rect;

use crate::view::{layout::MeasureSpec, view::measure_view};

use super::{
    layout::{Layout, SuperLayout},
    view::{Draw, SuperView, View},
};

#[derive(Debug)]
pub struct Column<'a> {
    super_view: SuperView,
    super_layout: SuperLayout<'a>,
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self {
            super_view: SuperView::new(),
            super_layout: SuperLayout::new(),
        }
    }
}

impl<'a> View<'a> for Column<'a> {
    fn super_view(&self) -> &SuperView {
        &self.super_view
    }
    fn super_view_mut(&mut self) -> &mut SuperView {
        &mut self.super_view
    }

    fn on_measure(
        &mut self,
        width: super::layout::MeasureSpec,
        height: super::layout::MeasureSpec,
    ) -> super::Size {
        let mut computed_width = 0.0f32;
        let mut computed_height = 0.0f32;

        let mut available_height = match height {
            MeasureSpec::Fixed(height) => height,
            MeasureSpec::AtMost(height) => height,
            MeasureSpec::Unspecified => f32::MAX,
        };

        let child_width_spec = match width {
            MeasureSpec::Fixed(width) => MeasureSpec::AtMost(width),
            MeasureSpec::AtMost(width) => MeasureSpec::AtMost(width),
            MeasureSpec::Unspecified => MeasureSpec::Unspecified,
        };

        for child in self.children_mut() {
            let child_height = MeasureSpec::AtMost(available_height);
            let child_size = measure_view(child, child_width_spec, child_height);
            available_height -= child_size.height;
            computed_height += child_size.height;
            computed_width = f32::max(computed_width, child_size.width);
        }

        let width = match width {
            MeasureSpec::Unspecified => computed_width,
            MeasureSpec::Fixed(width) => width,
            MeasureSpec::AtMost(width) => computed_width.max(width),
        };

        let height = match height {
            MeasureSpec::Unspecified => computed_height,
            MeasureSpec::Fixed(height) => height,
            MeasureSpec::AtMost(height) => computed_height.max(height),
        };

        (width, height).into()
    }
}

impl<'a> Layout<'a> for Column<'a> {
    fn super_layout(&self) -> &SuperLayout<'a> {
        &self.super_layout
    }
    fn super_layout_mut(&mut self) -> &mut SuperLayout<'a> {
        &mut self.super_layout
    }
    fn on_layout(&mut self) {
        let mut y = 0.0f32;
        for child in self.children_mut() {
            let child_size = child.size();
            let bounds = Rect::from_xywh(0.0, y, child_size.width, child_size.height);
            child.layout(bounds);
            y += child_size.height;
        }
    }
}

derive!(Column<'a>, dyn Draw + 'a, false);
derive!(Column<'a>, dyn Layout<'a> + 'a);
