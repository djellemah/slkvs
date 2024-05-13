#![feature(trait_alias)]
#![feature(btree_cursors)]
#![feature(iter_intersperse)]

mod bindings;
mod tree;

use crate::bindings::exports::golem::component::api::*;
use std::cell::RefCell;

use crate::tree::LeafPaths;

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<tree::LeafPaths> = RefCell::new(tree::LeafPaths::new());
}

struct Component;

impl Guest for Component {
    fn add(path: String, leaf: String) {
        STATE.with_borrow_mut(|state| state.add(path, leaf));
    }

    fn get(path: String) -> Option<String> {
        STATE.with_borrow(|state| state.get(path))
    }

    fn listpaths() -> Vec<String> {
        STATE.with_borrow(LeafPaths::listpaths)
    }

    fn addtree(path: String, json: String) -> Result<(), String> {
        let rv = STATE.with_borrow_mut(|db| db.addtree(path, json.clone()));
        rv.map_err(|str| {
            println!("{json:?}");
            str.to_string()
        })
    }

    fn gettree(path: String) -> Result<String,String> {
        STATE.with_borrow(|state| state.gettree(path).map_err(|v| v.to_string()) )
    }

    fn drop() {
        STATE.with_borrow_mut(|db| db.0.clear())
    }

    fn delete(path: std::string::String) {
        STATE.with_borrow_mut(|db| db.delete(path))
    }
}
