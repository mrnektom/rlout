use crate::view::node::Node;

use super::Layout;

pub struct Column {
    children: Vec<Node>,
}

impl Column {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl Layout for Column {
    fn on_measure(
        &self,
        width: super::MeasureSpec,
        height: super::MeasureSpec,
    ) -> crate::view::Size {
        todo!()
    }

    fn add_child(&mut self, node: Node) {
        self.children.push(node);
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }

    fn children(&self) -> &Vec<Node> {
        self.children.as_ref()
    }
}
