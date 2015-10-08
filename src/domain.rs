extern crate libc;

use std::ffi::*;
use std::{string, ptr, mem};
use node;
use virt;
use error::VirError;
use libc::funcs::c95::stdlib;

#[derive(Clone)]
pub struct VirDomain {
    pub ptr: virt::virDomainPtr
}

#[derive(Clone)]
struct VirDomainBlockInfo {
    ptr: virt::virDomainBlockInfoPtr
}

#[derive(Clone)]
pub struct VirDomainInfo {
    ptr: virt::virDomainInfoPtr
}

#[derive(Clone)]
pub struct VirDomainInterfaceStats {
    pub rx_bytes:   i64,
	pub rx_packets: i64,
	pub rx_errs:    i64,
	pub rx_drop:    i64,
	pub tx_bytes:   i64,
	pub tx_packets: i64,
	pub tx_errs:    i64,
	pub tx_drop:    i64,
}

impl VirDomain {

    pub fn name(self) -> Result<String, VirError> {
        unsafe {
            let name = virt::virDomainGetName(self.ptr);
            match name.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes()).into_owned())
            }
        }
    }

    pub fn free(self) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainFree(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn create(self) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainCreate(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn undefine(self) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainUndefine(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn destroy(self) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainDestroy(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn shutdown(self) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainShutdown(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn reboot(self, flags: u32) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainReboot(self.ptr, flags) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn suspend(self) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainSuspend(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn resume(self) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainResume(self.ptr) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn active(self) -> Result<(), VirError> {
        unsafe{
            match virt::virDomainIsActive(self.ptr) == -1 {
                false => Ok(()),
                true => Err(VirError::new())
            }
        }
    }

    pub fn set_memory(self, mem: u64) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainSetMemory(self.ptr, mem) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn set_memory_flag(self, mem: u64, flags: u32) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainSetMemoryFlags(self.ptr, mem, flags) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn set_max_memory(self, mem: u64) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainSetMaxMemory(self.ptr, mem) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn set_autostart(self, autostart: bool) -> bool {
        unsafe {
            virt::virDomainSetAutostart(self.ptr, match autostart {
                true => 1,
                false => 0
            }) != -1
        }
    }

    pub fn set_vcpu(self, cpu: u32) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainSetVcpus(self.ptr, cpu) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn set_vcpu_flags(self, cpu: u32, flags: u32) -> Result<(), VirError> {
        unsafe {
            match virt::virDomainSetVcpusFlags(self.ptr, cpu, flags) != -1 {
                true => Ok(()),
                false => Err(VirError::new())
            }
        }
    }

    pub fn get_autostart(self) -> bool {
        unsafe {
            let out: *mut i32 = 0 as *mut i32;
            virt::virDomainGetAutostart(self.ptr, out);
            out == 0 as *mut i32
        }
    }

    pub fn get_xmldesc(self, flags: u32) -> Result<String, VirError> {
        unsafe {
            let results = virt::virDomainGetXMLDesc(self.ptr, flags);
            if results == ptr::null_mut() {
                return Err(VirError::new())
            }
            Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
        }
    }

    pub fn interface_stat(self, name: &str) -> Result<VirDomainInterfaceStats, VirError> {
        unsafe {
            let path = CString::new(name).unwrap();
            let size = mem::size_of::<virt::virDomainInterfaceStatsStruct>() as libc::size_t;
            let stats: virt::virDomainInterfaceStatsPtr = stdlib::malloc(size) as virt::virDomainInterfaceStatsPtr;
            let results = virt::virDomainInterfaceStats(self.ptr, path.as_ptr(), stats, size);
            if results != -1 {
                let vs = VirDomainInterfaceStats {
                            rx_bytes:   (*stats).rx_bytes,
                        	rx_packets: (*stats).rx_packets,
                        	rx_errs:    (*stats).rx_errs,
                        	rx_drop:    (*stats).rx_drop,
                        	tx_bytes:   (*stats).tx_bytes,
                        	tx_packets: (*stats).tx_packets,
                        	tx_errs:    (*stats).tx_errs,
                        	tx_drop:    (*stats).tx_drop,
                };
                stdlib::free(stats as *mut libc::types::common::c95::c_void);
                Ok(vs)
            } else {
                stdlib::free(stats as *mut libc::types::common::c95::c_void);
                Err(VirError::new())
            }
        }
    }

    pub fn get_state(self) -> Result<Vec<i32>, VirError> {
        unsafe {
            let ssize = mem::size_of::<*mut i32>() as libc::size_t;
            let rsize = mem::size_of::<*mut i32>() as libc::size_t;

            let state: *mut i32 = stdlib::malloc(ssize) as *mut i32;
            let reason: *mut i32 = stdlib::malloc(rsize) as *mut i32;
            let result = virt::virDomainGetState(self.ptr, state, reason, 0);
            if result == -1 {
                stdlib::free(state as *mut libc::types::common::c95::c_void);
                stdlib::free(reason as *mut libc::types::common::c95::c_void);
                return Err(VirError::new());
            }
            let s = state;
            let r = reason;
            stdlib::free(state as *mut libc::types::common::c95::c_void);
            stdlib::free(reason as *mut libc::types::common::c95::c_void);
            return Ok(vec![*s, *r]);
        }
    }

    pub fn get_id(self) -> Result<u16, VirError> {
        unsafe {
            let id = virt::virDomainGetID(self.ptr);
            match id == 0 {
                true => Err(VirError::new()),
                false => Ok(id as u16)
            }
        }
    }

    pub fn get_info(self) -> Result<VirDomainInfo, VirError> {
        unsafe {
            let size = mem::size_of::<virt::virDomainInfo>() as libc::size_t;
            let info: *mut virt::virDomainInfo = stdlib::malloc(size) as *mut virt::virDomainInfo;

            match virt::virDomainGetInfo(self.ptr, info) != -1 {
                true => {
                    let d = VirDomainInfo{ptr: info};
                    stdlib::free(info as *mut libc::types::common::c95::c_void);
                    Ok(d)
                },
                false => {
                    stdlib::free(info as *mut libc::types::common::c95::c_void);
                    Err(VirError::new())
                }
            }
        }
    }

    pub fn get_uuid(self) -> Result<String, VirError> {
        unsafe {
            let mut array = [0i8; 37];
            let u = &mut array as *mut [i8] as *mut i8;
            match virt::virDomainGetUUIDString(self.ptr, u) == 0 {
                true => {
                    Ok(String::from_utf8_lossy(CStr::from_ptr(u).to_bytes()).into_owned())
                },
                false => {
                    Err(VirError::new())
                }
            }
        }
    }

}

impl VirDomainInfo {
    pub fn state(self) -> i8 {
        unsafe {
            (*self.ptr).state as i8
        }
    }

    pub fn max_mem(self) -> u64 {
        unsafe {
            (*self.ptr).maxMem as u64
        }
    }

    pub fn memory(self) -> u64 {
        unsafe {
            (*self.ptr).memory as u64
        }
    }

    pub fn nr_virt_cpu(self) -> u64 {
        unsafe {
            (*self.ptr).nrVirtCpu as u64
        }
    }

    pub fn cpu_time(self) -> u64 {
        unsafe {
            (*self.ptr).cpuTime as u64
        }
    }

}

//TODO implement VirDomainBlockInfo
