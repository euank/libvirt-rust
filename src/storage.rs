use std::ffi::*;
use std::{string, ptr, mem};
use virt;
use error::VirError;
use libc::funcs::c95::stdlib;

struct VirStoragePool {
    ptr: virt::virStoragePoolPtr
}

struct VirStoragePoolInfo {
    ptr: virt::virStoragePoolInfo
}

struct VirStorageVol {
    ptr: virt::virStorageVolPtr
}

struct VirStorageVolInfo {
    ptr: virt::virStorageVolInfo
}

impl VirStoragePool {
    pub fn create(self, flags: u32) -> Result<(), VirError> {
        unsafe {
            let result = virt::virStoragePoolCreate(self.ptr, flags);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn build(self, flags: u32) -> Result<(), VirError> {
        unsafe {
            let result = virt::virStoragePoolBuild(self.ptr, flags);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn delete(self, flags: u32) -> Result<(), VirError> {
        unsafe {
            let result = virt::virStoragePoolDelete(self.ptr, flags);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn destroy(self) -> Result<(), VirError> {
        unsafe {
            let result = virt::virStoragePoolDestroy(self.ptr);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn free(self) -> Result<(), VirError> {
        unsafe {
            let result = virt::virStoragePoolFree(self.ptr);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn get_autostart(self) -> Result<bool, VirError> {
        unsafe {
            let out = 0 as *mut i32;
            let result = virt::virStoragePoolGetAutostart(self.ptr, out);
            match result == -1 || out.is_null() {
                true => Err(VirError::new()),
                false => {
                    match *out {
                        1 => Ok(true),
                        _ => Ok(false),
                    }
                }
            }
        }
    }

    pub fn get_info(self) -> Result<VirStoragePoolInfo, VirError> {
        unsafe {
            let info = ptr::null_mut();
            let result = virt::virStoragePoolGetInfo(self.ptr, info);
            match result == -1 {
                true => Err(VirError::new()),
                false => Ok(VirStoragePoolInfo{ptr: *info})
            }
        }
    }

    pub fn get_name(self) -> Result<String, VirError> {
        unsafe {
            let result = virt::virStoragePoolGetName(self.ptr);
            match result.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).into_owned())
            }
        }
    }

    pub fn get_uuid(self) -> Result<String, VirError> {
        let mut array = [0i8; 37];
        let u = &mut array as *mut [i8] as *mut i8;
        unsafe {
            match virt::virStoragePoolGetUUIDString(self.ptr, u) != -1 {
                true => Ok(String::from_utf8_lossy(CStr::from_ptr(u).to_bytes()).into_owned()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn is_active(self) -> Result<(), VirError> {
        unsafe{
            match virt::virStoragePoolIsActive(self.ptr) == 1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn xml_desc(self, flags: u32) -> Result<String, VirError>{
        unsafe {
            let results = virt::virStoragePoolGetXMLDesc(self.ptr, flags);
            match results.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
            }
        }
    }
}
