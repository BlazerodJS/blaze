use std::collections::HashMap;
// use std::pin::Pin;
use crate::v8::MethodID;
use std::sync::{Arc, RwLock};

pub type Buf = Box<[u8]>;

pub enum Method {
  Sync(Buf),
}

pub type MethodHandler = dyn Fn(&[u8]) -> Method + Send + Sync + 'static;

#[derive(Default)]
pub struct MethodRegistry {
  handlers: RwLock<Vec<Arc<Box<MethodHandler>>>>,
  name_to_id: RwLock<HashMap<String, MethodID>>,
}

impl MethodRegistry {
  pub fn new() -> Self {
    let registry = Self::default();
    let method_id = registry.register("ops", |_| unreachable!());
    assert!(method_id == 0);
    registry
  }

  pub fn register<F>(&self, name: &str, handler: F) -> MethodID
  where
    F: Fn(&[u8]) -> Method + Send + Sync + 'static,
  {
    let mut lock = self.handlers.write().unwrap();
    let method_id = lock.len() as u32;

    let mut name_lock = self.name_to_id.write().unwrap();
    let existing = name_lock.insert(name.to_string(), method_id);
    assert!(existing.is_none(), format!("Method already registered: {}", name));

    lock.push(Arc::new(Box::new(handler)));
    drop(name_lock);
    drop(lock);
    method_id
  }

  fn json_map(&self) -> Buf {
    let lock = self.name_to_id.read().unwrap();
    let json_map = serde_json::to_string(&*lock).unwrap();
    json_map.as_bytes().to_owned().into_boxed_slice()
  }

  pub fn call(&self, method_id: MethodID, data: &[u8]) -> Option<Method> {
    if method_id == 0 {
      return Some(Method::Sync(self.json_map()));
    }

    let lock = self.handlers.read().unwrap();
    if let Some(handler) = lock.get(method_id as usize) {
      let handler_ = Arc::clone(&handler);
      drop(lock);
      Some(handler_(data))
    } else {
      None
    }
  }
}
