use std::collections::BTreeMap;
use dom;

#[derive(Clone)]
pub struct BasicVNode {
    pub tag: Option<String>,
    pub props: BTreeMap<String, String>,
    pub style: BTreeMap<String, String>,
    pub text: Option<String>
}

pub struct InternalVNode<T: dom::Node> {
    node: BasicVNode,
    dom_node: T,
    children: Vec<InternalVNode<T>>
}

pub struct AbstractVNode {
    node: BasicVNode,
    children: Vec<AbstractVNode>
}

impl BasicVNode {
    pub fn new_element<T: AsRef<str>>(tag: T) -> BasicVNode {
        BasicVNode {
            tag: Some(tag.as_ref().to_string()),
            props: BTreeMap::new(),
            style: BTreeMap::new(),
            text: None
        }
    }

    pub fn new_text<T: AsRef<str>>(t: T) -> BasicVNode {
        BasicVNode {
            tag: None,
            props: BTreeMap::new(),
            style: BTreeMap::new(),
            text: Some(t.as_ref().to_string())
        }
    }
}

impl AbstractVNode {
    pub fn new(node: BasicVNode) -> AbstractVNode {
        AbstractVNode {
            node: node,
            children: Vec::new()
        }
    }

    pub fn append_child(&mut self, other: AbstractVNode) {
        self.children.push(other);
    }
}

impl<T> InternalVNode<T> where T: dom::Node {
    pub fn from_abstract(root: &AbstractVNode) -> InternalVNode<T> {
        if let Some(ref t) = root.node.text {
            let mut text_node = T::new_text(t.as_str());
            InternalVNode {
                node: root.node.clone(),
                dom_node: text_node,
                children: Vec::new()
            }
        } else {
            let mut dom_node = T::new_element(root.node.tag.as_ref().unwrap().as_str());
            let mut children = Vec::new();

            for (k, v) in root.node.props.iter() {
                dom_node.set_property(k.as_str(), v.as_str());
            }

            for (k, v) in root.node.style.iter() {
                dom_node.set_style(k.as_str(), v.as_str());
            }

            for c in root.children.iter() {
                let child_node: InternalVNode<T> = InternalVNode::from_abstract(c);
                dom_node.append_child(&child_node.dom_node);
                children.push(child_node);
            }

            InternalVNode {
                node: root.node.clone(),
                dom_node: dom_node,
                children: children
            }
        }
    }

    pub fn into_dom_node(self) -> T {
        self.dom_node
    }
}

#[test]
fn test_basic_conversion() {
    use dom::LoggedAction::*;

    let mut root = BasicVNode::new_element("div");
    root.props.insert("id".to_string(), "abc".to_string());
    let mut root = AbstractVNode::new(root);

    let mut p_1 = AbstractVNode::new(BasicVNode::new_element("p"));
    p_1.append_child(AbstractVNode::new(BasicVNode::new_text("Hello world")));

    root.append_child(p_1);
    //let ivn: InternalVNode<dom::DebugNode> = InternalVNode::from_abstract(&root);
    let ivn: InternalVNode<dom::LoggedNode> = InternalVNode::from_abstract(&root);
    
    assert!(ivn.dom_node.text.is_none());
    assert!(ivn.dom_node.tag.as_ref().unwrap().as_str() == "div");

    assert!(ivn.dom_node.actions.len() == 2);
    if let SetProperty(ref k, ref v) = ivn.dom_node.actions[0] {
        assert!(k.as_str() == "id" && v.as_str() == "abc");
    } else {
        panic!();
    }
    if let AppendChild(ref c) = ivn.dom_node.actions[1] {
        assert!(c.tag.as_ref().unwrap().as_str() == "p");
        if let AppendChild(ref c) = c.actions[0] {
            assert!(c.text.as_ref().unwrap().as_str() == "Hello world");
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}
