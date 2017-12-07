use std::collections::BTreeMap;
use vnode::{BasicVNode, AbstractVNode};

#[macro_export]
macro_rules! vdmap {
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($key.to_string(), $value.to_string());
            )*
            m
        }
    }
}

pub struct BuildElementOptions {
    pub tag: String,
    pub props: BTreeMap<String, String>,
    pub style: BTreeMap<String, String>,
    pub children: Vec<AbstractVNode>
}

pub fn build_element(opt: BuildElementOptions) -> AbstractVNode {
    let mut node = BasicVNode::new_element(opt.tag.as_str());
    node.props = opt.props;
    node.style = opt.style;
    let mut node = AbstractVNode::new(node);

    for c in opt.children {
        node.append_child(c);
    }

    node
}

pub fn build_text<T: AsRef<str>>(t: T) -> AbstractVNode {
    AbstractVNode::new(BasicVNode::new_text(t.as_ref()))
}
