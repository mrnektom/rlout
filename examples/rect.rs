use std::error::Error;

use rlout::{create_window_context, view::{text::Text, node::Node, layout::{column::Column, Layout}}};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut context, event_loop) = create_window_context()?;

    let mut column = Column::new();

    let mut text = Text::new();
    text.set_text("text");

    for _ in 0..3 {
        column.add_child(Node::View(Box::new(text.clone())));
    }

    context.set_content(Node::Layout(Box::new(column)));

    context.launch_window(event_loop)?;
    Ok(())
}
