use std::error::Error;

use rlout::{view::{text::Text, view::View, column::Column, layout::Layout}, applcation::{self, Application}};
use skia_safe::Color;

fn main() -> Result<(), Box<dyn Error>> {
    let mut applcation = Application::new();

    let mut column = Column::new();

    let colors = vec![Color::RED, Color::GREEN, Color::BLUE];

    for (i, color) in colors.iter().enumerate() {
        let mut text = Text::new();
        text.set_text(format!("text: {i}").as_str());
        
        text.super_view_mut().set_background(*color);
        column.add_child(Box::new(text));
    }

    applcation.set_content(Box::new(column));

    applcation.launch();
    Ok(())
}
