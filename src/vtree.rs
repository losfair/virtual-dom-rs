use std::collections::BTreeMap;
use vnode::{BasicVNode, AbstractVNode};
use select;

#[macro_export]
macro_rules! vdmap {
    ( $($key:expr => $value:expr),* ) => {
        {
            #[allow(unused_mut)]
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

pub fn parse_html<T: AsRef<str>>(html: T) -> AbstractVNode {
    let html = html.as_ref().as_bytes();
    let doc = select::document::Document::from_read(html).unwrap();

    let mut root: usize = 0;
    while let Some(parent) = doc.nodes[root].parent {
        root = parent;
    }
    let raw_output = transform_node(&doc, root);
    let body = raw_output.children.into_iter().nth(1).unwrap();
    assert!(body.node.tag.as_ref().unwrap().as_str() == "body");
    body.children.into_iter().nth(0).unwrap()
}

fn transform_node(doc: &select::document::Document, root: usize) -> AbstractVNode {
    let doc_node = &doc.nodes[root];
    match doc_node.data {
        select::node::Data::Text(ref t) => {
            build_text(String::from(t))
        },
        select::node::Data::Comment(_) => build_text(""),
        select::node::Data::Element(ref name, ref props) => {
            let mut target_props = BTreeMap::new();
            for &(ref k, ref v) in props {
                target_props.insert(
                    k.local.to_string(),
                    String::from(v)
                );
            }

            let mut children = Vec::new();
            let mut current_child = doc_node.first_child;
            
            while let Some(id) = current_child {
                children.push(transform_node(doc, id));
                current_child = doc.nodes[id].next;
            }

            build_element(BuildElementOptions {
                tag: name.local.to_string(),
                props: target_props,
                style: vdmap! {},
                children: children
            })
        }
    }
}

#[test]
fn test_parse_html() {
    let output = parse_html(r#"<div id="aaa"><p>Hello world</p></div>"#);
    let expected = build_element(BuildElementOptions {
        tag: "div".to_string(),
        props: vdmap! {
            "id" => "aaa"
        },
        style: vdmap! {},
        children: vec! [
            build_element(BuildElementOptions {
                tag: "p".to_string(),
                props: vdmap! {},
                style: vdmap! {},
                children: vec! [
                    build_text("Hello world")
                ]
            })
        ]
    });
    assert!(output == expected);
}
