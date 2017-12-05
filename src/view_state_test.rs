use view_state;
use dom;
use node;

#[test]
fn test_update() {
    let ctx = dom::DomContext::new().rc();
    let mut vs = view_state::ViewState::new(ctx.clone());

    let mut new_root = node::Node::new_root(ctx.clone());
    let mut children = vec![];
    for i in 0..16 {
        children.push(node::Node::new_root(ctx.clone()));
    }

    for v in children {
        new_root.children.push(v);
    }

    vs.update(new_root.clone());

    new_root.children[0].props.insert("Prop A1".to_string(), "Value A1".to_string());
    new_root.children[0].props.insert("Prop A2".to_string(), "Value A2".to_string());
    new_root.children[7].props.insert("Prop B1".to_string(), "Value B1".to_string());
    vs.update(new_root.clone());

    new_root.children[0].props.remove("Prop A2");
    vs.update(new_root.clone());
}
