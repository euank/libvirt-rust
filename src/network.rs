use std::ffi::*;
use std::string;
use virt;
use error::VirError;


#[derive(Clone)]
pub struct VirNetwork {
    ptr: virt::virNetworkPtr,
}

impl VirNetwork {
    pub fn create(self) -> Result<(), VirError> {
        unsafe {
            match virt::virNetworkCreate(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn free(self) -> Result<(), VirError> {
        unsafe {
            match virt::virNetworkFree(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn destroy(self) -> Result<(), VirError> {
        unsafe {
            match virt::virNetworkDestroy(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn is_active(self) -> Result<(), VirError> {
        unsafe {
            match virt::virNetworkIsActive(self.ptr) == 1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn is_persistent(self) -> Result<(), VirError> {
        unsafe {
            match virt::virNetworkIsPersistent(self.ptr) == 1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn get_autostart(self) -> Result<(), VirError> {
        unsafe {
            let out: *mut i32 = 0 as *mut i32;
            match virt::virNetworkGetAutostart(self.ptr, out) != -1 {
                true => {
                    match *out != -1 {
                        true => Ok(()),
                        false => Err(VirError::new()),
                    }
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn set_autostart(self, val: bool) -> Result<(), VirError> {
        unsafe {
            let out: *mut i32 = match val {
                true => 1 as *mut i32,
                _ => 0 as *mut i32,
            };
            match virt::virNetworkGetAutostart(self.ptr,
                                               match val {
                                                   true => 1 as *mut i32,
                                                   false => 0 as *mut i32,
                                               }) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn name(self) -> Result<String, VirError> {
        unsafe {
            let results = virt::virNetworkGetName(self.ptr);
            match !results.is_null() {
                true => {
                    Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn uuid(self) -> Result<String, VirError> {
        unsafe {
            let mut array = [0i8; 37];
            let u = &mut array as *mut [i8] as *mut i8;
            match virt::virNetworkGetUUIDString(self.ptr, u) != -1 {
                true => Ok(String::from_utf8_lossy(CStr::from_ptr(u).to_bytes()).into_owned()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn get_bridge(self) -> Result<String, VirError> {
        unsafe {
            let results = virt::virNetworkGetBridgeName(self.ptr);
            if !results.is_null() {
                Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
            } else {
                Err(VirError::new())
            }
        }
    }
}
