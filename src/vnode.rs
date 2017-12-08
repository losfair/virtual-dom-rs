use std::collections::BTreeMap;
use vtree;
use dom;
use map_diff;

#[derive(Clone, Eq, PartialEq, Debug)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AbstractVNode {
    pub node: BasicVNode,
    pub children: Vec<AbstractVNode>
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
                dom_node.set_property(k.as_str(), Some(v.as_str()));
            }

            for (k, v) in root.node.style.iter() {
                dom_node.set_style(k.as_str(), Some(v.as_str()));
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

    // The return value indicates whether a new dom_node is
    // created (Some(original_dom_node)) or the original
    // dom_node is updated (None).
    pub fn update(&mut self, root: &AbstractVNode) -> Option<T> {
        if self.node.tag != root.node.tag || self.node.text != root.node.text {
            let orig = ::std::mem::replace(self, Self::from_abstract(root));
            return Some(orig.dom_node);
        }

        // A text node does not have any props, styles or children.
        if self.node.text.is_some() {
            return None;
        }

        {
            let prop_insertions = map_diff::btreemap_insertions(&root.node.props, &self.node.props, true);
            let prop_removals = map_diff::btreemap_insertions(&self.node.props, &root.node.props, false);

            for (k, v) in prop_insertions {
                self.dom_node.set_property(k.as_str(), Some(v.as_str()));
            }
            for (k, v) in prop_removals {
                self.dom_node.set_property(k.as_str(), None);
            }

            let style_insertions = map_diff::btreemap_insertions(&root.node.style, &self.node.style, true);
            let style_removals = map_diff::btreemap_insertions(&self.node.style, &root.node.style, false);

            for (k, v) in style_insertions {
                self.dom_node.set_style(k.as_str(), Some(v.as_str()));
            }
            for (k, v) in style_removals {
                self.dom_node.set_style(k.as_str(), None);
            }
        }
        self.node.props = root.node.props.clone();
        self.node.style = root.node.style.clone();

        while self.children.len() > root.children.len() {
            let last = self.children.pop().unwrap();
            self.dom_node.remove_child(&last.dom_node);
        }

        // self.children.len() <= root.children.len() holds here.

        let mut pos = 0;
        while pos < self.children.len() {
            let orig_dom_node = self.children[pos].update(&root.children[pos]);
            if let Some(orig_dom_node) = orig_dom_node {
                self.dom_node.replace_child(&self.children[pos].dom_node, &orig_dom_node);
            }
            pos += 1;
        }
        while pos < root.children.len() {
            let new_node = Self::from_abstract(&root.children[pos]);
            self.dom_node.append_child(&new_node.dom_node);
            self.children.push(new_node);
            pos += 1;
        }

        None
    }

    pub fn into_dom_node(self) -> T {
        self.dom_node
    }

    pub fn borrow_dom_node(&self) -> &T {
        &self.dom_node
    }
}

#[test]
fn test_basic_conversion() {
    use dom::LoggedAction::*;

    let root = vtree::build_element(vtree::BuildElementOptions {
        tag: "div".to_string(),
        props: vdmap! {
            "id" => "abc"
        },
        style: vdmap! {},
        children: vec! [
            vtree::build_element(vtree::BuildElementOptions {
                tag: "p".to_string(),
                props: vdmap! {},
                style: vdmap! {},
                children: vec! [
                    vtree::build_text("Hello world")
                ]
            })
        ]
    });

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
