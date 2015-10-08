use std::ffi::*;
use std::{string, ptr};
use virt;
use error::VirError;

#[derive(Clone)]
struct VirInterface {
    ptr: virt::virInterfacePtr
}

impl VirInterface {
    pub fn create(self, flags: u32) -> Result<(), VirError> {
        unsafe {
            match virt::virInterfaceCreate(self.ptr, flags) == -1{
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn destroy(self, flags: u32) -> Result<(), VirError> {
        unsafe {
            match virt::virInterfaceDestroy(self.ptr, flags) == -1{
                true => Err(VirError::new()),
                false => Ok(())
            }
        }
    }

    pub fn is_active(self) -> Result<(), VirError> {
        unsafe {
            match virt::virInterfaceIsActive(self.ptr) == 1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn mac(self) -> Result<String, VirError> {
        unsafe {
            let result = virt::virInterfaceGetMACString(self.ptr);
            match result.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).into_owned())
            }
        }
    }

    pub fn name(self) -> Result<String, VirError> {
        unsafe {
            let result = virt::virInterfaceGetName(self.ptr);
            match result.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).into_owned())
            }
        }
    }

    pub fn xml_desc(self, flags: u32) -> Result<String, VirError>{
        unsafe {
            let results = virt::virInterfaceGetXMLDesc(self.ptr, flags);
            match results.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
            }
        }
    }

    pub fn undefine(self) -> Result<(), VirError>{
        unsafe {
            match virt::virInterfaceUndefine(self.ptr) != -1 {
                false => Err(VirError::new()),
                true => Ok(())
            }
        }
    }

    pub fn free(self) -> Result<(), VirError> {
        unsafe {
            match virt::virInterfaceFree(self.ptr) != -1 {
                false => Err(VirError::new()),
                true => Ok(())
            }
        }
    }

}
