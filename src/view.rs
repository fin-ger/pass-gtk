use vgtk::{VNode, Component};

pub trait View: Component {
    fn view(&self) -> VNode<Self>;
}
