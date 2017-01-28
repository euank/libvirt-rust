extern crate libc;

use std::ffi::*;
use std::{ptr, mem, slice};
use virt;
use node::NodeInfo;
use domain::VirDomain;
use storage::VirStoragePool;
use error::VirError;

#[derive(Clone)]
pub struct Connection {
    conn: virt::virConnectPtr,
}

pub enum ConnectionType {
    OPEN,
    READONLY, //TODO: Add Auth Option
}

impl Connection {
    pub fn new(uri: String, conntype: ConnectionType) -> Result<Connection, VirError> {
        unsafe {
            let cUri = CString::new(uri).unwrap();
            let ptr: virt::virConnectPtr = match conntype {
                ConnectionType::OPEN => virt::virConnectOpen(cUri.as_ptr()),
                ConnectionType::READONLY => virt::virConnectOpenReadOnly(cUri.as_ptr()),
            };

            match ptr.is_null() {
                true => Err(VirError::new()),
                false => Ok(Connection { conn: ptr }),
            }
        }
    }

    pub fn capabilities(&self) -> Result<String, VirError> {
        unsafe {
            let cap = virt::virConnectGetCapabilities(self.conn);
            match cap.is_null() {
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(cap).to_bytes()).into_owned()),
                true => Err(VirError::new()),
            }
        }
    }

    pub fn connection_type(&self) -> Result<String, VirError> {
        unsafe {
            let results = virt::virConnectGetType(self.conn);
            match results.is_null() {
                true => Err(VirError::new()),
                false => {
                    Ok(String::from_utf8_lossy(CStr::from_ptr(results).to_bytes()).into_owned())
                }
            }
        }
    }

    pub fn node_info(&self) -> Result<NodeInfo, VirError> {
        unsafe {
            let nptr: virt::virNodeInfoPtr = ptr::null_mut();
            let results = virt::virNodeGetInfo(self.conn, nptr);
            match results == -1 {
                true => Err(VirError::new()),
                false => Ok(NodeInfo { ptr: nptr }),
            }
        }
    }

    pub fn hostname(&self) -> Result<String, VirError> {
        unsafe {
            let name = virt::virConnectGetHostname(self.conn);
            match name.is_null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes()).into_owned()),
            }
        }
    }

    pub fn version(&self) -> Result<u64, VirError> {
        unsafe {
            let size = mem::size_of::<*mut u64>() as libc::size_t;
            let v: *mut u64 = libc::malloc(size) as *mut u64;
            match virt::virConnectGetVersion(self.conn, v) != -1 {
                true => {
                    let ver = v;
                    libc::free(v as *mut libc::c_void);
                    Ok(ver as u64)
                }
                false => {
                    libc::free(v as *mut libc::c_void);
                    Err(VirError::new())
                }
            }
        }
    }

    pub fn libvirt_version(&self) -> Result<u64, VirError> {
        unsafe {
            let size = mem::size_of::<*mut u64>() as libc::size_t;
            let v: *mut u64 = libc::malloc(size) as *mut u64;
            match virt::virConnectGetLibVersion(self.conn, v) != -1 {
                true => {
                    let ver = v;
                    libc::free(v as *mut libc::c_void);
                    Ok(ver as u64)
                }
                false => {
                    libc::free(v as *mut libc::c_void);
                    Err(VirError::new())
                }
            }
        }
    }

    pub fn get_type(&self) -> Result<String, VirError> {
        unsafe {
            let name = virt::virConnectGetType(self.conn);
            match name == ptr::null() {
                true => Err(VirError::new()),
                false => Ok(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes()).into_owned()),
            }
        }
    }

    pub fn alive(&self) -> Result<(), VirError> {
        unsafe {
            match virt::virConnectIsAlive(self.conn) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn encrypted(&self) -> Result<(), VirError> {
        unsafe {
            match virt::virConnectIsEncrypted(self.conn) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn secured(&self) -> Result<(), VirError> {
        unsafe {
            match virt::virConnectIsSecure(self.conn) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }

    pub fn count_defined_domain(&self) -> Result<i32, VirError> {
        unsafe {
            let count = virt::virConnectNumOfDefinedDomains(self.conn);
            match count == -1 {
                true => Err(VirError::new()),
                false => Ok(count as i32),
            }
        }
    }


    pub fn count_domain(&self) -> Result<i32, VirError> {
        unsafe {
            let count = virt::virConnectNumOfDomains(self.conn);
            match count == -1 {
                true => Err(VirError::new()),
                false => Ok(count as i32),
            }
        }
    }

    pub fn domains(&self) -> Result<Vec<i32>, VirError> {
        unsafe {
            let mut ids = ptr::null_mut();

            let num = virt::virConnectListDomains(self.conn, ids, 1024);

            match num != -1 {
                true => {
                    let mut list: Vec<i32> = Vec::new();
                    let n = slice::from_raw_parts(ids, num as usize);
                    for id in n.to_vec() {
                        list.push(id as i32);
                    }
                    libc::free(ids as *mut libc::c_void);
                    Ok(list)
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn list_interfaces(&self) -> Result<Vec<String>, VirError> {
        unsafe {
            let count = virt::virConnectNumOfInterfaces(self.conn);
            let mut names = vec![ptr::null_mut(); count as usize];

            match virt::virConnectListInterfaces(self.conn, names.as_mut_ptr(), count) != -1 {
                true => {
                    let mut list: Vec<String> = Vec::new();
                    for name in names {
                        list.push(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes())
                            .into_owned());
                    }
                    Ok(list)
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn list_all_domains(&self, flags: u32) -> Result<Vec<VirDomain>, VirError> {
        unsafe {
            let mut doms: *mut virt::virDomainPtr = ptr::null_mut();
            let num = virt::virConnectListAllDomains(self.conn, &mut doms, flags);
            match num != -1 {
                true => {
                    let mut list: Vec<VirDomain> = Vec::new();
                    let d = slice::from_raw_parts::<virt::virDomainPtr>(&*doms, num as usize);
                    for dom in d.to_vec() {
                        list.push(VirDomain { ptr: dom });
                    }
                    libc::free(doms as *mut libc::c_void);
                    Ok(list)
                }
                false => {
                    libc::free(doms as *mut libc::c_void);
                    Err(VirError::new())
                }
            }
        }
    }

    pub fn defined_domains(&self) -> Result<Vec<String>, VirError> {
        unsafe {
            let mut names = ptr::null_mut();
            let num = virt::virConnectListDefinedDomains(self.conn, &mut names, 1024);
            match num != -1 {
                true => {
                    let mut list: Vec<String> = Vec::new();
                    let n = slice::from_raw_parts(names, num as usize);
                    for name in n.to_vec() {
                        list.push(String::from_utf8_lossy(CStr::from_ptr(&name).to_bytes())
                            .into_owned());
                    }
                    Ok(list)
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn list_network(&self) -> Result<Vec<String>, VirError> {
        unsafe {
            let count = virt::virConnectNumOfNetworks(self.conn);
            let mut names = vec![ptr::null_mut(); count as usize];

            match virt::virConnectListNetworks(self.conn, names.as_mut_ptr(), count) != -1 {
                true => {
                    let mut list: Vec<String> = Vec::new();
                    for name in names {
                        list.push(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes())
                            .into_owned());
                    }
                    Ok(list)
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn list_storage_pool(&self) -> Result<Vec<String>, VirError> {
        unsafe {
            let count = virt::virConnectNumOfStoragePools(self.conn);
            let mut names = vec![ptr::null_mut(); count as usize];

            match virt::virConnectListStoragePools(self.conn, names.as_mut_ptr(), count) != -1 {
                true => {
                    let mut list: Vec<String> = Vec::new();
                    for name in names {
                        list.push(String::from_utf8_lossy(CStr::from_ptr(name).to_bytes())
                            .into_owned());
                    }
                    Ok(list)
                }
                false => Err(VirError::new()),
            }
        }
    }

    pub fn lookup_domain_byid(&self, id: i32) -> Result<VirDomain, VirError> {
        unsafe {
            let dptr = virt::virDomainLookupByID(self.conn, id);

            match dptr.is_null() {
                false => Ok(VirDomain { ptr: dptr }),
                true => Err(VirError::new()),
            }

        }
    }

    pub fn lookup_domain_byname(&self, name: &str) -> Result<VirDomain, VirError> {
        unsafe {
            let dptr = virt::virDomainLookupByName(self.conn, CString::new(name).unwrap().as_ptr());
            match dptr.is_null() {
                false => Ok(VirDomain { ptr: dptr }),
                true => Err(VirError::new()),
            }
        }
    }

    pub fn create_domain(&self, xml: &str, flags: u32) -> Result<VirDomain, VirError> {
        unsafe {
            let cxml = CString::new(xml).unwrap();
            let pDomain = virt::virDomainCreateXML(self.conn, cxml.as_ptr(), flags);
            match pDomain.is_null() {
                false => Ok(VirDomain { ptr: pDomain }),
                true => Err(VirError::new()),
            }
        }
    }

    pub fn define_domain(&self, xml: &str) -> Result<VirDomain, VirError> {
        unsafe {
            let cxml = CString::new(xml).unwrap();
            let pDomain = virt::virDomainDefineXML(self.conn, cxml.as_ptr());

            match pDomain.is_null() {
                true => Err(VirError::new()),
                false => Ok(VirDomain { ptr: pDomain }),
            }
        }
    }

    pub fn lookup_storage_pool_byname(&self, name: &str) -> Result<VirStoragePool, VirError> {
        unsafe {
            let dptr = virt::virStoragePoolLookupByName(self.conn,
                                                        CString::new(name).unwrap().as_ptr());
            match dptr.is_null() {
                false => Ok(VirStoragePool { ptr: dptr }),
                true => Err(VirError::new()),
            }
        }
    }

    pub fn define_storage_pool(&self, xml: &str) -> Result<VirStoragePool, VirError> {
        unsafe {
            let cxml = CString::new(xml).unwrap();
            let pDomain = virt::virStoragePoolDefineXML(self.conn, cxml.as_ptr(), 0);

            match pDomain.is_null() {
                true => Err(VirError::new()),
                false => Ok(VirStoragePool { ptr: pDomain }),
            }
        }
    }

    pub fn undefine_storage_pool(&self, xml: &str) -> Result<VirStoragePool, VirError> {
        unsafe {
            let cxml = CString::new(xml).unwrap();
            let pDomain = virt::virStoragePoolDefineXML(self.conn, cxml.as_ptr(), 0);

            match pDomain.is_null() {
                true => Err(VirError::new()),
                false => Ok(VirStoragePool { ptr: pDomain }),
            }
        }
    }

    pub fn close(&self) -> Result<(), VirError> {
        unsafe {
            match virt::virConnectClose(self.conn) != -1 {
                true => Ok(()),
                false => Err(VirError::new()),
            }
        }
    }
}
