

use super::{view::View, layout::{Layout, MeasureSpec}, Size, Canvas};

pub enum Node {
    View(Box<dyn View>),
    Layout(Box<dyn Layout>)
}

impl Node {
    fn measure(&self, width: MeasureSpec, height: MeasureSpec) -> Size {
        match self {
            Self::View(v) => v.on_measure(width, height),
            Self::Layout(l) => l.on_measure(width, height)
        }
    }

    pub fn draw(&self, canvas: &Canvas) {
        match self {
            Node::View(v) => v.on_draw(canvas),
            Node::Layout(_) => {}
        }
    }
}


