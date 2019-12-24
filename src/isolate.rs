use crate::v8;
use std::ffi::CString;
use std::sync::Once;

pub struct Isolate {
  isolate: *const v8::isolate,
}

impl Drop for Isolate {
  fn drop(&mut self) {
    unsafe { v8::blazerod_delete(self.isolate) }
  }
}

static V8_INIT: Once = Once::new();

impl Isolate {
  pub fn new() -> Self {
    V8_INIT.call_once(|| {
      unsafe { v8::blazerod_init() };
    });

    let isolate = unsafe { v8::blazerod_new(Self::handle_call) };

    Self { isolate }
  }

  pub fn execute(&mut self, filename: &str, source: &str) {
    let filename = CString::new(filename).unwrap();
    let source = CString::new(source).unwrap();

    unsafe { v8::blazerod_execute(self.isolate, filename.as_ptr(), source.as_ptr()) }
  }

  extern "C" fn handle_call(call_id: v8::CallId) {
    println!("Got a call: {}", call_id);
  }
}
