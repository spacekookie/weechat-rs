//! A safe plugin shim for weechat
#![allow(non_camel_case_types)]

extern crate libc;

pub mod args;
pub mod ffi;
mod info;

use self::args::*;
use self::ffi::{t_gui_buffer, t_weechat_plugin, WEECHAT_RC_OK};
use std::ffi::CString;
use std::os::raw::*;
use std::ptr;

pub struct WeechatArg {
    txt: String,
}

/// A safe(ish) wrapper around weechat plugin functions
pub struct WeechatPlugin {
    inner: *mut t_weechat_plugin,
}

type WeechatCallback = unsafe extern "C" fn(
    // Plugin pointer
    *const c_void,
    // data pointer
    *mut c_void,
    // buffer where command is executed
    *mut t_gui_buffer,
    // number of arguments given
    c_int,
    // arguments given for command
    *mut *mut c_char,
    // arguments given to EOL (???)
    *mut *mut c_char,
);

impl WeechatPlugin {
    /// Add a new callback function that weechat can access
    pub fn add_callback(
        &mut self,
        name: &str,
        descr: &str,
        args: &str,
        args_descr: &str,
        completion: &str,
        cb: WeechatCallback,
    ) {
    }

    /// Write a message to the weechat log
    pub fn log(&self, message: &str) {
        let log_printf = unsafe { (*self.inner).log_printf.unwrap() };

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(message).unwrap();

        unsafe { log_printf(fmt.as_ptr(), msg.as_ptr()) };
    }

    /// Write a message to the current weechat buffer
    pub fn print(&self, message: &str) {
        let printf_date_tags = unsafe { (*self.inner).printf_date_tags.unwrap() };

        let fmt = CString::new("%s").unwrap();
        let msg = CString::new(message).unwrap();

        unsafe { printf_date_tags(ptr::null_mut(), 0, ptr::null(), fmt.as_ptr(), msg.as_ptr()) };
    }
}
