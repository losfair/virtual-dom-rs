use vnode::BasicVNode;

pub trait Node {
    fn new_text(t: &str) -> Self;
    fn new_element(tag: &str) -> Self;
    fn append_child(&mut self, child: &Self);
    fn set_property(&mut self, key: &str, value: &str);
    fn set_style(&mut self, key: &str, value: &str);
}

pub struct DebugNode {
}

impl Node for DebugNode {
    fn new_text(t: &str) -> Self {
        println!("new_text: {}", t);
        DebugNode {
        }
    }

    fn new_element(tag: &str) -> Self {
        println!("new_element: {}", tag);
        DebugNode {
        }
    }

    fn append_child(&mut self, child: &Self) {
        println!("-> append_child");
    }

    fn set_property(&mut self, key: &str, value: &str) {
        println!("-> set_property: {} = {}", key, value);
    }

    fn set_style(&mut self, key: &str, value: &str) {
        println!("-> set_style: {} = {}", key, value);
    }
}

#[derive(Clone)]
pub enum LoggedAction {
    AppendChild(LoggedNode),
    SetProperty(String, String),
    SetStyle(String, String)
}

#[derive(Clone)]
pub struct LoggedNode {
    pub text: Option<String>,
    pub tag: Option<String>,
    pub actions: Vec<LoggedAction>
}

impl Node for LoggedNode {
    fn new_text(t: &str) -> Self {
        LoggedNode {
            text: Some(t.to_string()),
            tag: None,
            actions: Vec::new()
        }
    }

    fn new_element(tag: &str) -> Self {
        LoggedNode {
            text: None,
            tag: Some(tag.to_string()),
            actions: Vec::new()
        }
    }

    fn append_child(&mut self, child: &Self) {
        self.actions.push(LoggedAction::AppendChild(child.clone()));
    }

    fn set_property(&mut self, key: &str, value: &str) {
        self.actions.push(LoggedAction::SetProperty(key.to_string(), value.to_string()));
    }

    fn set_style(&mut self, key: &str, value: &str) {
        self.actions.push(LoggedAction::SetStyle(key.to_string(), value.to_string()));
    }
}
