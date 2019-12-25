use libc::{c_char, c_void};
use std::ops::Deref;
use std::ptr::null;

pub type Opaque = [usize; 0];

#[repr(C)]
pub struct isolate(Opaque);

#[repr(C)]
pub struct blazerod_buf {
  data_ptr: *const u8,
  data_len: usize,
}

unsafe impl Send for blazerod_buf {}

impl blazerod_buf {
  #[inline]
  pub fn empty() -> Self {
    Self {
      data_ptr: null(),
      data_len: 0,
    }
  }

  pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
    Self {
      data_ptr: ptr,
      data_len: len,
    }
  }
}

impl<'a> From<&'a [u8]> for blazerod_buf {
  #[inline]
  fn from(x: &'a [u8]) -> Self {
    Self {
      data_ptr: x.as_ref().as_ptr(),
      data_len: x.len(),
    }
  }
}

impl Deref for blazerod_buf {
  type Target = [u8];
  #[inline]
  fn deref(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.data_ptr, self.data_len) }
  }
}

impl AsRef<[u8]> for blazerod_buf {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    &*self
  }
}

pub type MethodID = u32;

#[allow(non_camel_case_types)]
type blazerod_call_cb = unsafe extern "C" fn(handle: *mut c_void, method_id: MethodID, data: blazerod_buf);

extern "C" {
  pub fn blazerod_init();
  pub fn blazerod_new(cb: blazerod_call_cb) -> *const isolate;
  pub fn blazerod_execute(iso: *const isolate, handle: *const c_void, filename: *const c_char, source: *const c_char);
  pub fn blazerod_respond(iso: *const isolate, handle: *const c_void, method_id: MethodID, buf: blazerod_buf);
  pub fn blazerod_delete(iso: *const isolate);
  pub fn blazerod_v8_version() -> *const c_char;
}

pub fn version() -> &'static str {
  use std::ffi::CStr;
  let version = unsafe { blazerod_v8_version() };
  let c_str = unsafe { CStr::from_ptr(version) };
  c_str.to_str().unwrap()
}
