use skia_safe::{
    textlayout::{FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle},
    FontMgr, Paint, Point,
};

use super::view::View;

#[derive(Debug, Clone)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Self {
        Self {
            text: String::from(""),
        }
    }

    pub fn set_text(&mut self,text: &str) {
        self.text = text.to_string();
    }

    fn build_paragraph(&self) -> Paragraph {
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

impl View for Text {
    fn on_draw(&self, canvas: &super::Canvas) {
        let mut paragraph = self.build_paragraph();
        paragraph.layout(100.0);
        paragraph.paint(canvas, Point::default());
    }

    fn on_measure(
        &self,
        width: super::layout::MeasureSpec,
        height: super::layout::MeasureSpec,
    ) -> super::Size {
        let mut paragraph = self.build_paragraph();
        let width = match width {
            super::layout::MeasureSpec::Fixed(width) => {
                paragraph.layout(width as f32);
                width as f32
            }
            super::layout::MeasureSpec::AtMost(_) => todo!(),
            super::layout::MeasureSpec::Unspecified => {
                paragraph.layout(f32::MAX);
                f32::MAX
            }
        };

        let height = match height {
            super::layout::MeasureSpec::Fixed(height) => {
                height as f32
            },
            super::layout::MeasureSpec::AtMost(_) => todo!(),
            super::layout::MeasureSpec::Unspecified => todo!(),
        };

        (width, height).into()
    }
}
