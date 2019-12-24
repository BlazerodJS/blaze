use crate::methods::MethodRegistry;
use crate::v8;
use libc::c_void;
use std::ffi::CString;
use std::sync::{Arc, Once};

pub struct Isolate {
  isolate: *const v8::isolate,
  pub method_registry: Arc<MethodRegistry>,
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

    Self {
      isolate,
      method_registry: Arc::new(MethodRegistry::new()),
    }
  }

  pub fn execute(&mut self, filename: &str, source: &str) {
    let filename = CString::new(filename).unwrap();
    let source = CString::new(source).unwrap();

    unsafe { v8::blazerod_execute(self.isolate, self.as_raw_ptr(), filename.as_ptr(), source.as_ptr()) }
  }

  extern "C" fn handle_call(handle_ptr: *mut c_void, method_id: v8::MethodID, data: v8::blazerod_buf) {
    let b = unsafe { Isolate::from_raw_ptr(handle_ptr) };
    let maybe_method = b.method_registry.call(method_id, data.as_ref());

    println!("Got a call: {}", method_id);
    match maybe_method {
      Some(op) => println!("Has a method!"),
      None => println!("No method found"),
    };
  }

  #[inline]
  fn as_raw_ptr(&self) -> *const c_void {
    self as *const _ as *const c_void
  }

  #[inline]
  unsafe fn from_raw_ptr<'a>(ptr: *const c_void) -> &'a mut Self {
    let ptr = ptr as *mut _;
    &mut *ptr
  }
}
