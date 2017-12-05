use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use node;
use node::Node;
use utils;
use dom;

pub struct ViewState {
    vtree_root: Node,
    dom_ctx: Rc<dom::DomContext>
}

impl ViewState {
    pub fn new(dom_ctx: Rc<dom::DomContext>) -> ViewState {
        ViewState {
            vtree_root: Node::new_root(dom_ctx.clone()),
            dom_ctx: dom_ctx
        }
    }
    pub fn update(&mut self, mut new_root: Node) {
        ViewState::update_dfs(Some(&mut self.vtree_root), &mut new_root);
        self.vtree_root = new_root;
    }

    fn update_dfs(mut old_node: Option<&mut Node>, new_node: &mut Node) {
        // we assume new_node.dom_state has already been initialized at this point
        if let Some(ref old_node) = old_node {
            let dom_state = new_node.dom_state.as_ref().unwrap();
            let prop_insertions = utils::hashmap_insertions(&new_node.props, &old_node.props, true);
            for (k, v) in prop_insertions {
                dom_state.set_prop(k, v);
            }

            let prop_removals = utils::hashmap_insertions(&old_node.props, &new_node.props, false);
            for (k, _) in prop_removals {
                dom_state.remove_prop(k);
            }

            let state_insertions = utils::hashmap_insertions(&new_node.states, &old_node.states, true);
            let state_removals = utils::hashmap_insertions(&old_node.states, &new_node.states, false);
        }

        for i in 0..new_node.children.len() {
            let mut old_child = if let Some(ref mut old_node) = old_node {
                if i < old_node.children.len() {
                    Some(&mut old_node.children[i])
                } else {
                    None
                }
            } else {
                None
            };

            let new_child = &mut new_node.children[i];

            if let Some(ref mut old_child) = old_child {
                new_child.dom_state = ::std::mem::replace(&mut old_child.dom_state, None);
                assert!(new_child.dom_state.is_some());
            } else {
                new_child.dom_state = Some(new_node.dom_state.as_ref().unwrap().new_child());
            }

            ViewState::update_dfs(old_child, new_child);
        }
    }
}
