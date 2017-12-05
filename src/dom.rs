use std::rc::Rc;

pub struct DomContext {

}

#[derive(Clone)]
pub struct DomState {
    context: Rc<DomContext>
}

impl DomState {
    pub fn new_root(ctx: Rc<DomContext>) -> DomState {
        DomState {
            context: ctx
        }
    }

    pub fn new_child(&self) -> DomState {
        DomState {
            context: self.context.clone()
        }
    }

    pub fn set_prop<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: V) {

    }

    pub fn remove_prop<K: AsRef<str>>(&self, key: K) {

    }
}

impl DomContext {
    pub fn new() -> DomContext {
        DomContext {

        }
    }

    pub fn rc(self) -> Rc<DomContext> {
        Rc::new(self)
    }
}
