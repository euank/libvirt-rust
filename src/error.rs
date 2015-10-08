
use std::ffi::*;
use virt;

#[derive(Clone)]
pub struct VirError {
    pub code: i8,
    pub domain: i8,
    pub message: String,
    pub level: i8,
}

impl VirError {
    pub fn new() -> VirError {
        unsafe {
            let ptr: virt::virErrorPtr = virt::virGetLastError();

            let err = VirError {
                code: (*ptr).code as i8,
                domain: (*ptr).domain as i8,
                message: String::from_utf8_lossy(CStr::from_ptr((*ptr).message).to_bytes()).into_owned(),
                level: (*ptr).level as i8,
            };

            virt::virResetError(ptr);
            err
        }
    }
}
