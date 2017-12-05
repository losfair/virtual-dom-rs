use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;
use std::ops::Deref;
use dom::DomState;
use dom::DomContext;

#[derive(Clone)]
pub struct RcState {
    inner: Rc<State>
}

impl Deref for RcState {
    type Target = Rc<State>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Rc<State>> for RcState {
    fn from(other: Rc<State>) -> RcState {
        RcState {
            inner: other
        }
    }
}

#[derive(Clone)]
pub struct Node {
    context: Option<Rc<NodeContext>>,
    tag: String,
    pub props: HashMap<String, String>,
    pub states: HashMap<String, RcState>,
    pub children: Vec<Node>,
    pub dom_state: Option<DomState>
}

impl Node {
    pub fn new_root(dom_ctx: Rc<DomContext>) -> Node {
        Node {
            context: None,
            tag: "".to_string(),
            props: HashMap::new(),
            states: HashMap::new(),
            children: Vec::new(),
            dom_state: Some(DomState::new_root(dom_ctx))
        }
    }
}

pub trait State {
    fn is_equal(&self, other: &State) -> bool;
}

impl PartialEq for RcState {
    fn eq(&self, other: &RcState) -> bool {
        self.is_equal(&***other)
    }
}

impl Eq for RcState { }

pub trait NodeContext {
    fn render(&self, node: &Node);
}
