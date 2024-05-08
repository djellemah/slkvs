#![allow(dead_code)]

#[allow(unused_imports)]
use std::cell::RefCell;

/// Allow for pre-hashing this, since it will be used in many comparisons
#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq,Hash)]
pub enum Step {
  Key(String),
  Index(usize),
}

impl std::fmt::Display for &Step {
  fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match &self {
      Step::Key(v) => write!(f, "{v}"),
      Step::Index(i) => write!(f, "[{i}]"),
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

  pub fn listpaths(&self) -> Vec<String> {
    self.0
      .keys()
      .map(ToString::to_string)
      .collect()
  }

  fn add_at_path(&mut self, path : SchemaPath, json_obj: serde_json::Value) {
    let _rv = match json_obj {
      serde_json::Value::Null => self.0.insert(path, Leaf::String("null".into())),
      serde_json::Value::Bool(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
      serde_json::Value::Number(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
      serde_json::Value::String(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
      serde_json::Value::Array(ary) => {
        for (i,obj) in ary.iter().enumerate() {
          let mut path = path.0.clone();
          path.push(Step::Index(i));
          self.add_at_path(SchemaPath(path), obj.clone());
        };
        // TODO horrible hack
        None
      }
      serde_json::Value::Object(obj) =>{
        let path = path.0.clone();
        for (key,val) in obj {
          let mut path = path.clone();
          path.push(Step::Key(key));
          self.add_at_path(SchemaPath(path), val);
        };
        // another horrible hack
        None
      }
    };

  }

  pub fn addtree(&mut self, path: String, json: String) {
    let path : SchemaPath = path.into();
    let json = serde_json::to_value(json).expect("not parseable {json}");
    self.add_at_path(path, json)
  }
}

impl<T: std::convert::AsRef<[u8]>> std::fmt::Display for Leaf<T> {
  // add code here
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f,"{}",self)
  }
}

#[test]
fn path_string() {
  use Step::*;
  let path = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into())]);
  assert_eq!(path.to_string(), "uno/due/tre");
}

#[test]
fn listpaths() {
  use Step::*;
  let path = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into())]);
  let mut leaf_paths = LeafPaths::new();
  leaf_paths.0.insert(path, Leaf::String("empty not empty".into()));
  assert_eq!(leaf_paths.listpaths(), vec!["uno/due/tre"]);
}

#[test]
fn addtree_obj() {
  use Step::*;
  let obj = serde_json::json!({
    "top": "this",
    "next": {
      "inner": "some value"
    }
  });

  let base_path = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into())]);
  let mut leaf_paths = LeafPaths::new();
  leaf_paths.add_at_path(base_path, obj);

  let expected_path_one = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into()), Key("next".into()), Key("inner".into())]);
  let expected_path_two  = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into()), Key("top".into())]);
  let expected_value_one = Leaf::String("some value".to_string());
  let expected_value_two = Leaf::String("this".to_string());

  println!("{:?}", leaf_paths.0.keys().collect::<Vec<_>>());
  // assert_eq!(leaf_paths.0.keys().collect::<Vec<_>>()[0], &expected_path_zero);
  assert_eq!(leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one);
  // assert_eq!(leaf_paths.0.keys().collect::<Vec<_>>()[1], &expected_path_one);
  assert_eq!(leaf_paths.0.get(&expected_path_two).unwrap(), &expected_value_two);
}
