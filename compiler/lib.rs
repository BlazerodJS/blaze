// use fireball::isolate::Isolate;
// use std::sync::{Arc, Mutex};

pub fn ts_version() -> String {
  let data = include_str!("third_party/typescript/package.json");
  let pkg: serde_json::Value = serde_json::from_str(data).unwrap();
  pkg["version"].as_str().unwrap().to_string()
}

// pub struct EmitResult {
//   pub emit_skipped: bool,
//   pub diagnostics: Vec<String>,
//   pub emitted_files: Vec<String>,
// }

// pub struct WrittenFile {
//   pub url: String,
//   pub module_name: String,
//   pub source_code: String,
// }

// pub struct CompilerState {
//   exit_code: i32,
//   emit_result: Option<EmitResult>,
//   written_files: Vec<WrittenFile>,
// }

// pub struct TSIsolate {
//   isolate: Isolate,
//   state: Arc<Mutex<CompilerState>>,
// }

// impl TSIsolate {
//   fn new(bundle: bool) -> Self {
//     let isolate = Isolate::new();

//     let state = Arc::new(Mutex::new(CompilerState {
//       exit_code: 0,
//       emit_result: None,
//       written_files: Vec::new(),
//     }));

//     Self { isolate, state }
//   }
// }

pub fn resolve_asset(name: &str) -> Option<&'static str> {
  macro_rules! tslib {
    ($e:expr) => {
      Some(include_str!(concat!("third_party/typescript/lib/", $e)))
    };
  }

  match name {
    "lib.d.ts" => tslib!("lib.d.ts"),
    "lib.dom.d.ts" => tslib!("lib.dom.d.ts"),
    "lib.dom.iterable.d.ts" => tslib!("lib.dom.iterable.d.ts"),
    "lib.es5.d.ts" => tslib!("lib.es5.d.ts"),
    "lib.es6.d.ts" => tslib!("lib.es6.d.ts"),
    "lib.es2015.collection.d.ts" => tslib!("lib.es2015.collection.d.ts"),
    "lib.es2015.core.d.ts" => tslib!("lib.es2015.core.d.ts"),
    "lib.es2015.d.ts" => tslib!("lib.es2015.d.ts"),
    "lib.es2015.generator.d.ts" => tslib!("lib.es2015.generator.d.ts"),
    "lib.es2015.iterable.d.ts" => tslib!("lib.es2015.iterable.d.ts"),
    "lib.es2015.promise.d.ts" => tslib!("lib.es2015.promise.d.ts"),
    "lib.es2015.proxy.d.ts" => tslib!("lib.es2015.proxy.d.ts"),
    "lib.es2015.reflect.d.ts" => tslib!("lib.es2015.reflect.d.ts"),
    "lib.es2015.symbol.d.ts" => tslib!("lib.es2015.symbol.d.ts"),
    "lib.es2015.symbol.wellknown.d.ts" => tslib!("lib.es2015.symbol.wellknown.d.ts"),
    "lib.es2016.array.include.d.ts" => tslib!("lib.es2016.array.include.d.ts"),
    "lib.es2016.d.ts" => tslib!("lib.es2016.d.ts"),
    "lib.es2016.full.d.ts" => tslib!("lib.es2016.full.d.ts"),
    "lib.es2017.d.ts" => tslib!("lib.es2017.d.ts"),
    "lib.es2017.full.d.ts" => tslib!("lib.es2017.full.d.ts"),
    "lib.es2017.intl.d.ts" => tslib!("lib.es2017.intl.d.ts"),
    "lib.es2017.object.d.ts" => tslib!("lib.es2017.object.d.ts"),
    "lib.es2017.sharedmemory.d.ts" => tslib!("lib.es2017.sharedmemory.d.ts"),
    "lib.es2017.string.d.ts" => tslib!("lib.es2017.string.d.ts"),
    "lib.es2017.typedarrays.d.ts" => tslib!("lib.es2017.typedarrays.d.ts"),
    "lib.es2018.asyncgenerator.d.ts" => tslib!("lib.es2018.asyncgenerator.d.ts"),
    "lib.es2018.asynciterable.d.ts" => tslib!("lib.es2018.asynciterable.d.ts"),
    "lib.es2018.d.ts" => tslib!("lib.es2018.d.ts"),
    "lib.es2018.full.d.ts" => tslib!("lib.es2018.full.d.ts"),
    "lib.es2018.intl.d.ts" => tslib!("lib.es2018.intl.d.ts"),
    "lib.es2018.promise.d.ts" => tslib!("lib.es2018.promise.d.ts"),
    "lib.es2018.regexp.d.ts" => tslib!("lib.es2018.regexp.d.ts"),
    "lib.es2019.array.d.ts" => tslib!("lib.es2019.array.d.ts"),
    "lib.es2019.d.ts" => tslib!("lib.es2019.d.ts"),
    "lib.es2019.full.d.ts" => tslib!("lib.es2019.full.d.ts"),
    "lib.es2019.object.d.ts" => tslib!("lib.es2019.object.d.ts"),
    "lib.es2019.string.d.ts" => tslib!("lib.es2019.string.d.ts"),
    "lib.es2019.symbol.d.ts" => tslib!("lib.es2019.symbol.d.ts"),
    "lib.es2020.d.ts" => tslib!("lib.es2020.d.ts"),
    "lib.es2020.full.d.ts" => tslib!("lib.es2020.full.d.ts"),
    "lib.es2020.string.d.ts" => tslib!("lib.es2020.string.d.ts"),
    "lib.es2020.symbol.wellknown.d.ts" => tslib!("lib.es2020.symbol.wellknown.d.ts"),
    "lib.esnext.array.d.ts" => tslib!("lib.esnext.array.d.ts"),
    "lib.esnext.asynciterable.d.ts" => tslib!("lib.esnext.asynciterable.d.ts"),
    "lib.esnext.bigint.d.ts" => tslib!("lib.esnext.bigint.d.ts"),
    "lib.esnext.d.ts" => tslib!("lib.esnext.d.ts"),
    "lib.esnext.full.d.ts" => tslib!("lib.esnext.full.d.ts"),
    "lib.esnext.intl.d.ts" => tslib!("lib.esnext.intl.d.ts"),
    "lib.esnext.symbol.d.ts" => tslib!("lib.esnext.symbol.d.ts"),
    "lib.scripthost.d.ts" => tslib!("lib.scripthost.d.ts"),
    "lib.webworker.d.ts" => tslib!("lib.webworker.d.ts"),
    "lib.webworker.importscripts.d.ts" => tslib!("lib.webworker.importscripts.d.ts"),
    _ => None,
  }
}
