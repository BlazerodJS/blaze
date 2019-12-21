use crate::v8;

pub struct Isolate {
  isolate: *const v8::isolate,
}

impl Drop for Isolate {
  fn drop(&mut self) {
    v8::delete(self.isolate)
  }
}

impl Isolate {
  pub fn new() -> Self {
    let isolate = v8::new_isolate();

    Self { isolate }
  }

  pub fn execute(&mut self, filename: &str, code: &str) -> i32 {
    v8::execute(self.isolate, filename, code)
  }
}
