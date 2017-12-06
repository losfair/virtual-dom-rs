use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use dom::Node;
use vnode::{BasicVNode, AbstractVNode, InternalVNode};

extern "C" {
    fn vdbridge_create_element(tag: *const c_char) -> usize;
    fn vdbridge_create_text_node(text: *const c_char) -> usize;
    fn vdbridge_append_child(parent: usize, child: usize);
    fn vdbridge_set_property(handle: usize, key: *const c_char, value: *const c_char);
    fn vdbridge_set_style(handle: usize, key: *const c_char, value: *const c_char);
    fn vdbridge_release_node(handle: usize);
}

pub struct NativeNode {
    handle: usize
}

impl Node for NativeNode {
    fn new_text(t: &str) -> Self {
        let t = CString::new(t).unwrap();
        NativeNode {
            handle: unsafe { vdbridge_create_text_node(t.as_ptr()) }
        }
    }

    fn new_element(tag: &str) -> Self {
        let tag = CString::new(tag).unwrap();
        NativeNode {
            handle: unsafe { vdbridge_create_element(tag.as_ptr()) }
        }
    }

    fn append_child(&mut self, child: &Self) {
        unsafe { vdbridge_append_child(self.handle, child.handle); }
    }

    fn set_property(&mut self, key: &str, value: &str) {
        let key = CString::new(key).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { vdbridge_set_property(self.handle, key.as_ptr(), value.as_ptr() ); }
    }

    fn set_style(&mut self, key: &str, value: &str) {
        let key = CString::new(key).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { vdbridge_set_style(self.handle, key.as_ptr(), value.as_ptr() ); }
    }
}

impl Drop for NativeNode {
    fn drop(&mut self) {
        unsafe { vdbridge_release_node(self.handle); }
    }
}

#[no_mangle]
pub extern "C" fn vdcore_hello_world() -> usize {
    let mut root = BasicVNode::new_element("div");
    root.props.insert("id".to_string(), "abc".to_string());
    let mut root = AbstractVNode::new(root);

    let mut p_1 = AbstractVNode::new(BasicVNode::new_element("p"));
    p_1.append_child(AbstractVNode::new(BasicVNode::new_text("Hello world")));

    root.append_child(p_1);
    //let ivn: InternalVNode<dom::DebugNode> = InternalVNode::from_abstract(&root);
    let ivn: InternalVNode<NativeNode> = InternalVNode::from_abstract(&root);

    let dom_node = ivn.into_dom_node();
    let handle = dom_node.handle;
    ::std::mem::forget(dom_node);

    handle
}
