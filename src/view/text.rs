use rlout_runtime::{MaybeDerivedFrom, derive};
use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    FontMgr, Paint, Point, Rect, Font, font,
};

use super::{view::{View, Draw, SuperView}, layout::Layout, Size};

#[derive(Debug, Clone)]
pub struct Text {
    text: String,

    super_view: SuperView
}

impl Text {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
            super_view: SuperView::new()
        }
    }

    pub fn set_text(&mut self,text: &str) {
        self.text = text.to_string();
    }

    fn build_paragraph(&self) -> Paragraph {
        let mut font = Font::default();
        font.set_size(20.0);
    
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);

        let paragraph_style = ParagraphStyle::new();
        

        let mut text_style = TextStyle::new();
        
        text_style.set_foreground_paint(&Paint::default());

        
        ParagraphBuilder::new(&paragraph_style, font_collection)
            .push_style(&text_style)
            .add_text(self.text.as_str())
            .build()
    }
}

derive!(Text, dyn Draw + 'a);

derive!(Text, dyn Layout<'a> + 'a, false);


impl Draw for Text {
    fn draw(&self, canvas: &super::Canvas) {
        let mut paragraph = self.build_paragraph();
        paragraph.layout(100.0);
        paragraph.paint(canvas, Point::default());
    }
}

impl View<'_> for Text {
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
        let mut paragraph = self.build_paragraph();
        let width = match width {
            super::layout::MeasureSpec::Fixed(width) => {
                paragraph.layout(width);
                width
            }
            super::layout::MeasureSpec::AtMost(width) => {
                paragraph.layout(width);
                width.min(paragraph.max_intrinsic_width())
            },
            super::layout::MeasureSpec::Unspecified => {
                paragraph.layout(f32::MAX);
                f32::MAX
            }
        };

        let height = match height {
            super::layout::MeasureSpec::Fixed(height) => {
                height
            },
            super::layout::MeasureSpec::AtMost(height) => height.min(paragraph.height()),
            super::layout::MeasureSpec::Unspecified => todo!(),
        };

        (width, height).into()
    }
}
