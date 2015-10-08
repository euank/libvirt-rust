use std::ffi::*;
use std::string;
use virt;

#[derive(Clone)]
pub struct NodeInfo {
    pub ptr: virt::virNodeInfoPtr
}

impl NodeInfo {
    pub fn model(self) -> String {
        unsafe {
            String::from_utf8_lossy(CStr::from_ptr((*self.ptr).model.as_ptr()).to_bytes()).into_owned()
        }
    }

    pub fn memory(self) -> u64 {
        unsafe {
            (*self.ptr).memory
        }
    }

    pub fn cpus(self) -> u32 {
        unsafe {
            (*self.ptr).cpus
        }
    }

    pub fn mhz(self) -> u32 {
        unsafe {
            (*self.ptr).mhz
        }
    }

    pub fn nodes(self) -> u32 {
        unsafe {
            (*self.ptr).mhz
        }
    }

    pub fn sockets(self) -> u32 {
        unsafe {
            (*self.ptr).sockets
        }
    }

    pub fn cores(self) -> u32 {
        unsafe {
            (*self.ptr).cores
        }
    }

    pub fn threads(self) -> u32 {
        unsafe {
            (*self.ptr).threads
        }
    }
}
