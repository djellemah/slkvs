#![allow(dead_code)]

#[allow(unused_imports)]
use std::cell::RefCell;

/// Allow for pre-hashing this, since it will be used in many comparisons
#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq,Hash)]
pub enum Step {
  Key(String),
  Index,
}

impl std::fmt::Display for &Step {
  fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match &self {
      Step::Key(v) => write!(f, "{v}"),
      Step::Index => write!(f, "[]"),
    }
  }
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Hash)]
pub struct SchemaPath(Vec<Step>);

impl std::fmt::Display for SchemaPath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string_parts = self.0.iter().map(|step| step.to_string()).collect::<Vec<String>>();
    let repr = string_parts.join("/");

    write!(f,"{repr}")
  }
}

impl From<String> for SchemaPath
{
  fn from(slash_sep: String) -> Self {
    let steps = slash_sep
      .split('/')
      .map(|path_step|{
        Step::Key(path_step.into())
      })
      .collect::<Vec<_>>();
    SchemaPath(steps)
  }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash, Copy)]
pub enum Leaf<T>
where T : AsRef<[u8]> // because we want storage
{
  String(T),
  // Leave it as string representation for now
  Number(T),
  Boolean(bool),
  Null,
}

pub struct LeafPaths(pub std::collections::HashMap<SchemaPath, Leaf<String>>);

impl LeafPaths {
  pub fn new() -> Self {
    Self(std::collections::HashMap::new())
  }
}
