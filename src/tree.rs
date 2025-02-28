/// Steps in a json path. Each step is either a key (for an object) or an index (for an array)
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Step {
  Key(String),
  Index(usize),
}

impl std::fmt::Display for &Step {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    match self {
      Step::Key(v) => f.write_str(v),
      Step::Index(i) => write!(f, "{i}"),
    }
  }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub struct SchemaPath(Vec<Step>);

impl SchemaPath {
  pub fn singleton(step : Step) -> Self {
    Self(vec![step])
  }
}

impl std::fmt::Display for SchemaPath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let string_parts = self.0
      .iter()
      .map(|step| step.to_string())
      .collect::<Vec<_>>();

    f.write_str(&string_parts.join("/"))
  }
}

// convert a path like "root/things/3/name/first"
// into &[Key("root"), Key("things"), Index(3), Key("name"), Key("first")]
fn split_slash_path<S : AsRef<str>>(slash_sep : S) -> Vec<Step> {
  slash_sep
    .as_ref()
    .split('/')
    .map(|path_step| {
      // anything that is not parseable as an integer is treated as a key
      match path_step.parse::<usize>() {
        Ok(i) => Step::Index(i),
        Err(_) => Step::Key(path_step.into()),
      }
    })
    .collect::<Vec<_>>()
}

// E0119 , so can't do mpl<S : AsRef<str>> From<S> for SchemaPath :-(
impl From<&str> for SchemaPath {
  fn from(slash_sep: &str) -> Self {
    Self(split_slash_path(&slash_sep))
  }
}

impl From<String> for SchemaPath {
  fn from(slash_sep: String) -> Self {
    Self(split_slash_path(&slash_sep))
  }
}

impl From<Vec<Step>> for SchemaPath {
  fn from(steps : Vec<Step>) -> Self {
    Self(steps)
  }
}

use std::{collections::HashMap, ops::Add};

impl Add<Step> for SchemaPath
{
  type Output = Self;
  fn add(self, single : Step) -> <Self as Add<SchemaPath>>::Output {
    let mut rv : Vec<Step> = vec![];
    rv.extend_from_slice(self.0.as_slice());
    rv.push(single);
    Self(rv)
  }
}

impl Add for SchemaPath {
  type Output = Self;
  fn add(self, rhs : SchemaPath) -> <Self as Add<SchemaPath>>::Output {
    let mut rv : Vec<Step> = vec![];
    match (&self,&rhs) {
      (Self(ref lhs), Self(ref rhs)) => {
        rv.extend_from_slice(lhs);
        rv.extend_from_slice(rhs);
      },
    };
    Self(rv)
  }
}

impl Add<&SchemaPath> for &SchemaPath {
  type Output = SchemaPath;
  fn add(self, rhs : &SchemaPath) -> <Self as Add<&SchemaPath>>::Output {
    let mut rv : Vec<Step> = vec![];
    match (self,rhs) {
      (SchemaPath(lhs), SchemaPath(rhs)) => {
        rv.extend_from_slice(lhs);
        rv.extend_from_slice(rhs);
      },
    };
    SchemaPath(rv)
  }
}

impl Add<&Vec<Step>> for SchemaPath
{
  type Output = Self;
  fn add(self, rhs : &Vec<Step>) -> <Self as Add<SchemaPath>>::Output {
    let mut rv : Vec<Step> = vec![];
    rv.extend_from_slice(self.0.as_slice());
    rv.extend_from_slice(&rhs);
    Self(rv)
  }
}

impl Add<Step> for &SchemaPath {
  type Output = SchemaPath;
  fn add(self, single : Step) -> <Self as Add<Step>>::Output {
    let mut rv : Vec<Step> = vec![];
    rv.extend_from_slice(self.0.as_slice());
    rv.push(single);
    SchemaPath(rv)
  }
}

// pub trait LeafStorage = AsRef<str>; also works
pub trait LeafStorage = AsRef<[u8]>;

#[derive(Eq, PartialEq, Debug, Clone, Hash, Copy)]
pub enum Leaf<T>
where T : LeafStorage, // because we want storage
{
  String(T),
  // Leave it as string representation for now
  Number(T),
  Boolean(bool),
  Null,
}

// E0119 :-(

// impl<T: LeafStorage> From<i64> for Leaf<T> {
//   fn from(src: i64) -> Self {
//     Self::Number(src)
//   }
// }

// impl<T: std::convert::AsRef<[u8]>> From<bool> for Leaf<T> {
//   fn from(src: bool) -> Self {
//     Self::Boolean(src)
//   }
// }

impl From<&Leaf<String>> for serde_json::Value {
  fn from(leaf: &Leaf<String>) -> Self {
    use serde_json::Value;

    match leaf {
      Leaf::String(v) => Value::String(v.clone()),

      // If this parse fails we should've chosen arbitrary precision at compile
      // time. So not much we can do about that here, short of not using
      // serde_json::Value as the enum. Which is not worthwhile at this point,
      // in this codebase.
      Leaf::Number(v) => Value::Number(v.parse().unwrap()),

      Leaf::Boolean(v) => Value::Bool(*v),
      Leaf::Null => Value::Null,
    }
  }
}

impl<T, Src> From<Src> for Leaf<T>
where
  T: LeafStorage + std::convert::From<Src>,
  Src : std::convert::AsRef<str>,
{
  fn from(src: Src) -> Self {
    Self::String(src.into())
  }
}

// The storage for the keys and the values.
type PathMap<K, V> = std::collections::BTreeMap<K, V>;
pub struct LeafPaths(pub PathMap<SchemaPath,Leaf<String>>);

// 'Ding' cos that's what happens when you get an error.
#[derive(Debug)]
pub struct DingString(String);

impl std::fmt::Display for DingString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    f.write_str(&self.0)
  }
}

impl From<serde_json::Error> for DingString {
  fn from(err: serde_json::Error) -> Self {
    Self(format!("{err:?}"))
  }
}

use std::collections::BTreeMap;

/**
  Need this to collect the results of a traverse_tree

  Needs to be different to serde_json::Value because:

  1) it can be empty (rather than null)

  2) the array must be sparse so that indexes coming across from a
  serde_json::Value can be matched. What happens it that a path into a
  serde_json::Value might skip indexes, so the output is a sparse collection
  of indexes, and the easiest way to model that is a HashMap.
*/
#[derive(Debug,PartialEq,Eq)]
pub enum Collector {
  Empty,

  // Now have to duplicate some of the serde_json::Value members
  Null,
  Bool(bool),
  Number(String),
  String(String),

  // instead of serde_json::Value::Array
  Sparse(BTreeMap<usize,Collector>),

  Object(HashMap<String,Collector>),
}

impl From<&Leaf<String>> for Collector {
  fn from(leaf: &Leaf<String>) -> Self {
    match leaf {
      Leaf::String(v) => Collector::String(v.clone()),

      // If this parse fails we should've chosen arbitrary precision at compile
      // time. So not much we can do about that here, short of not using
      // serde_json::Value as the enum. Which is not worthwhile at this point,
      // in this codebase.
      Leaf::Number(v) => Collector::Number(v.parse().unwrap()),

      Leaf::Boolean(v) => Collector::Bool(*v),
      Leaf::Null => Collector::Null,
    }
  }
}

impl Collector {
  pub fn to_json(&self) -> serde_json::Value {
    use serde_json::Value;
    match self {
      // TODO hmmmmm :-s
      Self::Empty => Value::Null,
      Self::Null => Value::Null,
      Self::Bool(v) => Value::Bool(*v),
      // TODO hmmm. But look, if it's a number then we should be fine here, unless its a "NaN" or "Inf"
      Self::Number(v) => Value::Number(v.parse::<serde_json::Number>().unwrap()),
      Self::String(v) => Value::String(v.to_string()),
      Self::Sparse(v) => {
        if let Some((min_index,_)) = v.first_key_value() {
          // safe to unwrap here because there is at least one item - see min_index above
          let (max_index,_) = v.last_key_value().unwrap();
          let mut values : Vec<serde_json::Value> = Vec::with_capacity(max_index - min_index + 1);

          // order from index least m to greatest n and then assign indexes m-m .. n-m
          for (_i,v) in v.iter() {
            values.push(v.into());
          }
          Value::Array(values)
        } else {
          Value::Array(vec![])
        }
      }
      Self::Object(v) => {
        let mut values : serde_json::Map<String,Value> = serde_json::Map::new();
        for (key,val) in v.iter() {
          values.insert(key.into(),val.into());
        }
        Value::Object(values)
      }
    }
  }
}

impl From<&serde_json::Value> for Collector {
  fn from(val: &serde_json::Value) -> Self {
    match val {
      serde_json::Value::Null => Self::Null,
      serde_json::Value::Bool(v) => Self::Bool(*v),
      // TODO this conversion must be optimised.
      serde_json::Value::Number(v) => Self::Number(v.to_string()),
      serde_json::Value::String(v) => Self::String(v.to_string()),
      serde_json::Value::Array(v) => {
        let mut values : BTreeMap<usize,Collector> = BTreeMap::new();
        for (i,v) in v.iter().enumerate() {
          values.insert(i,v.into());
        }
        Self::Sparse(values)
      }
      serde_json::Value::Object(v) => {
        let mut values : HashMap<String,Collector> = HashMap::new();
        for (i,v) in v.iter() {
          values.insert(i.to_string(),v.into());
        }
        Self::Object(values)
      }
    }
  }
}

impl Into<serde_json::Value> for &Collector {
  fn into(self) -> serde_json::Value {self.to_json() }
}

impl Into<serde_json::Value> for Collector {
  fn into(self) -> serde_json::Value { (&self).into() }
}

// This provides a thin wrapper around the BTree/Hash map and implements
// function calls coming in from the component. Because it's easier to write
// tests this way.
impl LeafPaths {
  pub fn new() -> Self {
    Self(PathMap::new())
  }

  pub fn get(&self, path: String) -> Option<String> {
    let path: SchemaPath = path.into();
    match self.0.get(&path) {
      None => None,
      Some(v) => Some(format!("{v}")),
    }
  }

  pub fn listpaths(&self) -> Vec<String> {
    self.0.keys().map(ToString::to_string).collect()
  }

  /// Given a path, provide all subpaths with their values.
  fn subtree_paths(&self, path: SchemaPath) -> Vec<(SchemaPath,Leaf<String>)> {
    use std::ops::Bound;

    // find the first matching path
    let mut cursor = self.0.lower_bound(Bound::Included(&path));
    // cache to obviate repeated calculation
    let path_len = path.0.len();
    // result goes here
    let mut filtered_paths : Vec<(SchemaPath,Leaf<String>)> = vec![];

    loop {
      match cursor.next() {
        Some((k,v)) => {
          if let Some (prefix) = k.0.get(0..path_len) {
            if prefix > &path.0 { break };
            filtered_paths.push((k.clone(),v.clone()));
          } else {
            break
          }
        },
        None => break,
      }
    }

    filtered_paths
  }

  fn insert(&mut self, path : SchemaPath, leaf : Leaf<String>) -> Option<Leaf<String>> {
    self.0.insert(path, leaf)
  }

  pub fn add(&mut self, path: String, leaf: String) {
    self.insert(path.into(), Leaf::String(leaf));
  }

  fn add_at_path(&mut self, base_path : SchemaPath, json_obj: serde_json::Value) {
    use serde_json::Value::*;
    // These all return Option<_> with the previous value but we don't care
    match json_obj {
      Null => self.insert(base_path, Leaf::Null.into()),
      Bool(v) => self.insert(base_path, format!("{v}").into()),
      Number(v) => self.insert(base_path, Leaf::Number(format!("{v}"))),
      String(v) => self.insert(base_path, v.into()),
      Array(ary) => {
        for (i, obj) in ary.into_iter().enumerate() {
          self.add_at_path(&base_path + Step::Index(i), obj);
        }
        None
      }
      Object(obj) => {
        for (key, val) in obj {
          self.add_at_path(&base_path + Step::Key(key), val);
        }
        None
      }
    };
  }

  pub fn addtree(&mut self, path: String, json: String) -> Result<(), DingString> {
    let json = serde_json::from_str(json.as_str())?;
    self.add_at_path(path.into(), json);
    Ok(())
  }

  #[allow(dead_code,unused_variables)]
  fn append_value(parent : &serde_json::Value, step : &Step, value : &Leaf<String>) -> serde_json::Value {
    serde_json::Value::Null
  }

  // Given a path and a value, insert keys into json obj until path is empty.
  // Essentially, the path and the rcp must be traversed in parallel, and new
  // values must be inserted into the rcp where necessary. New values are
  // either a new collection (ie array or map), or an individual value.
  //
  // rcp is "recipient", which is kinda like an io, except tree-structured.
  fn traverse_tree<'a,'b>(path: &'a [Step], value: &'a Leaf<String>, rcp : &'b mut Collector) {
    // Essentially, a path step is either a key or an index; and a value is a collection or a naked value.
    match (path, rcp) {
      // last step, therefore we can insert value
      ([Step::Key(k)], Collector::Object(ref mut map)) => {
        map.insert(k.into(),value.into());
      },
      ([Step::Index(i)], Collector::Sparse(ref mut ary)) => {
        // TODO looks like this check makes no sense with a sparse array?
        // if *i != ary.len() { panic!("index {i} unexpected compared to length {}", ary.len()) };
        ary.insert(*i, value.into());
      },

      // not the last step, so construct intermediate and keep going
      ([Step::Key(k), rst @ .. ], Collector::Object(ref mut map)) => {
        if let Some(intermediate) = map.get_mut(k) {
          // we already have an object at this key, so reuse it
          Self::traverse_tree(rst, &value, intermediate);
        } else {
          // Dunno yet what kind of object it's going to be
          let mut intermediate = Collector::Empty;
          Self::traverse_tree(rst, &value, &mut intermediate);
          map.insert(k.into(),intermediate);
        }
      }
      // FIXME i is not necessarily an index into ary. Because the tree path may have skipped lower i values.
      ([Step::Index(i), rst @ ..], Collector::Sparse(ref mut ary)) => {
        if let Some(intermediate) = ary.get_mut(i) {
          // we already have an object at this index, so reuse it
          Self::traverse_tree(rst, &value, intermediate);
        } else {
          // Dunno yet what kind of object it's going to be
          let mut intermediate = Collector::Empty;
          Self::traverse_tree(rst, &value, &mut intermediate);
          ary.insert(*i, intermediate);
        }
      }

      // The cases where rcp is Empty, ie we haven't yet figured out what it is.
      // So assign the correct kind of collection, and try again, which will
      // hit one of the above matches.
      ([Step::Key(_), ..], rcp @ Collector::Empty) => {
        // create a new map and try again
        *rcp = Collector::Object(HashMap::new());
        Self::traverse_tree(path, value, rcp)
      }
      ([Step::Index(_), ..], rcp @ Collector::Empty) => {
        // create a new ary and try again
        *rcp = Collector::Sparse(BTreeMap::new());
        Self::traverse_tree(path, value, rcp)
      }
      (path, rcp) => todo!("oopsies with {:?} {:?}", path, rcp),
    };
  }

  /// Fetch an entire subtree, as a string representation of the json rooted at that path.
  pub fn gettree(&self, path: String) -> Collector {
    // fetch all subtree paths with their values
    let path = SchemaPath::from(path);
    let subtree_path_values = self.subtree_paths(path);

    // ok build the object
    let mut obj = Collector::Empty;
    for (schema_path,value) in subtree_path_values {
      LeafPaths::traverse_tree(&schema_path.0, &value, &mut obj);
    }
    obj
  }

  pub fn delete(&mut self, path: String) {
    let path: SchemaPath = path.into();
    let _ = self.0.remove(&path);
  }
}

impl<T> std::fmt::Display for Leaf<T>
where
  T: LeafStorage + std::fmt::Display
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    use Leaf::*;
    match &self {
      String(v) => f.write_str(&v.to_string()),
      Number(v) => write!(f, "{}", v),
      Boolean(v) => write!(f, "{}", v),
      Null => write!(f,"null"),
    }
  }
}

#[cfg(test)]
mod t {
  use super::*;
  #[allow(unused_imports)]
  use pretty_assertions::{assert_eq, assert_ne};

  macro_rules! step_of {
    ($x:ident) => (Step::Key("$x".into()));
    ($x:literal) => (Step::Key($x.into()));
    () => ();
  }

  // This is kinda redundant given split_slash_path
  macro_rules! path_of_strs {
    () => ( vec![] );
    ($($x:expr),+ $(,)?) => (SchemaPath(vec![$(step_of!($x)),*]));
  }


  #[test]
  fn path_string() {
    let path = path_of_strs!["uno", "due", "tre"];
    assert_eq!(path.to_string(), "uno/due/tre");
  }

  #[test]
  fn listpaths() {
    let path = path_of_strs!["uno", "due", "tre"];
    let mut leaf_paths = LeafPaths::new();
    leaf_paths.0.insert(path, Leaf::String("empty not empty".into()));

    assert_eq!(leaf_paths.listpaths(), vec!["uno/due/tre"]);
    assert_eq!(leaf_paths.get("uno/due/tre".into()),Some("empty not empty".into()));
  }

  #[test]
  fn get() {
    let mut leaf_paths = LeafPaths::new();
    leaf_paths.0 .insert(path_of_strs!["wut"], Leaf::String("empty not empty".into()));

    assert_eq!(leaf_paths.listpaths(), vec!["wut"]);
    assert_eq!(leaf_paths.get("wut".into()), Some("empty not empty".into()));
  }

  // was originally to debug what looked like a hashing problem, but was not.
  #[allow(dead_code)]
  // #[test]
  fn hash_leaf() {
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hshr = std::hash::DefaultHasher::new();
    let leaf_string = Leaf::String("hello");
    let leaf_number : Leaf<String> = Leaf::Number("5".into());
    leaf_string.hash(&mut hshr);
    leaf_number.hash(&mut hshr);
    assert_eq!(hshr.finish(), 3319468350668666690);
  }

  // was originally to debug what looked like a hashing problem, but was not.
  #[allow(dead_code)]
  // #[test]
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

  // was originally to debug what looked like a hashing problem, but was not.
  #[allow(dead_code)]
  // #[test]
  fn hash_path() {
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hshr = std::hash::DefaultHasher::new();
    let path = SchemaPath(vec![Step::Key("wut".into()), Step::Index(5)]);
    path.hash(&mut hshr);
    assert_eq!(hshr.finish(), 5364082836139773743);
  }

  #[test]
  fn addtree_obj() {
    let obj = serde_json::json!({
      "top": "this",
      "next": {
        "inner": "some value"
      }
    });

    let base_path = path_of_strs!["uno","due","tre",];
    let mut leaf_paths = LeafPaths::new();
    leaf_paths.add_at_path(base_path, obj);

    let expected_path_one = path_of_strs!["uno", "due", "tre", "next", "inner"];
    let expected_path_two = path_of_strs!["uno", "due", "tre", "top"];
    let expected_value_one = Leaf::String("some value".to_string());
    let expected_value_two = Leaf::String("this".to_string());

    assert_eq!( leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one );
    assert_eq!( leaf_paths.0.get(&expected_path_two).unwrap(), &expected_value_two );
  }

  #[test]
  fn addtree_singular() {
    let json = r#""singular""#;

    let mut leaf_paths = LeafPaths::new();
    leaf_paths
    .addtree("uno/due/tre".into(), json.into())
    .unwrap();

    let expected_path_one = path_of_strs!["uno", "due", "tre"];
    let expected_value_one = Leaf::String("singular".to_string());

    assert_eq!( leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one );
  }

  #[test]
  fn addtree() {
    let json = r#"{
      "top": "this",
      "next": {
        "inner": "some value"
      }
    }"#;

    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("uno/due/tre".into(), json.into()).unwrap();

    let expected_path_one = path_of_strs!["uno", "due", "tre", "next", "inner"];
    let expected_value_one = Leaf::String("some value".to_string());
    let expected_path_two = path_of_strs!["uno", "due", "tre", "top"];
    let expected_value_two = Leaf::String("this".to_string());

    assert_eq!( leaf_paths.0.get(&expected_path_one).unwrap(), &expected_value_one );
    assert_eq!( leaf_paths.0.get(&expected_path_two).unwrap(), &expected_value_two );
  }

  #[test]
  fn bad() {
    let json = r#""singular": "bad bad json"#;

    let mut leaf_paths = LeafPaths::new();
    let err = leaf_paths.addtree("uno/due/tre".into(), json.into()).unwrap_err();

    assert_eq!( err.to_string(), "Error(\"trailing characters\", line: 1, column: 11)" );
  }

  #[test]
  fn basic_subtree_order() {
    let path1 = SchemaPath(split_slash_path("top"));
    let path2 = path_of_strs!["top","next"];
    let path3 = path_of_strs!["top","next","innter"];

    assert!(path1 < path2);
    assert!(path2 < path3);
    assert!(path1 < path3);

    assert!(!(path1 == path2));
    assert!(!(path2 == path3));
    assert!(!(path1 == path3));
  }

  #[allow(unused_mut,unused_variables)]
  #[test]
  fn subtree_paths() {
    let json = r#"{
      "top": "this",
      "next": {
        "inner": "some value"
      }
    }"#;

    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("root".into(), json.into()).unwrap();
    // the converse case - this should be excluded when checking root
    leaf_paths.addtree("another".into(), r#""singular""#.into()).unwrap();

    // small subtree
    let paths = leaf_paths.subtree_paths("root/next".into());
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].0.to_string(), "root/next/inner");

    // bigger subtree
    let paths = leaf_paths.subtree_paths(path_of_strs!["root"]);
    assert_eq!(paths.len(), 2);
    assert_eq!(paths[0].0.to_string(), "root/next/inner");
    assert_eq!(paths[1].0.to_string(), "root/top");

    // and finally, the whole tree
    let paths = leaf_paths.subtree_paths(SchemaPath(vec![]));
    assert_eq!(paths.len(), 3);
    assert_eq!(paths[0].0.to_string(), "another");
    assert_eq!(paths[1].0.to_string(), "root/next/inner");
    assert_eq!(paths[2].0.to_string(), "root/top");
  }

  #[test]
  fn subtree_traverse() {
    let json = serde_json::json!({
      "top": "this",
      "next": {
        "inner": "some value"
      },
      "wut": null,
      "stuff": [9,8,7,6,5]
    });

    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("root".into(), json.to_string()).unwrap();
    let paths = leaf_paths.subtree_paths(path_of_strs!["root"]);
    let mut subtree = Collector::Empty;
    for (schema_path, value) in paths {
      LeafPaths::traverse_tree(&schema_path.0, &value, &mut subtree);
    }

    let json = serde_json::json!({ "root": json });
    assert_eq!(subtree.to_json(),json);
  }

  #[test]
  fn gettree() {
    let json = r#"{
      "top": "this",
      "next": {
        "inner": "some value"
      },
      "wut": null,
      "stuff": [9,8,7,6,5],
      "things": [
        {"name": "one"},
        {"name": "two"},
        {"name": "tre"}
      ]
    }"#;

    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("root".into(), json.into()).unwrap();
    let subtree = leaf_paths.gettree("root".into());
    let json : serde_json::Value = (&subtree).into();
    assert_eq!(json.to_string(), r#"{"root":{"next":{"inner":"some value"},"stuff":[9,8,7,6,5],"things":[{"name":"one"},{"name":"two"},{"name":"tre"}],"top":"this","wut":null}}"#);

    let subtree = leaf_paths.gettree("root/things".into());
    assert_eq!(subtree.to_json(), serde_json::json!({"root":{"things":[{"name":"one"},{"name":"two"},{"name":"tre"}]}}));

    let subtree = leaf_paths.gettree("root/things/1".into());
    assert_eq!(subtree.to_json(), serde_json::json!({"root":{"things":[{"name":"two"}]}}));

    let subtree = leaf_paths.gettree("does/not/exist/5/really".into());
    assert_eq!(subtree, Collector::Empty);
  }

  #[test]
  fn big_gettree() {
    let json = r#"{
      "top": "this",
      "next": [{
        "inner": "some value",
        "tweede": "'n ander waarde",
        "third": "stone from the sun"
      }],
      "wut": null,
      "stuff": [9,8,7,6,5],
      "things": [
        {"name": "one"},
        {"name": "two"},
        {"name": "tre"}
      ]
    }"#;

    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("root".into(), json.into()).unwrap();

    let subtree = leaf_paths.gettree("root/next".into());
    let expected = serde_json::json!({"root":{"next":[{"inner":"some value","third":"stone from the sun","tweede":"'n ander waarde"}]}});
    assert_eq!(subtree.to_json(), expected);

    let subtree = leaf_paths.gettree("root/things".into());
    assert_eq!(subtree.to_json().to_string(), r#"{"root":{"things":[{"name":"one"},{"name":"two"},{"name":"tre"}]}}"#);

    let subtree = leaf_paths.gettree("root/things/1".into());
    assert_eq!(subtree.to_json().to_string(), r#"{"root":{"things":[{"name":"two"}]}}"#);

    let subtree = leaf_paths.gettree("does/not/exist/5/really".into());
    assert_eq!(subtree.to_json(), serde_json::Value::Null);
  }

  #[test]
  // This exercises the construction of the sparse array of the Collector
  fn sample_gettree() {
    let sample_json_str = include_str!("../sample.json");
    let mut leaf_paths = LeafPaths::new();
    leaf_paths.addtree("root".into(), sample_json_str.into()).unwrap();

    let subtree = leaf_paths.gettree("root/web-app/servlet/2".into());
    let expected = serde_json::json!({
      "root": {
        "web-app": {
          "servlet": [
            {
              "servlet-class": "org.cofax.cds.AdminServlet",
              "servlet-name": "cofaxAdmin",
            }
          ]
        }
      }
    });
    assert_eq!(subtree.to_json(), expected);
  }
}
