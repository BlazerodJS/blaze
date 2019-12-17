use libc::c_char;
use libc::c_int;
use std::ffi::CString;

pub type Opaque = [usize; 0];

#[repr(C)]
pub struct CxxIsolate(Opaque);

extern "C" {
  fn v8_init();
  fn v8_version() -> *const c_char;
  fn v8_new() -> *const CxxIsolate;
  fn v8_run(isolate: *const CxxIsolate, filename: *const c_char, source: *const c_char) -> c_int;
}

pub fn init() {
  unsafe {
    v8_init();
  };
}

pub fn new_isolate() -> *const CxxIsolate {
  unsafe { v8_new() }
}

pub fn execute(isolate: *const CxxIsolate, filename: &str, source: &str) -> i32 {
  let filename = CString::new(filename).unwrap();
  let source = CString::new(source).unwrap();

  let ret = unsafe { v8_run(isolate, filename.as_ptr(), source.as_ptr()) };
  ret
}

pub fn version() -> &'static str {
  use std::ffi::CStr;
  let version = unsafe { v8_version() };
  let c_str = unsafe { CStr::from_ptr(version) };
  c_str.to_str().unwrap()
}
