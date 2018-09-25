//! A safe plugin shim for weechat
#![allow(non_camel_case_types)]

extern crate libc;

pub mod ffi;
mod info;
pub use self::info::*;

use self::ffi::t_weechat_plugin;

/// A safe(ish) wrapper around weechat plugin functions
pub struct WeechatPlugin {
    inner: *mut t_weechat_plugin,
}

impl WeechatPlugin {
    /// Create a new weechat plugin wrapper from a raw pointer
    pub fn from_ptr(inner: *mut t_weechat_plugin) -> Option<WeechatPlugin> {
        if inner.is_null() {
            None
        } else {
            Some(WeechatPlugin { inner })
        }
    }

    /// Get the inner (raw) representation of a plugin state
    #[inline]
    pub fn get(&self) -> &t_weechat_plugin {
        unsafe { &*self.inner }
    }
}
