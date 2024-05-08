mod bindings;

use crate::bindings::exports::golem::component::api::*;
use std::cell::RefCell;

mod tree;

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<tree::LeafPaths> = RefCell::new(tree::LeafPaths::new());
}

struct Component;

impl Guest for Component {
    fn add(path : String, leaf: String) {
        STATE.with_borrow_mut(|state| state.0.insert(path.into(),tree::Leaf::String(leaf)));
    }

    fn get(path : String) -> Option<String> {
        STATE.with_borrow(|state| {
            let path : tree::SchemaPath = path.into();
            let value = match state.0.get(&path) {
                None => None,
                Some(v) => Some(format!("{v:?}")),
            };
            value
        })
    }
}
