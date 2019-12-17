use libc::c_char;

extern "C" {
  fn v8_version() -> *const c_char;
}

fn version() -> &'static str {
  use std::ffi::CStr;
  let version = unsafe { v8_version() };
  let c_str = unsafe { CStr::from_ptr(version) };
  c_str.to_str().unwrap()
}

fn main() {
  println!("Using V8 {}", version())
}
