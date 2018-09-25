use std::ffi::CStr;
use std::ops::Index;
use std::os::raw::*;

pub struct Args {
    argc: u32,
    argv: *mut *mut c_char,
}

impl Args {
    pub fn len(&self) -> usize {
        self.argc as usize
    }
}

impl Index<usize> for Args {
    type Output = CStr;

    fn index<'a>(&'a self, index: usize) -> &'a CStr {
        assert!(index < self.len());

        unsafe {
            let ptr = self.argv.offset(index as isize);
            CStr::from_ptr(ptr as *const c_char)
        }
    }
}
