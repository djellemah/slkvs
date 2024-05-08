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

// pub struct LeafPaths(pub std::collections::HashMap<SchemaPath, Leaf<String>>);
pub struct LeafPaths(pub std::collections::BTreeMap<SchemaPath, Leaf<String>>);

impl LeafPaths {
  pub fn new() -> Self {
    Self(std::collections::BTreeMap::new())
  }

  pub fn get(&self, path : String) -> Option<String> {
    let path : SchemaPath = path.into();
    match self.0.get(&path) {
      None => Some("ya got an oops".into()),
      Some(v) => Some(format!("{v}"))
    }
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
    // rv.expect(format!("error adding {} at {}", json_obj, path).as_str());
  }

  pub fn addtree(&mut self, path: String, json: String) {
    let path : SchemaPath = path.into();
    let json = serde_json::from_str(json.as_str()).expect("not parseable {json}");
    self.add_at_path(path, json)
  }
}

impl<T: std::convert::AsRef<[u8]> + std::fmt::Display> std::fmt::Display for Leaf<T> {
  // add code here
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match &self {
        Leaf::String(str) => write!(f,"{}",str),
        Leaf::Number(_) => todo!(),
        Leaf::Boolean(_) => todo!(),
        Leaf::Null => todo!(),
    }
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

  assert_eq!(leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one);
  assert_eq!(leaf_paths.0.get(&expected_path_two).unwrap(), &expected_value_two);
}

#[test]
fn addtree() {
  use Step::*;
  let json = r#"{
    "top": "this",
    "next": {
      "inner": "some value"
    }
  }"#;

  let mut leaf_paths = LeafPaths::new();
  leaf_paths.addtree("uno/due/tre".into(), json.into());

  println!("{:?}", leaf_paths.0);

  let expected_path_one = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into()), Key("next".into()), Key("inner".into())]);
  let expected_path_two  = SchemaPath(vec![Key("uno".into()), Key("due".into()), Key("tre".into()), Key("top".into())]);
  let expected_value_one = Leaf::String("some value".to_string());
  let expected_value_two = Leaf::String("this".to_string());

  assert_eq!(leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one);
  assert_eq!(leaf_paths.0.get(&expected_path_two).unwrap(), &expected_value_two);
}

#[test]
fn get() {
  use Step::*;
  let path = SchemaPath(vec![Key("wut".into())]);
  let mut leaf_paths = LeafPaths::new();
  leaf_paths.0.insert(path, Leaf::String("empty not empty".into()));

  assert_eq!(leaf_paths.listpaths(), vec!["wut"]);
  assert_eq!(leaf_paths.get("wut".into()), Some("empty not empty".into()));
  // assert_eq!("not", "here");
}

#[test]
fn hash_leaf() {
  use std::hash::Hash;
  use std::hash::Hasher;

  let mut hshr = std::hash::DefaultHasher::new();
  let leaf_string = Leaf::String("hello");
  let leaf_number = Leaf::Number("5");
  leaf_string.hash(&mut hshr);
  leaf_number.hash(&mut hshr);
  assert_eq!(hshr.finish(), 3319468350668666690);
}

#[test]
fn hash_step() {
  use std::hash::Hash;
  use std::hash::Hasher;

  let mut hshr = std::hash::DefaultHasher::new();
  let step_string = Step::Key("hello".into());
  let step_number = Step::Index(5);
  step_string.hash(&mut hshr);
  step_number.hash(&mut hshr);
  assert_eq!(hshr.finish(), 18188393044637332126);
}

#[test]
fn hash_path() {
  use std::hash::Hash;
  use std::hash::Hasher;

  let mut hshr = std::hash::DefaultHasher::new();
  let path = SchemaPath(vec![Step::Key("wut".into()), Step::Index(5)]);
  path.hash(&mut hshr);
  assert_eq!(hshr.finish(), 5364082836139773743);
}
