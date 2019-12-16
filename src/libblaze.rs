use std::convert::From;
use std::ops::{Deref, DerefMut};
use std::ptr::null;

#[repr(C)]
pub struct isolate {
  _dummy: [u8; 0],
}

#[repr(C)]
pub struct blaze_buffer {
  data_ptr: *const u8,
  data_len: usize,
}

unsafe impl Send for blaze_buffer {}

impl blaze_buffer {
  #[inline]
  pub fn empty() -> Self {
    Self {
      data_ptr: null(),
      data_len: 0,
    }
  }

  #[inline]
  pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
    Self {
      data_ptr: ptr,
      data_len: len,
    }
  }
}

impl<'t> From<&'t [u8]> for blaze_buffer {
  #[inline]
  fn from(x: &'t [u8]) -> Self {
    Self {
      data_ptr: x.as_ref().as_ptr(),
      data_len: x.len(),
    }
  }
}

impl Deref for blaze_buffer {
  type Target = [u8];
  #[inline]
  fn deref(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.data_ptr, self.data_len) }
  }
}

impl AsRef<[u8]> for blaze_buffer {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    &*self
  }
}
