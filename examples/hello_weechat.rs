#![feature(const_str_as_bytes)]

extern crate weechat_rs;

use std::os::raw::*;
use weechat_rs::ffi::{t_gui_buffer, t_weechat_plugin, WEECHAT_RC_OK};

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_name: &[u8] = "pluggy".as_bytes();

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_api_version: &[u8] = weechat_rs::ffi::WEECHAT_PLUGIN_API_VERSION;

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_author: &[u8] = "Katharina Fey <kookie@spacekookie.de>".as_bytes();

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_description: &[u8] = "A stupid shitty plugin for stupid shit".as_bytes();

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_version: &[u8] = "1.0".as_bytes();

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin_license: &[u8] = "WTFPL".as_bytes();

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static weechat_plugin: *mut t_weechat_plugin = ::std::ptr::null_mut();

unsafe extern "C" fn plugging_cb(
    ptr: *const c_void,
    data: *mut c_void,
    buffer: *mut t_gui_buffer,
    argc: c_int,
    argv: *mut *mut c_char,
    argv_eol: *mut *mut c_char,
) -> c_int {
    // Bindgen can't really export macros for us so we need to do it ourselves
    // Macro below for documentation and references purposes
    // #define weechat_printf(__buffer, __message, __argz...)                  \
    //     (weechat_plugin->printf_date_tags)(__buffer, 0, NULL, __message,    \
    //                                    ##__argz)

    (*weechat_plugin).printf_date_tags.unwrap()(
        buffer,
        0,
        ::std::ptr::null_mut(),
        "Yo what's up?!".as_ptr() as *const i8,
        ::std::ptr::null_mut(),
    );

    // We say it's fine but it's really not 😭
    WEECHAT_RC_OK as i32
}

#[no_mangle]
extern "C" fn weechat_plugin_init(
    plugin: *mut t_weechat_plugin,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    weechat_plugin = plugin;

    // Next the ... hook...macro... *bursts into tears*
    // #define weechat_hook_command(__command, __description, __args,          \
    //                              __args_desc, __completion, __callback,     \
    //                              __pointer, __data)                         \
    //     (weechat_plugin->hook_command)(weechat_plugin, __command,           \
    //                                    __description, __args, __args_desc,  \
    //                                    __completion, __callback, __pointer, \
    //                                    __data)
    unsafe {
        (*weechat_plugin).hook_command.unwrap()(
            plugin,
            "plugger".as_ptr() as *const i8,
            "Plugs things into other things".as_ptr() as *const i8,
            "object".as_ptr() as *const i8,
            "object: Just some object you like\n".as_ptr() as *const i8,
            ::std::ptr::null_mut(),
            Some(plugging_cb),
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
        );
    }

    WEECHAT_RC_OK as i32
}

#[no_mangle]
extern "C" fn weechat_plugin_end(plugin: *mut t_weechat_plugin) -> c_int {

    WEECHAT_RC_OK as i32
}
