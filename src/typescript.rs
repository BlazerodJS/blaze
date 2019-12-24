extern crate serde;
extern crate serde_json;

// use std::path::Path;
// use std::path::PathBuf;

pub fn ts_version() -> String {
  let pkg = include_str!("typescript/package.json");
  let pkg: serde_json::Value = serde_json::from_str(pkg).unwrap();
  pkg["version"].as_str().unwrap().to_string()
}

// pub fn compile_bundle(bundle: &Path, root_names: Vec<PathBuf>) {
//   let config_json = serde_json::json!({});
// }
