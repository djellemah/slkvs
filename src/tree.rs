#[allow(unused_imports)]
use std::cell::RefCell;

/// Allow for pre-hashing this, since it will be used in many comparisons
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Step {
    Key(String),
    Index(usize),
}

impl<'a> AsRef<Step> for Step {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl std::fmt::Display for &Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Step::Key(v) => write!(f, "{v}"),
            Step::Index(i) => write!(f, "{i}"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SchemaPath(Vec<Step>);

impl std::fmt::Display for SchemaPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_parts = self
            .0
            .iter()
            .map(|step| step.to_string())
            .collect::<Vec<String>>();

        write!(f, "{}", string_parts.join("/"))
    }
}

impl From<String> for SchemaPath {
    fn from(slash_sep: String) -> Self {
        let steps = slash_sep
            .split('/')
            .map(|path_step| Step::Key(path_step.into()))
            .collect::<Vec<_>>();
        SchemaPath(steps)
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug, Clone, Hash, Copy)]
pub enum Leaf<T>
where
    T: AsRef<[u8]>, // because we want storage
{
    String(T),
    // Leave it as string representation for now
    Number(T),
    Boolean(bool),
    Null,
}

// type PathMap<K,V> = std::collections::HashMap<K, V>;
type PathMap<K, V> = std::collections::BTreeMap<K, V>;
pub struct LeafPaths(pub PathMap<SchemaPath, Leaf<String>>);

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

impl std::error::Error for DingString {}

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

    pub fn add(&mut self, path: String, leaf: String) {
        self.0.insert(path.into(), Leaf::String(leaf));
    }

    fn add_at_path(&mut self, path: SchemaPath, json_obj: serde_json::Value) {
        use serde_json::Value::*;
        let _rv = match json_obj {
            Null => self.0.insert(path, Leaf::String("null".into())),
            Bool(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
            Number(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
            String(v) => self.0.insert(path, Leaf::String(format!("{v}"))),
            Array(ary) => {
                for (i, obj) in ary.iter().enumerate() {
                    let mut path = path.0.clone();
                    path.push(Step::Index(i));
                    self.add_at_path(SchemaPath(path), obj.clone());
                }
                // TODO horrible hack
                None
            }
            Object(obj) => {
                let path = path.0.clone();
                for (key, val) in obj {
                    let mut path = path.clone();
                    path.push(Step::Key(key));
                    self.add_at_path(SchemaPath(path), val);
                }
                // another horrible hack
                None
            }
        };
        // rv.expect(format!("error adding {} at {}", json_obj, path).as_str());
    }

    pub fn addtree(&mut self, path: String, json: String) -> Result<(), DingString> {
        let path: SchemaPath = path.into();
        let json = serde_json::from_str(json.as_str())?;
        self.add_at_path(path, json);
        Ok(())
    }

    pub fn delete(&mut self, path: String) {
        let path: SchemaPath = path.into();
        let _ = self.0.remove(&path);
    }
}

impl<T: std::convert::AsRef<[u8]> + std::fmt::Display> std::fmt::Display for Leaf<T> {
    // add code here
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            Leaf::String(str) => write!(f, "{}", str),
            Leaf::Number(_) => todo!(),
            Leaf::Boolean(_) => todo!(),
            Leaf::Null => todo!(),
        }
    }
}

#[test]
fn path_string() {
    use Step::*;
    let path = SchemaPath(vec![
        Key("uno".into()),
        Key("due".into()),
        Key("tre".into()),
    ]);
    assert_eq!(path.to_string(), "uno/due/tre");
}

#[test]
fn listpaths() {
    use Step::*;
    let path = SchemaPath(vec![
        Key("uno".into()),
        Key("due".into()),
        Key("tre".into()),
    ]);
    let mut leaf_paths = LeafPaths::new();
    leaf_paths
        .0
        .insert(path, Leaf::String("empty not empty".into()));
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

    let base_path = SchemaPath(vec![
        Key("uno".into()),
        Key("due".into()),
        Key("tre".into()),
    ]);
    let mut leaf_paths = LeafPaths::new();
    leaf_paths.add_at_path(base_path, obj);

    let expected_path_one = SchemaPath(vec![
        Key("uno".into()),
        Key("due".into()),
        Key("tre".into()),
        Key("next".into()),
        Key("inner".into()),
    ]);
    let expected_path_two = SchemaPath(vec![
        Key("uno".into()),
        Key("due".into()),
        Key("tre".into()),
        Key("top".into()),
    ]);
    let expected_value_one = Leaf::String("some value".to_string());
    let expected_value_two = Leaf::String("this".to_string());

    assert_eq!(
        leaf_paths.0.get(&expected_path_one).unwrap(),
        &expected_value_one
    );
    assert_eq!(
        leaf_paths.0.get(&expected_path_two).unwrap(),
        &expected_value_two
    );
}

#[test]
fn get() {
    use Step::*;
    let path = SchemaPath(vec![Key("wut".into())]);
    let mut leaf_paths = LeafPaths::new();
    leaf_paths
        .0
        .insert(path, Leaf::String("empty not empty".into()));

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

#[cfg(test)]
mod addtree {
    use super::Step::*;
    use super::*;

    #[test]
    fn addtree_singular() {
        let json = r#""singular""#;

        let mut leaf_paths = LeafPaths::new();
        leaf_paths
            .addtree("uno/due/tre".into(), json.into())
            .unwrap();

        let expected_path_one = SchemaPath(vec![
            Key("uno".into()),
            Key("due".into()),
            Key("tre".into()),
        ]);
        let expected_value_one = Leaf::String("singular".to_string());

        assert_eq!(
            leaf_paths.0.get(&expected_path_one).unwrap(),
            &expected_value_one
        );
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
        leaf_paths
            .addtree("uno/due/tre".into(), json.into())
            .unwrap();

        let expected_path_one = SchemaPath(vec![
            Key("uno".into()),
            Key("due".into()),
            Key("tre".into()),
            Key("next".into()),
            Key("inner".into()),
        ]);
        let expected_path_two = SchemaPath(vec![
            Key("uno".into()),
            Key("due".into()),
            Key("tre".into()),
            Key("top".into()),
        ]);
        let expected_value_one = Leaf::String("some value".to_string());
        let expected_value_two = Leaf::String("this".to_string());

        assert_eq!(
            leaf_paths.0.get(&expected_path_one).unwrap(),
            &expected_value_one
        );
        assert_eq!(
            leaf_paths.0.get(&expected_path_two).unwrap(),
            &expected_value_two
        );
    }

    #[test]
    fn bad() {
        let json = r#""singular": "bad bad json"#;

        let mut leaf_paths = LeafPaths::new();
        let err = leaf_paths
            .addtree("uno/due/tre".into(), json.into())
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Error(\"trailing characters\", line: 1, column: 11)"
        );
    }
}
