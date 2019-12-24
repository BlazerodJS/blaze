use libc::c_char;

pub type Opaque = [usize; 0];

#[repr(C)]
pub struct isolate(Opaque);

pub type CallId = u32;

#[allow(non_camel_case_types)]
type blazerod_call_cb = unsafe extern "C" fn(call_id: CallId);

extern "C" {
  pub fn blazerod_init();
  pub fn blazerod_new(cb: blazerod_call_cb) -> *const isolate;
  pub fn blazerod_execute(iso: *const isolate, filename: *const c_char, source: *const c_char);
  pub fn blazerod_delete(iso: *const isolate);
  pub fn blazerod_v8_version() -> *const c_char;
}

pub fn version() -> &'static str {
  use std::ffi::CStr;
  let version = unsafe { blazerod_v8_version() };
  let c_str = unsafe { CStr::from_ptr(version) };
  c_str.to_str().unwrap()
}
