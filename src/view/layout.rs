pub mod column;

use super::{node::Node, Size};

pub trait Layout {
    fn on_measure(&self, width: MeasureSpec, height: MeasureSpec) -> Size;

    fn child_count(&self) -> usize;

    fn children(&self) -> &Vec<Node>;

    fn add_child(&mut self, node: Node);
}

trait LayoutParams {}

pub enum MeasureSpec {
    Fixed(usize),
    AtMost(usize),
    Unspecified,
}
