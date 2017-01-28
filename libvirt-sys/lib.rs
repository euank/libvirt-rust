// TODO: Do a complete clean up to make everything readable as possible.

extern crate libc;

use self::libc::*;

pub type virFreeCallback = ::std::option::Option<extern "C" fn(opaque: *mut self::libc::c_void)
                                                                 -> ()>;
pub enum Struct__virConnect { }
pub type virConnect = Struct__virConnect;
pub type virConnectPtr = *mut virConnect;
pub enum Struct__virDomain { }
pub type virDomain = Struct__virDomain;
pub type virDomainPtr = *mut virDomain;
pub type Enum_Unnamed1 = self::libc::c_uint;
pub const VIR_DOMAIN_NOSTATE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_RUNNING: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCKED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_PAUSED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_SHUTDOWN: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SHUTOFF: self::libc::c_uint = 5;
pub const VIR_DOMAIN_CRASHED: self::libc::c_uint = 6;
pub const VIR_DOMAIN_PMSUSPENDED: self::libc::c_uint = 7;
pub type virDomainState = Enum_Unnamed1;
pub type Enum_Unnamed2 = self::libc::c_uint;
pub const VIR_DOMAIN_NOSTATE_UNKNOWN: self::libc::c_uint = 0;
pub type virDomainNostateReason = Enum_Unnamed2;
pub type Enum_Unnamed3 = self::libc::c_uint;
pub const VIR_DOMAIN_RUNNING_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_RUNNING_BOOTED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_RUNNING_MIGRATED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_RUNNING_RESTORED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_RUNNING_FROM_SNAPSHOT: self::libc::c_uint = 4;
pub const VIR_DOMAIN_RUNNING_UNPAUSED: self::libc::c_uint = 5;
pub const VIR_DOMAIN_RUNNING_MIGRATION_CANCELED: self::libc::c_uint = 6;
pub const VIR_DOMAIN_RUNNING_SAVE_CANCELED: self::libc::c_uint = 7;
pub const VIR_DOMAIN_RUNNING_WAKEUP: self::libc::c_uint = 8;
pub const VIR_DOMAIN_RUNNING_CRASHED: self::libc::c_uint = 9;
pub type virDomainRunningReason = Enum_Unnamed3;
pub type Enum_Unnamed4 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCKED_UNKNOWN: self::libc::c_uint = 0;
pub type virDomainBlockedReason = Enum_Unnamed4;
pub type Enum_Unnamed5 = self::libc::c_uint;
pub const VIR_DOMAIN_PAUSED_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_PAUSED_USER: self::libc::c_uint = 1;
pub const VIR_DOMAIN_PAUSED_MIGRATION: self::libc::c_uint = 2;
pub const VIR_DOMAIN_PAUSED_SAVE: self::libc::c_uint = 3;
pub const VIR_DOMAIN_PAUSED_DUMP: self::libc::c_uint = 4;
pub const VIR_DOMAIN_PAUSED_IOERROR: self::libc::c_uint = 5;
pub const VIR_DOMAIN_PAUSED_WATCHDOG: self::libc::c_uint = 6;
pub const VIR_DOMAIN_PAUSED_FROM_SNAPSHOT: self::libc::c_uint = 7;
pub const VIR_DOMAIN_PAUSED_SHUTTING_DOWN: self::libc::c_uint = 8;
pub const VIR_DOMAIN_PAUSED_SNAPSHOT: self::libc::c_uint = 9;
pub const VIR_DOMAIN_PAUSED_CRASHED: self::libc::c_uint = 10;
pub type virDomainPausedReason = Enum_Unnamed5;
pub type Enum_Unnamed6 = self::libc::c_uint;
pub const VIR_DOMAIN_SHUTDOWN_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_SHUTDOWN_USER: self::libc::c_uint = 1;
pub type virDomainShutdownReason = Enum_Unnamed6;
pub type Enum_Unnamed7 = self::libc::c_uint;
pub const VIR_DOMAIN_SHUTOFF_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_SHUTOFF_SHUTDOWN: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SHUTOFF_DESTROYED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SHUTOFF_CRASHED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_SHUTOFF_MIGRATED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SHUTOFF_SAVED: self::libc::c_uint = 5;
pub const VIR_DOMAIN_SHUTOFF_FAILED: self::libc::c_uint = 6;
pub const VIR_DOMAIN_SHUTOFF_FROM_SNAPSHOT: self::libc::c_uint = 7;
pub type virDomainShutoffReason = Enum_Unnamed7;
pub type Enum_Unnamed8 = self::libc::c_uint;
pub const VIR_DOMAIN_CRASHED_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_CRASHED_PANICKED: self::libc::c_uint = 1;
pub type virDomainCrashedReason = Enum_Unnamed8;
pub type Enum_Unnamed9 = self::libc::c_uint;
pub const VIR_DOMAIN_PMSUSPENDED_UNKNOWN: self::libc::c_uint = 0;
pub type virDomainPMSuspendedReason = Enum_Unnamed9;
pub type Enum_Unnamed10 = self::libc::c_uint;
pub const VIR_DOMAIN_PMSUSPENDED_DISK_UNKNOWN: self::libc::c_uint = 0;
pub type virDomainPMSuspendedDiskReason = Enum_Unnamed10;
pub type Enum_Unnamed11 = self::libc::c_uint;
pub const VIR_DOMAIN_CONTROL_OK: self::libc::c_uint = 0;
pub const VIR_DOMAIN_CONTROL_JOB: self::libc::c_uint = 1;
pub const VIR_DOMAIN_CONTROL_OCCUPIED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_CONTROL_ERROR: self::libc::c_uint = 3;
pub type virDomainControlState = Enum_Unnamed11;
pub type virDomainControlInfo = Struct__virDomainControlInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainControlInfo {
    pub state: self::libc::c_uint,
    pub details: self::libc::c_uint,
    pub stateTime: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virDomainControlInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainControlInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainControlInfoPtr = *mut virDomainControlInfo;
pub type Enum_Unnamed12 = self::libc::c_uint;
pub const VIR_DOMAIN_AFFECT_CURRENT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_AFFECT_LIVE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_AFFECT_CONFIG: self::libc::c_uint = 2;
pub type virDomainModificationImpact = Enum_Unnamed12;
pub type virDomainInfo = Struct__virDomainInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainInfo {
    pub state: self::libc::c_uchar,
    pub maxMem: self::libc::c_ulong,
    pub memory: self::libc::c_ulong,
    pub nrVirtCpu: self::libc::c_ushort,
    pub cpuTime: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virDomainInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainInfoPtr = *mut virDomainInfo;
pub type Enum_Unnamed13 = self::libc::c_uint;
pub const VIR_DOMAIN_NONE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_START_PAUSED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_START_AUTODESTROY: self::libc::c_uint = 2;
pub const VIR_DOMAIN_START_BYPASS_CACHE: self::libc::c_uint = 4;
pub const VIR_DOMAIN_START_FORCE_BOOT: self::libc::c_uint = 8;
pub type virDomainCreateFlags = Enum_Unnamed13;
pub type Enum_Unnamed14 = self::libc::c_uint;
pub const VIR_NODE_SUSPEND_TARGET_MEM: self::libc::c_uint = 0;
pub const VIR_NODE_SUSPEND_TARGET_DISK: self::libc::c_uint = 1;
pub const VIR_NODE_SUSPEND_TARGET_HYBRID: self::libc::c_uint = 2;
pub type virNodeSuspendTarget = Enum_Unnamed14;
pub enum Struct__virStream { }
pub type virStream = Struct__virStream;
pub type virStreamPtr = *mut virStream;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virSecurityLabel {
    pub label: [self::libc::c_char; 4097usize],
    pub enforcing: self::libc::c_int,
}
impl ::std::clone::Clone for Struct__virSecurityLabel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virSecurityLabel {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virSecurityLabel = Struct__virSecurityLabel;
pub type virSecurityLabelPtr = *mut virSecurityLabel;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virSecurityModel {
    pub model: [self::libc::c_char; 257usize],
    pub doi: [self::libc::c_char; 257usize],
}
impl ::std::clone::Clone for Struct__virSecurityModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virSecurityModel {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virSecurityModel = Struct__virSecurityModel;
pub type virSecurityModelPtr = *mut virSecurityModel;
pub type Enum_Unnamed15 = self::libc::c_uint;
pub const VIR_TYPED_PARAM_INT: self::libc::c_uint = 1;
pub const VIR_TYPED_PARAM_UINT: self::libc::c_uint = 2;
pub const VIR_TYPED_PARAM_LLONG: self::libc::c_uint = 3;
pub const VIR_TYPED_PARAM_ULLONG: self::libc::c_uint = 4;
pub const VIR_TYPED_PARAM_DOUBLE: self::libc::c_uint = 5;
pub const VIR_TYPED_PARAM_BOOLEAN: self::libc::c_uint = 6;
pub const VIR_TYPED_PARAM_STRING: self::libc::c_uint = 7;
pub type virTypedParameterType = Enum_Unnamed15;
pub type Enum_Unnamed16 = self::libc::c_uint;
pub const VIR_TYPED_PARAM_STRING_OKAY: self::libc::c_uint = 4;
pub type virTypedParameterFlags = Enum_Unnamed16;
pub type virTypedParameter = Struct__virTypedParameter;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virTypedParameter {
    pub field: [self::libc::c_char; 80usize],
    pub _type: self::libc::c_int,
    pub value: Union_Unnamed17,
}
impl ::std::clone::Clone for Struct__virTypedParameter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virTypedParameter {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed17 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed17 {
    pub unsafe fn i(&mut self) -> *mut self::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ui(&mut self) -> *mut self::libc::c_uint {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn l(&mut self) -> *mut self::libc::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ul(&mut self) -> *mut self::libc::c_ulonglong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn d(&mut self) -> *mut self::libc::c_double {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn b(&mut self) -> *mut self::libc::c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s(&mut self) -> *mut *mut self::libc::c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed17 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Union_Unnamed17 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virTypedParameterPtr = *mut virTypedParameter;
pub type virNodeInfo = Struct__virNodeInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virNodeInfo {
    pub model: [self::libc::c_char; 32usize],
    pub memory: self::libc::c_ulong,
    pub cpus: self::libc::c_uint,
    pub mhz: self::libc::c_uint,
    pub nodes: self::libc::c_uint,
    pub sockets: self::libc::c_uint,
    pub cores: self::libc::c_uint,
    pub threads: self::libc::c_uint,
}
impl ::std::clone::Clone for Struct__virNodeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virNodeInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed18 = self::libc::c_int;
pub const VIR_NODE_CPU_STATS_ALL_CPUS: self::libc::c_int = -1;
pub type virNodeGetCPUStatsAllCPUs = Enum_Unnamed18;
pub type virNodeCPUStats = Struct__virNodeCPUStats;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virNodeCPUStats {
    pub field: [self::libc::c_char; 80usize],
    pub value: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virNodeCPUStats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virNodeCPUStats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed19 = self::libc::c_int;
pub const VIR_NODE_MEMORY_STATS_ALL_CELLS: self::libc::c_int = -1;
pub type virNodeGetMemoryStatsAllCells = Enum_Unnamed19;
pub type virNodeMemoryStats = Struct__virNodeMemoryStats;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virNodeMemoryStats {
    pub field: [self::libc::c_char; 80usize],
    pub value: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virNodeMemoryStats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virNodeMemoryStats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainBlockStatsStruct = Struct__virDomainBlockStats;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainBlockStats {
    pub rd_req: self::libc::c_longlong,
    pub rd_bytes: self::libc::c_longlong,
    pub wr_req: self::libc::c_longlong,
    pub wr_bytes: self::libc::c_longlong,
    pub errs: self::libc::c_longlong,
}
impl ::std::clone::Clone for Struct__virDomainBlockStats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainBlockStats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainBlockStatsPtr = *mut virDomainBlockStatsStruct;
pub type virDomainInterfaceStatsStruct = Struct__virDomainInterfaceStats;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainInterfaceStats {
    pub rx_bytes: self::libc::c_longlong,
    pub rx_packets: self::libc::c_longlong,
    pub rx_errs: self::libc::c_longlong,
    pub rx_drop: self::libc::c_longlong,
    pub tx_bytes: self::libc::c_longlong,
    pub tx_packets: self::libc::c_longlong,
    pub tx_errs: self::libc::c_longlong,
    pub tx_drop: self::libc::c_longlong,
}
impl ::std::clone::Clone for Struct__virDomainInterfaceStats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainInterfaceStats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainInterfaceStatsPtr = *mut virDomainInterfaceStatsStruct;
pub type Enum_Unnamed20 = self::libc::c_uint;
pub const VIR_DOMAIN_MEMORY_STAT_SWAP_IN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_MEMORY_STAT_SWAP_OUT: self::libc::c_uint = 1;
pub const VIR_DOMAIN_MEMORY_STAT_MAJOR_FAULT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_MEMORY_STAT_MINOR_FAULT: self::libc::c_uint = 3;
pub const VIR_DOMAIN_MEMORY_STAT_UNUSED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_MEMORY_STAT_AVAILABLE: self::libc::c_uint = 5;
pub const VIR_DOMAIN_MEMORY_STAT_ACTUAL_BALLOON: self::libc::c_uint = 6;
pub const VIR_DOMAIN_MEMORY_STAT_RSS: self::libc::c_uint = 7;
pub const VIR_DOMAIN_MEMORY_STAT_NR: self::libc::c_uint = 8;
pub type virDomainMemoryStatTags = Enum_Unnamed20;
pub type virDomainMemoryStatStruct = Struct__virDomainMemoryStat;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainMemoryStat {
    pub tag: self::libc::c_int,
    pub val: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virDomainMemoryStat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainMemoryStat {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainMemoryStatPtr = *mut virDomainMemoryStatStruct;
pub type Enum_Unnamed21 = self::libc::c_uint;
pub const VIR_DUMP_CRASH: self::libc::c_uint = 1;
pub const VIR_DUMP_LIVE: self::libc::c_uint = 2;
pub const VIR_DUMP_BYPASS_CACHE: self::libc::c_uint = 4;
pub const VIR_DUMP_RESET: self::libc::c_uint = 8;
pub const VIR_DUMP_MEMORY_ONLY: self::libc::c_uint = 16;
pub type virDomainCoreDumpFlags = Enum_Unnamed21;
pub type Enum_Unnamed22 = self::libc::c_uint;
pub const VIR_MIGRATE_LIVE: self::libc::c_uint = 1;
pub const VIR_MIGRATE_PEER2PEER: self::libc::c_uint = 2;
pub const VIR_MIGRATE_TUNNELLED: self::libc::c_uint = 4;
pub const VIR_MIGRATE_PERSIST_DEST: self::libc::c_uint = 8;
pub const VIR_MIGRATE_UNDEFINE_SOURCE: self::libc::c_uint = 16;
pub const VIR_MIGRATE_PAUSED: self::libc::c_uint = 32;
pub const VIR_MIGRATE_NON_SHARED_DISK: self::libc::c_uint = 64;
pub const VIR_MIGRATE_NON_SHARED_INC: self::libc::c_uint = 128;
pub const VIR_MIGRATE_CHANGE_PROTECTION: self::libc::c_uint = 256;
pub const VIR_MIGRATE_UNSAFE: self::libc::c_uint = 512;
pub const VIR_MIGRATE_OFFLINE: self::libc::c_uint = 1024;
pub const VIR_MIGRATE_COMPRESSED: self::libc::c_uint = 2048;
pub const VIR_MIGRATE_ABORT_ON_ERROR: self::libc::c_uint = 4096;
pub type virDomainMigrateFlags = Enum_Unnamed22;
pub type virNodeInfoPtr = *mut virNodeInfo;
pub type virNodeCPUStatsPtr = *mut virNodeCPUStats;
pub type virNodeMemoryStatsPtr = *mut virNodeMemoryStats;
pub type Enum_Unnamed23 = self::libc::c_uint;
pub const VIR_CONNECT_RO: self::libc::c_uint = 1;
pub const VIR_CONNECT_NO_ALIASES: self::libc::c_uint = 2;
pub type virConnectFlags = Enum_Unnamed23;
pub type Enum_Unnamed24 = self::libc::c_uint;
pub const VIR_CRED_USERNAME: self::libc::c_uint = 1;
pub const VIR_CRED_AUTHNAME: self::libc::c_uint = 2;
pub const VIR_CRED_LANGUAGE: self::libc::c_uint = 3;
pub const VIR_CRED_CNONCE: self::libc::c_uint = 4;
pub const VIR_CRED_PASSPHRASE: self::libc::c_uint = 5;
pub const VIR_CRED_ECHOPROMPT: self::libc::c_uint = 6;
pub const VIR_CRED_NOECHOPROMPT: self::libc::c_uint = 7;
pub const VIR_CRED_REALM: self::libc::c_uint = 8;
pub const VIR_CRED_EXTERNAL: self::libc::c_uint = 9;
pub type virConnectCredentialType = Enum_Unnamed24;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virConnectCredential {
    pub _type: self::libc::c_int,
    pub prompt: *const self::libc::c_char,
    pub challenge: *const self::libc::c_char,
    pub defresult: *const self::libc::c_char,
    pub result: *mut self::libc::c_char,
    pub resultlen: self::libc::c_uint,
}
impl ::std::clone::Clone for Struct__virConnectCredential {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virConnectCredential {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virConnectCredential = Struct__virConnectCredential;
pub type virConnectCredentialPtr = *mut virConnectCredential;
pub type virConnectAuthCallbackPtr =
    ::std::option::Option<extern "C" fn(cred: virConnectCredentialPtr,
                                          ncred: self::libc::c_uint,
                                          cbdata: *mut self::libc::c_void)
                                          -> self::libc::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virConnectAuth {
    pub credtype: *mut self::libc::c_int,
    pub ncredtype: self::libc::c_uint,
    pub cb: virConnectAuthCallbackPtr,
    pub cbdata: *mut self::libc::c_void,
}
impl ::std::clone::Clone for Struct__virConnectAuth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virConnectAuth {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virConnectAuth = Struct__virConnectAuth;
pub type virConnectAuthPtr = *mut virConnectAuth;
pub type Enum_Unnamed25 = self::libc::c_uint;
pub const VIR_CONNECT_CLOSE_REASON_ERROR: self::libc::c_uint = 0;
pub const VIR_CONNECT_CLOSE_REASON_EOF: self::libc::c_uint = 1;
pub const VIR_CONNECT_CLOSE_REASON_KEEPALIVE: self::libc::c_uint = 2;
pub const VIR_CONNECT_CLOSE_REASON_CLIENT: self::libc::c_uint = 3;
pub type virConnectCloseReason = Enum_Unnamed25;
pub type virConnectCloseFunc =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed26 = self::libc::c_uint;
pub const VIR_DOMAIN_SHUTDOWN_DEFAULT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_SHUTDOWN_ACPI_POWER_BTN: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SHUTDOWN_GUEST_AGENT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SHUTDOWN_INITCTL: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SHUTDOWN_SIGNAL: self::libc::c_uint = 8;
pub type virDomainShutdownFlagValues = Enum_Unnamed26;
pub type Enum_Unnamed27 = self::libc::c_uint;
pub const VIR_DOMAIN_REBOOT_DEFAULT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_REBOOT_ACPI_POWER_BTN: self::libc::c_uint = 1;
pub const VIR_DOMAIN_REBOOT_GUEST_AGENT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_REBOOT_INITCTL: self::libc::c_uint = 4;
pub const VIR_DOMAIN_REBOOT_SIGNAL: self::libc::c_uint = 8;
pub type virDomainRebootFlagValues = Enum_Unnamed27;
pub type Enum_Unnamed28 = self::libc::c_uint;
pub const VIR_DOMAIN_DESTROY_DEFAULT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_DESTROY_GRACEFUL: self::libc::c_uint = 1;
pub type virDomainDestroyFlagsValues = Enum_Unnamed28;
pub type Enum_Unnamed29 = self::libc::c_uint;
pub const VIR_DOMAIN_SAVE_BYPASS_CACHE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SAVE_RUNNING: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SAVE_PAUSED: self::libc::c_uint = 4;
pub type virDomainSaveRestoreFlags = Enum_Unnamed29;
pub type Enum_Unnamed30 = self::libc::c_uint;
pub const VIR_DOMAIN_MEM_CURRENT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_MEM_LIVE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_MEM_CONFIG: self::libc::c_uint = 2;
pub const VIR_DOMAIN_MEM_MAXIMUM: self::libc::c_uint = 4;
pub type virDomainMemoryModFlags = Enum_Unnamed30;
pub type Enum_Unnamed31 = self::libc::c_uint;
pub const VIR_DOMAIN_NUMATUNE_MEM_STRICT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_NUMATUNE_MEM_PREFERRED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_NUMATUNE_MEM_INTERLEAVE: self::libc::c_uint = 2;
pub type virDomainNumatuneMemMode = Enum_Unnamed31;
pub type Enum_Unnamed32 = self::libc::c_uint;
pub const VIR_DOMAIN_METADATA_DESCRIPTION: self::libc::c_uint = 0;
pub const VIR_DOMAIN_METADATA_TITLE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_METADATA_ELEMENT: self::libc::c_uint = 2;
pub type virDomainMetadataType = Enum_Unnamed32;
pub type Enum_Unnamed33 = self::libc::c_uint;
pub const VIR_DOMAIN_XML_SECURE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_XML_INACTIVE: self::libc::c_uint = 2;
pub const VIR_DOMAIN_XML_UPDATE_CPU: self::libc::c_uint = 4;
pub const VIR_DOMAIN_XML_MIGRATABLE: self::libc::c_uint = 8;
pub type virDomainXMLFlags = Enum_Unnamed33;
pub type Enum_Unnamed34 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_RESIZE_BYTES: self::libc::c_uint = 1;
pub type virDomainBlockResizeFlags = Enum_Unnamed34;
pub type virDomainBlockInfo = Struct__virDomainBlockInfo;
pub type virDomainBlockInfoPtr = *mut virDomainBlockInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainBlockInfo {
    pub capacity: self::libc::c_ulonglong,
    pub allocation: self::libc::c_ulonglong,
    pub physical: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virDomainBlockInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainBlockInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed35 = self::libc::c_uint;
pub const VIR_MEMORY_VIRTUAL: self::libc::c_uint = 1;
pub const VIR_MEMORY_PHYSICAL: self::libc::c_uint = 2;
pub type virDomainMemoryFlags = Enum_Unnamed35;
pub type Enum_Unnamed36 = self::libc::c_uint;
pub const VIR_DOMAIN_UNDEFINE_MANAGED_SAVE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_UNDEFINE_SNAPSHOTS_METADATA: self::libc::c_uint = 2;
pub type virDomainUndefineFlagsValues = Enum_Unnamed36;
pub type Enum_Unnamed37 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_DOMAINS_ACTIVE: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_DOMAINS_INACTIVE: self::libc::c_uint = 2;
pub const VIR_CONNECT_LIST_DOMAINS_PERSISTENT: self::libc::c_uint = 4;
pub const VIR_CONNECT_LIST_DOMAINS_TRANSIENT: self::libc::c_uint = 8;
pub const VIR_CONNECT_LIST_DOMAINS_RUNNING: self::libc::c_uint = 16;
pub const VIR_CONNECT_LIST_DOMAINS_PAUSED: self::libc::c_uint = 32;
pub const VIR_CONNECT_LIST_DOMAINS_SHUTOFF: self::libc::c_uint = 64;
pub const VIR_CONNECT_LIST_DOMAINS_OTHER: self::libc::c_uint = 128;
pub const VIR_CONNECT_LIST_DOMAINS_MANAGEDSAVE: self::libc::c_uint = 256;
pub const VIR_CONNECT_LIST_DOMAINS_NO_MANAGEDSAVE: self::libc::c_uint = 512;
pub const VIR_CONNECT_LIST_DOMAINS_AUTOSTART: self::libc::c_uint = 1024;
pub const VIR_CONNECT_LIST_DOMAINS_NO_AUTOSTART: self::libc::c_uint = 2048;
pub const VIR_CONNECT_LIST_DOMAINS_HAS_SNAPSHOT: self::libc::c_uint = 4096;
pub const VIR_CONNECT_LIST_DOMAINS_NO_SNAPSHOT: self::libc::c_uint = 8192;
pub type virConnectListAllDomainsFlags = Enum_Unnamed37;
pub type Enum_Unnamed38 = self::libc::c_uint;
pub const VIR_VCPU_OFFLINE: self::libc::c_uint = 0;
pub const VIR_VCPU_RUNNING: self::libc::c_uint = 1;
pub const VIR_VCPU_BLOCKED: self::libc::c_uint = 2;
pub type virVcpuState = Enum_Unnamed38;
pub type virVcpuInfo = Struct__virVcpuInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virVcpuInfo {
    pub number: self::libc::c_uint,
    pub state: self::libc::c_int,
    pub cpuTime: self::libc::c_ulonglong,
    pub cpu: self::libc::c_int,
}
impl ::std::clone::Clone for Struct__virVcpuInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virVcpuInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virVcpuInfoPtr = *mut virVcpuInfo;
pub type Enum_Unnamed39 = self::libc::c_uint;
pub const VIR_DOMAIN_VCPU_CURRENT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_VCPU_LIVE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_VCPU_CONFIG: self::libc::c_uint = 2;
pub const VIR_DOMAIN_VCPU_MAXIMUM: self::libc::c_uint = 4;
pub const VIR_DOMAIN_VCPU_GUEST: self::libc::c_uint = 8;
pub type virDomainVcpuFlags = Enum_Unnamed39;
pub type Enum_Unnamed40 = self::libc::c_uint;
pub const VIR_DOMAIN_DEVICE_MODIFY_CURRENT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_DEVICE_MODIFY_LIVE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_DEVICE_MODIFY_CONFIG: self::libc::c_uint = 2;
pub const VIR_DOMAIN_DEVICE_MODIFY_FORCE: self::libc::c_uint = 4;
pub type virDomainDeviceModifyFlags = Enum_Unnamed40;
pub type Enum_Unnamed41 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_JOB_TYPE_UNKNOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_BLOCK_JOB_TYPE_PULL: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCK_JOB_TYPE_COPY: self::libc::c_uint = 2;
pub const VIR_DOMAIN_BLOCK_JOB_TYPE_COMMIT: self::libc::c_uint = 3;
pub type virDomainBlockJobType = Enum_Unnamed41;
pub type Enum_Unnamed42 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_JOB_ABORT_ASYNC: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCK_JOB_ABORT_PIVOT: self::libc::c_uint = 2;
pub type virDomainBlockJobAbortFlags = Enum_Unnamed42;
pub type virDomainBlockJobCursor = self::libc::c_ulonglong;
pub type virDomainBlockJobInfo = Struct__virDomainBlockJobInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainBlockJobInfo {
    pub _type: virDomainBlockJobType,
    pub bandwidth: self::libc::c_ulong,
    pub cur: virDomainBlockJobCursor,
    pub end: virDomainBlockJobCursor,
}
impl ::std::clone::Clone for Struct__virDomainBlockJobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainBlockJobInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainBlockJobInfoPtr = *mut virDomainBlockJobInfo;
pub type Enum_Unnamed43 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_REBASE_SHALLOW: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCK_REBASE_REUSE_EXT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_BLOCK_REBASE_COPY_RAW: self::libc::c_uint = 4;
pub const VIR_DOMAIN_BLOCK_REBASE_COPY: self::libc::c_uint = 8;
pub type virDomainBlockRebaseFlags = Enum_Unnamed43;
pub type Enum_Unnamed44 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_COMMIT_SHALLOW: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCK_COMMIT_DELETE: self::libc::c_uint = 2;
pub type virDomainBlockCommitFlags = Enum_Unnamed44;
pub type Enum_Unnamed45 = self::libc::c_uint;
pub const VIR_DOMAIN_DISK_ERROR_NONE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_DISK_ERROR_UNSPEC: self::libc::c_uint = 1;
pub const VIR_DOMAIN_DISK_ERROR_NO_SPACE: self::libc::c_uint = 2;
pub type virDomainDiskErrorCode = Enum_Unnamed45;
pub type virDomainDiskError = Struct__virDomainDiskError;
pub type virDomainDiskErrorPtr = *mut virDomainDiskError;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainDiskError {
    pub disk: *mut self::libc::c_char,
    pub error: self::libc::c_int,
}
impl ::std::clone::Clone for Struct__virDomainDiskError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainDiskError {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Enum_Unnamed46 = self::libc::c_uint;
pub const VIR_NETWORK_XML_INACTIVE: self::libc::c_uint = 1;
pub type virNetworkXMLFlags = Enum_Unnamed46;
pub enum Struct__virNetwork { }
pub type virNetwork = Struct__virNetwork;
pub type virNetworkPtr = *mut virNetwork;
pub type Enum_Unnamed47 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_NETWORKS_INACTIVE: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_NETWORKS_ACTIVE: self::libc::c_uint = 2;
pub const VIR_CONNECT_LIST_NETWORKS_PERSISTENT: self::libc::c_uint = 4;
pub const VIR_CONNECT_LIST_NETWORKS_TRANSIENT: self::libc::c_uint = 8;
pub const VIR_CONNECT_LIST_NETWORKS_AUTOSTART: self::libc::c_uint = 16;
pub const VIR_CONNECT_LIST_NETWORKS_NO_AUTOSTART: self::libc::c_uint = 32;
pub type virConnectListAllNetworksFlags = Enum_Unnamed47;
pub type Enum_Unnamed48 = self::libc::c_uint;
pub const VIR_NETWORK_UPDATE_COMMAND_NONE: self::libc::c_uint = 0;
pub const VIR_NETWORK_UPDATE_COMMAND_MODIFY: self::libc::c_uint = 1;
pub const VIR_NETWORK_UPDATE_COMMAND_DELETE: self::libc::c_uint = 2;
pub const VIR_NETWORK_UPDATE_COMMAND_ADD_LAST: self::libc::c_uint = 3;
pub const VIR_NETWORK_UPDATE_COMMAND_ADD_FIRST: self::libc::c_uint = 4;
pub type virNetworkUpdateCommand = Enum_Unnamed48;
pub type Enum_Unnamed49 = self::libc::c_uint;
pub const VIR_NETWORK_SECTION_NONE: self::libc::c_uint = 0;
pub const VIR_NETWORK_SECTION_BRIDGE: self::libc::c_uint = 1;
pub const VIR_NETWORK_SECTION_DOMAIN: self::libc::c_uint = 2;
pub const VIR_NETWORK_SECTION_IP: self::libc::c_uint = 3;
pub const VIR_NETWORK_SECTION_IP_DHCP_HOST: self::libc::c_uint = 4;
pub const VIR_NETWORK_SECTION_IP_DHCP_RANGE: self::libc::c_uint = 5;
pub const VIR_NETWORK_SECTION_FORWARD: self::libc::c_uint = 6;
pub const VIR_NETWORK_SECTION_FORWARD_INTERFACE: self::libc::c_uint = 7;
pub const VIR_NETWORK_SECTION_FORWARD_PF: self::libc::c_uint = 8;
pub const VIR_NETWORK_SECTION_PORTGROUP: self::libc::c_uint = 9;
pub const VIR_NETWORK_SECTION_DNS_HOST: self::libc::c_uint = 10;
pub const VIR_NETWORK_SECTION_DNS_TXT: self::libc::c_uint = 11;
pub const VIR_NETWORK_SECTION_DNS_SRV: self::libc::c_uint = 12;
pub type virNetworkUpdateSection = Enum_Unnamed49;
pub type Enum_Unnamed50 = self::libc::c_uint;
pub const VIR_NETWORK_UPDATE_AFFECT_CURRENT: self::libc::c_uint = 0;
pub const VIR_NETWORK_UPDATE_AFFECT_LIVE: self::libc::c_uint = 1;
pub const VIR_NETWORK_UPDATE_AFFECT_CONFIG: self::libc::c_uint = 2;
pub type virNetworkUpdateFlags = Enum_Unnamed50;
pub enum Struct__virInterface { }
pub type virInterface = Struct__virInterface;
pub type virInterfacePtr = *mut virInterface;
pub type Enum_Unnamed51 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_INTERFACES_INACTIVE: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_INTERFACES_ACTIVE: self::libc::c_uint = 2;
pub type virConnectListAllInterfacesFlags = Enum_Unnamed51;
pub type Enum_Unnamed52 = self::libc::c_uint;
pub const VIR_INTERFACE_XML_INACTIVE: self::libc::c_uint = 1;
pub type virInterfaceXMLFlags = Enum_Unnamed52;
pub enum Struct__virStoragePool { }
pub type virStoragePool = Struct__virStoragePool;
pub type virStoragePoolPtr = *mut virStoragePool;
pub type Enum_Unnamed53 = self::libc::c_uint;
pub const VIR_STORAGE_POOL_INACTIVE: self::libc::c_uint = 0;
pub const VIR_STORAGE_POOL_BUILDING: self::libc::c_uint = 1;
pub const VIR_STORAGE_POOL_RUNNING: self::libc::c_uint = 2;
pub const VIR_STORAGE_POOL_DEGRADED: self::libc::c_uint = 3;
pub const VIR_STORAGE_POOL_INACCESSIBLE: self::libc::c_uint = 4;
pub type virStoragePoolState = Enum_Unnamed53;
pub type Enum_Unnamed54 = self::libc::c_uint;
pub const VIR_STORAGE_POOL_BUILD_NEW: self::libc::c_uint = 0;
pub const VIR_STORAGE_POOL_BUILD_REPAIR: self::libc::c_uint = 1;
pub const VIR_STORAGE_POOL_BUILD_RESIZE: self::libc::c_uint = 2;
pub const VIR_STORAGE_POOL_BUILD_NO_OVERWRITE: self::libc::c_uint = 4;
pub const VIR_STORAGE_POOL_BUILD_OVERWRITE: self::libc::c_uint = 8;
pub type virStoragePoolBuildFlags = Enum_Unnamed54;
pub type Enum_Unnamed55 = self::libc::c_uint;
pub const VIR_STORAGE_POOL_DELETE_NORMAL: self::libc::c_uint = 0;
pub const VIR_STORAGE_POOL_DELETE_ZEROED: self::libc::c_uint = 1;
pub type virStoragePoolDeleteFlags = Enum_Unnamed55;
pub type virStoragePoolInfo = Struct__virStoragePoolInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virStoragePoolInfo {
    pub state: self::libc::c_int,
    pub capacity: self::libc::c_ulonglong,
    pub allocation: self::libc::c_ulonglong,
    pub available: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virStoragePoolInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virStoragePoolInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virStoragePoolInfoPtr = *mut virStoragePoolInfo;
pub enum Struct__virStorageVol { }
pub type virStorageVol = Struct__virStorageVol;
pub type virStorageVolPtr = *mut virStorageVol;
pub type Enum_Unnamed56 = self::libc::c_uint;
pub const VIR_STORAGE_VOL_FILE: self::libc::c_uint = 0;
pub const VIR_STORAGE_VOL_BLOCK: self::libc::c_uint = 1;
pub const VIR_STORAGE_VOL_DIR: self::libc::c_uint = 2;
pub const VIR_STORAGE_VOL_NETWORK: self::libc::c_uint = 3;
pub const VIR_STORAGE_VOL_NETDIR: self::libc::c_uint = 4;
pub type virStorageVolType = Enum_Unnamed56;
pub type Enum_Unnamed57 = self::libc::c_uint;
pub const VIR_STORAGE_VOL_DELETE_NORMAL: self::libc::c_uint = 0;
pub const VIR_STORAGE_VOL_DELETE_ZEROED: self::libc::c_uint = 1;
pub type virStorageVolDeleteFlags = Enum_Unnamed57;
pub type Enum_Unnamed58 = self::libc::c_uint;
pub const VIR_STORAGE_VOL_WIPE_ALG_ZERO: self::libc::c_uint = 0;
pub const VIR_STORAGE_VOL_WIPE_ALG_NNSA: self::libc::c_uint = 1;
pub const VIR_STORAGE_VOL_WIPE_ALG_DOD: self::libc::c_uint = 2;
pub const VIR_STORAGE_VOL_WIPE_ALG_BSI: self::libc::c_uint = 3;
pub const VIR_STORAGE_VOL_WIPE_ALG_GUTMANN: self::libc::c_uint = 4;
pub const VIR_STORAGE_VOL_WIPE_ALG_SCHNEIER: self::libc::c_uint = 5;
pub const VIR_STORAGE_VOL_WIPE_ALG_PFITZNER7: self::libc::c_uint = 6;
pub const VIR_STORAGE_VOL_WIPE_ALG_PFITZNER33: self::libc::c_uint = 7;
pub const VIR_STORAGE_VOL_WIPE_ALG_RANDOM: self::libc::c_uint = 8;
pub type virStorageVolWipeAlgorithm = Enum_Unnamed58;
pub type virStorageVolInfo = Struct__virStorageVolInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virStorageVolInfo {
    pub _type: self::libc::c_int,
    pub capacity: self::libc::c_ulonglong,
    pub allocation: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virStorageVolInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virStorageVolInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virStorageVolInfoPtr = *mut virStorageVolInfo;
pub type Enum_Unnamed59 = self::libc::c_uint;
pub const VIR_STORAGE_XML_INACTIVE: self::libc::c_uint = 1;
pub type virStorageXMLFlags = Enum_Unnamed59;
pub type Enum_Unnamed60 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_INACTIVE: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_ACTIVE: self::libc::c_uint = 2;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_PERSISTENT: self::libc::c_uint = 4;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_TRANSIENT: self::libc::c_uint = 8;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_AUTOSTART: self::libc::c_uint = 16;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_NO_AUTOSTART: self::libc::c_uint = 32;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_DIR: self::libc::c_uint = 64;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_FS: self::libc::c_uint = 128;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_NETFS: self::libc::c_uint = 256;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_LOGICAL: self::libc::c_uint = 512;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_DISK: self::libc::c_uint = 1024;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_ISCSI: self::libc::c_uint = 2048;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_SCSI: self::libc::c_uint = 4096;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_MPATH: self::libc::c_uint = 8192;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_RBD: self::libc::c_uint = 16384;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_SHEEPDOG: self::libc::c_uint = 32768;
pub const VIR_CONNECT_LIST_STORAGE_POOLS_GLUSTER: self::libc::c_uint = 65536;
pub type virConnectListAllStoragePoolsFlags = Enum_Unnamed60;
pub type Enum_Unnamed61 = self::libc::c_uint;
pub const VIR_STORAGE_VOL_CREATE_PREALLOC_METADATA: self::libc::c_uint = 1;
pub type virStorageVolCreateFlags = Enum_Unnamed61;
pub type Enum_Unnamed62 = self::libc::c_uint;
pub const VIR_STORAGE_VOL_RESIZE_ALLOCATE: self::libc::c_uint = 1;
pub const VIR_STORAGE_VOL_RESIZE_DELTA: self::libc::c_uint = 2;
pub const VIR_STORAGE_VOL_RESIZE_SHRINK: self::libc::c_uint = 4;
pub type virStorageVolResizeFlags = Enum_Unnamed62;
pub type Enum_Unnamed63 = self::libc::c_uint;
pub const VIR_KEYCODE_SET_LINUX: self::libc::c_uint = 0;
pub const VIR_KEYCODE_SET_XT: self::libc::c_uint = 1;
pub const VIR_KEYCODE_SET_ATSET1: self::libc::c_uint = 2;
pub const VIR_KEYCODE_SET_ATSET2: self::libc::c_uint = 3;
pub const VIR_KEYCODE_SET_ATSET3: self::libc::c_uint = 4;
pub const VIR_KEYCODE_SET_OSX: self::libc::c_uint = 5;
pub const VIR_KEYCODE_SET_XT_KBD: self::libc::c_uint = 6;
pub const VIR_KEYCODE_SET_USB: self::libc::c_uint = 7;
pub const VIR_KEYCODE_SET_WIN32: self::libc::c_uint = 8;
pub const VIR_KEYCODE_SET_RFB: self::libc::c_uint = 9;
pub type virKeycodeSet = Enum_Unnamed63;
pub type Enum_Unnamed64 = self::libc::c_uint;
pub const VIR_DOMAIN_PROCESS_SIGNAL_NOP: self::libc::c_uint = 0;
pub const VIR_DOMAIN_PROCESS_SIGNAL_HUP: self::libc::c_uint = 1;
pub const VIR_DOMAIN_PROCESS_SIGNAL_INT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_PROCESS_SIGNAL_QUIT: self::libc::c_uint = 3;
pub const VIR_DOMAIN_PROCESS_SIGNAL_ILL: self::libc::c_uint = 4;
pub const VIR_DOMAIN_PROCESS_SIGNAL_TRAP: self::libc::c_uint = 5;
pub const VIR_DOMAIN_PROCESS_SIGNAL_ABRT: self::libc::c_uint = 6;
pub const VIR_DOMAIN_PROCESS_SIGNAL_BUS: self::libc::c_uint = 7;
pub const VIR_DOMAIN_PROCESS_SIGNAL_FPE: self::libc::c_uint = 8;
pub const VIR_DOMAIN_PROCESS_SIGNAL_KILL: self::libc::c_uint = 9;
pub const VIR_DOMAIN_PROCESS_SIGNAL_USR1: self::libc::c_uint = 10;
pub const VIR_DOMAIN_PROCESS_SIGNAL_SEGV: self::libc::c_uint = 11;
pub const VIR_DOMAIN_PROCESS_SIGNAL_USR2: self::libc::c_uint = 12;
pub const VIR_DOMAIN_PROCESS_SIGNAL_PIPE: self::libc::c_uint = 13;
pub const VIR_DOMAIN_PROCESS_SIGNAL_ALRM: self::libc::c_uint = 14;
pub const VIR_DOMAIN_PROCESS_SIGNAL_TERM: self::libc::c_uint = 15;
pub const VIR_DOMAIN_PROCESS_SIGNAL_STKFLT: self::libc::c_uint = 16;
pub const VIR_DOMAIN_PROCESS_SIGNAL_CHLD: self::libc::c_uint = 17;
pub const VIR_DOMAIN_PROCESS_SIGNAL_CONT: self::libc::c_uint = 18;
pub const VIR_DOMAIN_PROCESS_SIGNAL_STOP: self::libc::c_uint = 19;
pub const VIR_DOMAIN_PROCESS_SIGNAL_TSTP: self::libc::c_uint = 20;
pub const VIR_DOMAIN_PROCESS_SIGNAL_TTIN: self::libc::c_uint = 21;
pub const VIR_DOMAIN_PROCESS_SIGNAL_TTOU: self::libc::c_uint = 22;
pub const VIR_DOMAIN_PROCESS_SIGNAL_URG: self::libc::c_uint = 23;
pub const VIR_DOMAIN_PROCESS_SIGNAL_XCPU: self::libc::c_uint = 24;
pub const VIR_DOMAIN_PROCESS_SIGNAL_XFSZ: self::libc::c_uint = 25;
pub const VIR_DOMAIN_PROCESS_SIGNAL_VTALRM: self::libc::c_uint = 26;
pub const VIR_DOMAIN_PROCESS_SIGNAL_PROF: self::libc::c_uint = 27;
pub const VIR_DOMAIN_PROCESS_SIGNAL_WINCH: self::libc::c_uint = 28;
pub const VIR_DOMAIN_PROCESS_SIGNAL_POLL: self::libc::c_uint = 29;
pub const VIR_DOMAIN_PROCESS_SIGNAL_PWR: self::libc::c_uint = 30;
pub const VIR_DOMAIN_PROCESS_SIGNAL_SYS: self::libc::c_uint = 31;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT0: self::libc::c_uint = 32;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT1: self::libc::c_uint = 33;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT2: self::libc::c_uint = 34;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT3: self::libc::c_uint = 35;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT4: self::libc::c_uint = 36;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT5: self::libc::c_uint = 37;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT6: self::libc::c_uint = 38;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT7: self::libc::c_uint = 39;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT8: self::libc::c_uint = 40;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT9: self::libc::c_uint = 41;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT10: self::libc::c_uint = 42;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT11: self::libc::c_uint = 43;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT12: self::libc::c_uint = 44;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT13: self::libc::c_uint = 45;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT14: self::libc::c_uint = 46;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT15: self::libc::c_uint = 47;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT16: self::libc::c_uint = 48;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT17: self::libc::c_uint = 49;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT18: self::libc::c_uint = 50;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT19: self::libc::c_uint = 51;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT20: self::libc::c_uint = 52;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT21: self::libc::c_uint = 53;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT22: self::libc::c_uint = 54;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT23: self::libc::c_uint = 55;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT24: self::libc::c_uint = 56;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT25: self::libc::c_uint = 57;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT26: self::libc::c_uint = 58;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT27: self::libc::c_uint = 59;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT28: self::libc::c_uint = 60;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT29: self::libc::c_uint = 61;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT30: self::libc::c_uint = 62;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT31: self::libc::c_uint = 63;
pub const VIR_DOMAIN_PROCESS_SIGNAL_RT32: self::libc::c_uint = 64;
pub type virDomainProcessSignal = Enum_Unnamed64;
pub enum Struct__virNodeDevice { }
pub type virNodeDevice = Struct__virNodeDevice;
pub type virNodeDevicePtr = *mut virNodeDevice;
pub type Enum_Unnamed65 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SYSTEM: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_PCI_DEV: self::libc::c_uint = 2;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_USB_DEV: self::libc::c_uint = 4;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_USB_INTERFACE: self::libc::c_uint = 8;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_NET: self::libc::c_uint = 16;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_HOST: self::libc::c_uint = 32;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_TARGET: self::libc::c_uint = 64;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI: self::libc::c_uint = 128;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_STORAGE: self::libc::c_uint = 256;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_FC_HOST: self::libc::c_uint = 512;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_VPORTS: self::libc::c_uint = 1024;
pub const VIR_CONNECT_LIST_NODE_DEVICES_CAP_SCSI_GENERIC: self::libc::c_uint = 2048;
pub type virConnectListAllNodeDeviceFlags = Enum_Unnamed65;
pub type Enum_Unnamed66 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_DEFINED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_UNDEFINED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_STARTED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_SUSPENDED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_RESUMED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_EVENT_STOPPED: self::libc::c_uint = 5;
pub const VIR_DOMAIN_EVENT_SHUTDOWN: self::libc::c_uint = 6;
pub const VIR_DOMAIN_EVENT_PMSUSPENDED: self::libc::c_uint = 7;
pub const VIR_DOMAIN_EVENT_CRASHED: self::libc::c_uint = 8;
pub type virDomainEventType = Enum_Unnamed66;
pub type Enum_Unnamed67 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_DEFINED_ADDED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_DEFINED_UPDATED: self::libc::c_uint = 1;
pub type virDomainEventDefinedDetailType = Enum_Unnamed67;
pub type Enum_Unnamed68 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_UNDEFINED_REMOVED: self::libc::c_uint = 0;
pub type virDomainEventUndefinedDetailType = Enum_Unnamed68;
pub type Enum_Unnamed69 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_STARTED_BOOTED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_STARTED_MIGRATED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_STARTED_RESTORED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_STARTED_FROM_SNAPSHOT: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_STARTED_WAKEUP: self::libc::c_uint = 4;
pub type virDomainEventStartedDetailType = Enum_Unnamed69;
pub type Enum_Unnamed70 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_SUSPENDED_PAUSED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_SUSPENDED_MIGRATED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_SUSPENDED_IOERROR: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_SUSPENDED_WATCHDOG: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_SUSPENDED_RESTORED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_EVENT_SUSPENDED_FROM_SNAPSHOT: self::libc::c_uint = 5;
pub const VIR_DOMAIN_EVENT_SUSPENDED_API_ERROR: self::libc::c_uint = 6;
pub type virDomainEventSuspendedDetailType = Enum_Unnamed70;
pub type Enum_Unnamed71 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_RESUMED_UNPAUSED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_RESUMED_MIGRATED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_RESUMED_FROM_SNAPSHOT: self::libc::c_uint = 2;
pub type virDomainEventResumedDetailType = Enum_Unnamed71;
pub type Enum_Unnamed72 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_STOPPED_SHUTDOWN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_STOPPED_DESTROYED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_STOPPED_CRASHED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_STOPPED_MIGRATED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_STOPPED_SAVED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_EVENT_STOPPED_FAILED: self::libc::c_uint = 5;
pub const VIR_DOMAIN_EVENT_STOPPED_FROM_SNAPSHOT: self::libc::c_uint = 6;
pub type virDomainEventStoppedDetailType = Enum_Unnamed72;
pub type Enum_Unnamed73 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_SHUTDOWN_FINISHED: self::libc::c_uint = 0;
pub type virDomainEventShutdownDetailType = Enum_Unnamed73;
pub type Enum_Unnamed74 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_PMSUSPENDED_MEMORY: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_PMSUSPENDED_DISK: self::libc::c_uint = 1;
pub type virDomainEventPMSuspendedDetailType = Enum_Unnamed74;
pub type Enum_Unnamed75 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_CRASHED_PANICKED: self::libc::c_uint = 0;
pub type virDomainEventCrashedDetailType = Enum_Unnamed75;
pub type virConnectDomainEventCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          event: self::libc::c_int,
                                          detail: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> self::libc::c_int>;
pub type Enum_Unnamed76 = self::libc::c_uint;
pub const VIR_EVENT_HANDLE_READABLE: self::libc::c_uint = 1;
pub const VIR_EVENT_HANDLE_WRITABLE: self::libc::c_uint = 2;
pub const VIR_EVENT_HANDLE_ERROR: self::libc::c_uint = 4;
pub const VIR_EVENT_HANDLE_HANGUP: self::libc::c_uint = 8;
pub type virEventHandleType = Enum_Unnamed76;
pub type virEventHandleCallback =
    ::std::option::Option<extern "C" fn(watch: self::libc::c_int,
                                          fd: self::libc::c_int,
                                          events: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virEventAddHandleFunc =
    ::std::option::Option<extern "C" fn(fd: self::libc::c_int,
                                          event: self::libc::c_int,
                                          cb: virEventHandleCallback,
                                          opaque: *mut self::libc::c_void,
                                          ff: virFreeCallback)
                                          -> self::libc::c_int>;
pub type virEventUpdateHandleFunc =
    ::std::option::Option<extern "C" fn(watch: self::libc::c_int, event: self::libc::c_int) -> ()>;
pub type virEventRemoveHandleFunc =
    ::std::option::Option<extern "C" fn(watch: self::libc::c_int) -> self::libc::c_int>;
pub type virEventTimeoutCallback =
    ::std::option::Option<extern "C" fn(timer: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virEventAddTimeoutFunc =
    ::std::option::Option<extern "C" fn(timeout: self::libc::c_int,
                                          cb: virEventTimeoutCallback,
                                          opaque: *mut self::libc::c_void,
                                          ff: virFreeCallback)
                                          -> self::libc::c_int>;
pub type virEventUpdateTimeoutFunc =
    ::std::option::Option<extern "C" fn(timer: self::libc::c_int, timeout: self::libc::c_int)
                                          -> ()>;
pub type virEventRemoveTimeoutFunc =
    ::std::option::Option<extern "C" fn(timer: self::libc::c_int) -> self::libc::c_int>;
pub enum Struct__virSecret { }
pub type virSecret = Struct__virSecret;
pub type virSecretPtr = *mut virSecret;
pub type Enum_Unnamed77 = self::libc::c_uint;
pub const VIR_SECRET_USAGE_TYPE_NONE: self::libc::c_uint = 0;
pub const VIR_SECRET_USAGE_TYPE_VOLUME: self::libc::c_uint = 1;
pub const VIR_SECRET_USAGE_TYPE_CEPH: self::libc::c_uint = 2;
pub const VIR_SECRET_USAGE_TYPE_ISCSI: self::libc::c_uint = 3;
pub type virSecretUsageType = Enum_Unnamed77;
pub type Enum_Unnamed78 = self::libc::c_uint;
pub const VIR_CONNECT_LIST_SECRETS_EPHEMERAL: self::libc::c_uint = 1;
pub const VIR_CONNECT_LIST_SECRETS_NO_EPHEMERAL: self::libc::c_uint = 2;
pub const VIR_CONNECT_LIST_SECRETS_PRIVATE: self::libc::c_uint = 4;
pub const VIR_CONNECT_LIST_SECRETS_NO_PRIVATE: self::libc::c_uint = 8;
pub type virConnectListAllSecretsFlags = Enum_Unnamed78;
pub type Enum_Unnamed79 = self::libc::c_uint;
pub const VIR_STREAM_NONBLOCK: self::libc::c_uint = 1;
pub type virStreamFlags = Enum_Unnamed79;
pub type virStreamSourceFunc =
    ::std::option::Option<extern "C" fn(st: virStreamPtr,
                                          data: *mut self::libc::c_char,
                                          nbytes: size_t,
                                          opaque: *mut self::libc::c_void)
                                          -> self::libc::c_int>;
pub type virStreamSinkFunc =
    ::std::option::Option<extern "C" fn(st: virStreamPtr,
                                          data: *const self::libc::c_char,
                                          nbytes: size_t,
                                          opaque: *mut self::libc::c_void)
                                          -> self::libc::c_int>;
pub type Enum_Unnamed80 = self::libc::c_uint;
pub const VIR_STREAM_EVENT_READABLE: self::libc::c_uint = 1;
pub const VIR_STREAM_EVENT_WRITABLE: self::libc::c_uint = 2;
pub const VIR_STREAM_EVENT_ERROR: self::libc::c_uint = 4;
pub const VIR_STREAM_EVENT_HANGUP: self::libc::c_uint = 8;
pub type virStreamEventType = Enum_Unnamed80;
pub type virStreamEventCallback =
    ::std::option::Option<extern "C" fn(stream: virStreamPtr,
                                          events: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed81 = self::libc::c_int;
pub const VIR_CPU_COMPARE_ERROR: self::libc::c_int = -1;
pub const VIR_CPU_COMPARE_INCOMPATIBLE: self::libc::c_int = 0;
pub const VIR_CPU_COMPARE_IDENTICAL: self::libc::c_int = 1;
pub const VIR_CPU_COMPARE_SUPERSET: self::libc::c_int = 2;
pub type virCPUCompareResult = Enum_Unnamed81;
pub type Enum_Unnamed82 = self::libc::c_uint;
pub const VIR_CONNECT_BASELINE_CPU_EXPAND_FEATURES: self::libc::c_uint = 1;
pub type virConnectBaselineCPUFlags = Enum_Unnamed82;
pub type Enum_Unnamed83 = self::libc::c_uint;
pub const VIR_DOMAIN_JOB_NONE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_JOB_BOUNDED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_JOB_UNBOUNDED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_JOB_COMPLETED: self::libc::c_uint = 3;
pub const VIR_DOMAIN_JOB_FAILED: self::libc::c_uint = 4;
pub const VIR_DOMAIN_JOB_CANCELLED: self::libc::c_uint = 5;
pub type virDomainJobType = Enum_Unnamed83;
pub type virDomainJobInfo = Struct__virDomainJobInfo;
pub type virDomainJobInfoPtr = *mut virDomainJobInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainJobInfo {
    pub _type: self::libc::c_int,
    pub timeElapsed: self::libc::c_ulonglong,
    pub timeRemaining: self::libc::c_ulonglong,
    pub dataTotal: self::libc::c_ulonglong,
    pub dataProcessed: self::libc::c_ulonglong,
    pub dataRemaining: self::libc::c_ulonglong,
    pub memTotal: self::libc::c_ulonglong,
    pub memProcessed: self::libc::c_ulonglong,
    pub memRemaining: self::libc::c_ulonglong,
    pub fileTotal: self::libc::c_ulonglong,
    pub fileProcessed: self::libc::c_ulonglong,
    pub fileRemaining: self::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct__virDomainJobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainJobInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub enum Struct__virDomainSnapshot { }
pub type virDomainSnapshot = Struct__virDomainSnapshot;
pub type virDomainSnapshotPtr = *mut virDomainSnapshot;
pub type Enum_Unnamed84 = self::libc::c_uint;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_REDEFINE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_CURRENT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_NO_METADATA: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_HALT: self::libc::c_uint = 8;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_DISK_ONLY: self::libc::c_uint = 16;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_REUSE_EXT: self::libc::c_uint = 32;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_QUIESCE: self::libc::c_uint = 64;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_ATOMIC: self::libc::c_uint = 128;
pub const VIR_DOMAIN_SNAPSHOT_CREATE_LIVE: self::libc::c_uint = 256;
pub type virDomainSnapshotCreateFlags = Enum_Unnamed84;
pub type Enum_Unnamed85 = self::libc::c_uint;
pub const VIR_DOMAIN_SNAPSHOT_LIST_ROOTS: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SNAPSHOT_LIST_DESCENDANTS: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SNAPSHOT_LIST_LEAVES: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SNAPSHOT_LIST_NO_LEAVES: self::libc::c_uint = 8;
pub const VIR_DOMAIN_SNAPSHOT_LIST_METADATA: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SNAPSHOT_LIST_NO_METADATA: self::libc::c_uint = 16;
pub const VIR_DOMAIN_SNAPSHOT_LIST_INACTIVE: self::libc::c_uint = 32;
pub const VIR_DOMAIN_SNAPSHOT_LIST_ACTIVE: self::libc::c_uint = 64;
pub const VIR_DOMAIN_SNAPSHOT_LIST_DISK_ONLY: self::libc::c_uint = 128;
pub const VIR_DOMAIN_SNAPSHOT_LIST_INTERNAL: self::libc::c_uint = 256;
pub const VIR_DOMAIN_SNAPSHOT_LIST_EXTERNAL: self::libc::c_uint = 512;
pub type virDomainSnapshotListFlags = Enum_Unnamed85;
pub type Enum_Unnamed86 = self::libc::c_uint;
pub const VIR_DOMAIN_SNAPSHOT_REVERT_RUNNING: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SNAPSHOT_REVERT_PAUSED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SNAPSHOT_REVERT_FORCE: self::libc::c_uint = 4;
pub type virDomainSnapshotRevertFlags = Enum_Unnamed86;
pub type Enum_Unnamed87 = self::libc::c_uint;
pub const VIR_DOMAIN_SNAPSHOT_DELETE_CHILDREN: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SNAPSHOT_DELETE_METADATA_ONLY: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SNAPSHOT_DELETE_CHILDREN_ONLY: self::libc::c_uint = 4;
pub type virDomainSnapshotDeleteFlags = Enum_Unnamed87;
pub type virConnectDomainEventGenericCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventRTCChangeCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          utcoffset: self::libc::c_longlong,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed88 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_WATCHDOG_NONE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_WATCHDOG_PAUSE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_WATCHDOG_RESET: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_WATCHDOG_POWEROFF: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_WATCHDOG_SHUTDOWN: self::libc::c_uint = 4;
pub const VIR_DOMAIN_EVENT_WATCHDOG_DEBUG: self::libc::c_uint = 5;
pub type virDomainEventWatchdogAction = Enum_Unnamed88;
pub type virConnectDomainEventWatchdogCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          action: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed89 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_IO_ERROR_NONE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_IO_ERROR_PAUSE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_IO_ERROR_REPORT: self::libc::c_uint = 2;
pub type virDomainEventIOErrorAction = Enum_Unnamed89;
pub type virConnectDomainEventIOErrorCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          srcPath: *const self::libc::c_char,
                                          devAlias: *const self::libc::c_char,
                                          action: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventIOErrorReasonCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          srcPath: *const self::libc::c_char,
                                          devAlias: *const self::libc::c_char,
                                          action: self::libc::c_int,
                                          reason: *const self::libc::c_char,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed90 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_GRAPHICS_CONNECT: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_GRAPHICS_INITIALIZE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_GRAPHICS_DISCONNECT: self::libc::c_uint = 2;
pub type virDomainEventGraphicsPhase = Enum_Unnamed90;
pub type Enum_Unnamed91 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_GRAPHICS_ADDRESS_IPV4: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_GRAPHICS_ADDRESS_IPV6: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_GRAPHICS_ADDRESS_UNIX: self::libc::c_uint = 2;
pub type virDomainEventGraphicsAddressType = Enum_Unnamed91;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainEventGraphicsAddress {
    pub family: self::libc::c_int,
    pub node: *mut self::libc::c_char,
    pub service: *mut self::libc::c_char,
}
impl ::std::clone::Clone for Struct__virDomainEventGraphicsAddress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainEventGraphicsAddress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainEventGraphicsAddress = Struct__virDomainEventGraphicsAddress;
pub type virDomainEventGraphicsAddressPtr = *mut virDomainEventGraphicsAddress;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainEventGraphicsSubjectIdentity {
    pub _type: *mut self::libc::c_char,
    pub name: *mut self::libc::c_char,
}
impl ::std::clone::Clone for Struct__virDomainEventGraphicsSubjectIdentity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainEventGraphicsSubjectIdentity {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainEventGraphicsSubjectIdentity = Struct__virDomainEventGraphicsSubjectIdentity;
pub type virDomainEventGraphicsSubjectIdentityPtr = *mut virDomainEventGraphicsSubjectIdentity;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virDomainEventGraphicsSubject {
    pub nidentity: self::libc::c_int,
    pub identities: virDomainEventGraphicsSubjectIdentityPtr,
}
impl ::std::clone::Clone for Struct__virDomainEventGraphicsSubject {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virDomainEventGraphicsSubject {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type virDomainEventGraphicsSubject = Struct__virDomainEventGraphicsSubject;
pub type virDomainEventGraphicsSubjectPtr = *mut virDomainEventGraphicsSubject;
pub type virConnectDomainEventGraphicsCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          phase: self::libc::c_int,
                                          local: *const virDomainEventGraphicsAddress,
                                          remote: *const virDomainEventGraphicsAddress,
                                          authScheme: *const self::libc::c_char,
                                          subject: *const virDomainEventGraphicsSubject,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed92 = self::libc::c_uint;
pub const VIR_DOMAIN_BLOCK_JOB_COMPLETED: self::libc::c_uint = 0;
pub const VIR_DOMAIN_BLOCK_JOB_FAILED: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLOCK_JOB_CANCELED: self::libc::c_uint = 2;
pub const VIR_DOMAIN_BLOCK_JOB_READY: self::libc::c_uint = 3;
pub type virConnectDomainEventBlockJobStatus = Enum_Unnamed92;
pub type virConnectDomainEventBlockJobCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          disk: *const self::libc::c_char,
                                          _type: self::libc::c_int,
                                          status: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed93 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_DISK_CHANGE_MISSING_ON_START: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_DISK_DROP_MISSING_ON_START: self::libc::c_uint = 1;
pub type virConnectDomainEventDiskChangeReason = Enum_Unnamed93;
pub type virConnectDomainEventDiskChangeCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          oldSrcPath: *const self::libc::c_char,
                                          newSrcPath: *const self::libc::c_char,
                                          devAlias: *const self::libc::c_char,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed94 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_TRAY_CHANGE_OPEN: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_TRAY_CHANGE_CLOSE: self::libc::c_uint = 1;
pub type virDomainEventTrayChangeReason = Enum_Unnamed94;
pub type virConnectDomainEventTrayChangeCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          devAlias: *const self::libc::c_char,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventPMWakeupCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventPMSuspendCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventBalloonChangeCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          actual: self::libc::c_ulonglong,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventPMSuspendDiskCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          reason: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type virConnectDomainEventDeviceRemovedCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          dom: virDomainPtr,
                                          devAlias: *const self::libc::c_char,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed95 = self::libc::c_uint;
pub const VIR_DOMAIN_EVENT_ID_LIFECYCLE: self::libc::c_uint = 0;
pub const VIR_DOMAIN_EVENT_ID_REBOOT: self::libc::c_uint = 1;
pub const VIR_DOMAIN_EVENT_ID_RTC_CHANGE: self::libc::c_uint = 2;
pub const VIR_DOMAIN_EVENT_ID_WATCHDOG: self::libc::c_uint = 3;
pub const VIR_DOMAIN_EVENT_ID_IO_ERROR: self::libc::c_uint = 4;
pub const VIR_DOMAIN_EVENT_ID_GRAPHICS: self::libc::c_uint = 5;
pub const VIR_DOMAIN_EVENT_ID_IO_ERROR_REASON: self::libc::c_uint = 6;
pub const VIR_DOMAIN_EVENT_ID_CONTROL_ERROR: self::libc::c_uint = 7;
pub const VIR_DOMAIN_EVENT_ID_BLOCK_JOB: self::libc::c_uint = 8;
pub const VIR_DOMAIN_EVENT_ID_DISK_CHANGE: self::libc::c_uint = 9;
pub const VIR_DOMAIN_EVENT_ID_TRAY_CHANGE: self::libc::c_uint = 10;
pub const VIR_DOMAIN_EVENT_ID_PMWAKEUP: self::libc::c_uint = 11;
pub const VIR_DOMAIN_EVENT_ID_PMSUSPEND: self::libc::c_uint = 12;
pub const VIR_DOMAIN_EVENT_ID_BALLOON_CHANGE: self::libc::c_uint = 13;
pub const VIR_DOMAIN_EVENT_ID_PMSUSPEND_DISK: self::libc::c_uint = 14;
pub const VIR_DOMAIN_EVENT_ID_DEVICE_REMOVED: self::libc::c_uint = 15;
pub type virDomainEventID = Enum_Unnamed95;
pub type Enum_Unnamed96 = self::libc::c_uint;
pub const VIR_NETWORK_EVENT_DEFINED: self::libc::c_uint = 0;
pub const VIR_NETWORK_EVENT_UNDEFINED: self::libc::c_uint = 1;
pub const VIR_NETWORK_EVENT_STARTED: self::libc::c_uint = 2;
pub const VIR_NETWORK_EVENT_STOPPED: self::libc::c_uint = 3;
pub type virNetworkEventLifecycleType = Enum_Unnamed96;
pub type virConnectNetworkEventLifecycleCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          net: virNetworkPtr,
                                          event: self::libc::c_int,
                                          detail: self::libc::c_int,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub type Enum_Unnamed97 = self::libc::c_uint;
pub const VIR_NETWORK_EVENT_ID_LIFECYCLE: self::libc::c_uint = 0;
pub type virNetworkEventID = Enum_Unnamed97;
pub type virConnectNetworkEventGenericCallback =
    ::std::option::Option<extern "C" fn(conn: virConnectPtr,
                                          net: virNetworkPtr,
                                          opaque: *mut self::libc::c_void)
                                          -> ()>;
pub enum Struct__virNWFilter { }
pub type virNWFilter = Struct__virNWFilter;
pub type virNWFilterPtr = *mut virNWFilter;
pub type Enum_Unnamed98 = self::libc::c_uint;
pub const VIR_DOMAIN_CONSOLE_FORCE: self::libc::c_uint = 1;
pub const VIR_DOMAIN_CONSOLE_SAFE: self::libc::c_uint = 2;
pub type virDomainConsoleFlags = Enum_Unnamed98;
pub type Enum_Unnamed99 = self::libc::c_uint;
pub const VIR_DOMAIN_CHANNEL_FORCE: self::libc::c_uint = 1;
pub type virDomainChannelFlags = Enum_Unnamed99;
pub type Enum_Unnamed100 = self::libc::c_uint;
pub const VIR_DOMAIN_OPEN_GRAPHICS_SKIPAUTH: self::libc::c_uint = 1;
pub type virDomainOpenGraphicsFlags = Enum_Unnamed100;
pub type Enum_Unnamed101 = self::libc::c_uint;
pub const VIR_DOMAIN_SCHED_FIELD_INT: self::libc::c_uint = 1;
pub const VIR_DOMAIN_SCHED_FIELD_UINT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_SCHED_FIELD_LLONG: self::libc::c_uint = 3;
pub const VIR_DOMAIN_SCHED_FIELD_ULLONG: self::libc::c_uint = 4;
pub const VIR_DOMAIN_SCHED_FIELD_DOUBLE: self::libc::c_uint = 5;
pub const VIR_DOMAIN_SCHED_FIELD_BOOLEAN: self::libc::c_uint = 6;
pub type virSchedParameterType = Enum_Unnamed101;
pub type virSchedParameter = Struct__virTypedParameter;
pub type virSchedParameterPtr = *mut virSchedParameter;
pub type Enum_Unnamed102 = self::libc::c_uint;
pub const VIR_DOMAIN_BLKIO_PARAM_INT: self::libc::c_uint = 1;
pub const VIR_DOMAIN_BLKIO_PARAM_UINT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_BLKIO_PARAM_LLONG: self::libc::c_uint = 3;
pub const VIR_DOMAIN_BLKIO_PARAM_ULLONG: self::libc::c_uint = 4;
pub const VIR_DOMAIN_BLKIO_PARAM_DOUBLE: self::libc::c_uint = 5;
pub const VIR_DOMAIN_BLKIO_PARAM_BOOLEAN: self::libc::c_uint = 6;
pub type virBlkioParameterType = Enum_Unnamed102;
pub type virBlkioParameter = Struct__virTypedParameter;
pub type virBlkioParameterPtr = *mut virBlkioParameter;
pub type Enum_Unnamed103 = self::libc::c_uint;
pub const VIR_DOMAIN_MEMORY_PARAM_INT: self::libc::c_uint = 1;
pub const VIR_DOMAIN_MEMORY_PARAM_UINT: self::libc::c_uint = 2;
pub const VIR_DOMAIN_MEMORY_PARAM_LLONG: self::libc::c_uint = 3;
pub const VIR_DOMAIN_MEMORY_PARAM_ULLONG: self::libc::c_uint = 4;
pub const VIR_DOMAIN_MEMORY_PARAM_DOUBLE: self::libc::c_uint = 5;
pub const VIR_DOMAIN_MEMORY_PARAM_BOOLEAN: self::libc::c_uint = 6;
pub type virMemoryParameterType = Enum_Unnamed103;
pub type virMemoryParameter = Struct__virTypedParameter;
pub type virMemoryParameterPtr = *mut virMemoryParameter;
pub type Enum_Unnamed135 = ::libc::c_uint;
pub const VIR_ERR_NONE: ::libc::c_uint = 0;
pub const VIR_ERR_WARNING: ::libc::c_uint = 1;
pub const VIR_ERR_ERROR: ::libc::c_uint = 2;
pub type virErrorLevel = Enum_Unnamed135;
pub type Enum_Unnamed136 = ::libc::c_uint;
pub const VIR_FROM_NONE: ::libc::c_uint = 0;
pub const VIR_FROM_XEN: ::libc::c_uint = 1;
pub const VIR_FROM_XEND: ::libc::c_uint = 2;
pub const VIR_FROM_XENSTORE: ::libc::c_uint = 3;
pub const VIR_FROM_SEXPR: ::libc::c_uint = 4;
pub const VIR_FROM_XML: ::libc::c_uint = 5;
pub const VIR_FROM_DOM: ::libc::c_uint = 6;
pub const VIR_FROM_RPC: ::libc::c_uint = 7;
pub const VIR_FROM_PROXY: ::libc::c_uint = 8;
pub const VIR_FROM_CONF: ::libc::c_uint = 9;
pub const VIR_FROM_QEMU: ::libc::c_uint = 10;
pub const VIR_FROM_NET: ::libc::c_uint = 11;
pub const VIR_FROM_TEST: ::libc::c_uint = 12;
pub const VIR_FROM_REMOTE: ::libc::c_uint = 13;
pub const VIR_FROM_OPENVZ: ::libc::c_uint = 14;
pub const VIR_FROM_XENXM: ::libc::c_uint = 15;
pub const VIR_FROM_STATS_LINUX: ::libc::c_uint = 16;
pub const VIR_FROM_LXC: ::libc::c_uint = 17;
pub const VIR_FROM_STORAGE: ::libc::c_uint = 18;
pub const VIR_FROM_NETWORK: ::libc::c_uint = 19;
pub const VIR_FROM_DOMAIN: ::libc::c_uint = 20;
pub const VIR_FROM_UML: ::libc::c_uint = 21;
pub const VIR_FROM_NODEDEV: ::libc::c_uint = 22;
pub const VIR_FROM_XEN_INOTIFY: ::libc::c_uint = 23;
pub const VIR_FROM_SECURITY: ::libc::c_uint = 24;
pub const VIR_FROM_VBOX: ::libc::c_uint = 25;
pub const VIR_FROM_INTERFACE: ::libc::c_uint = 26;
pub const VIR_FROM_ONE: ::libc::c_uint = 27;
pub const VIR_FROM_ESX: ::libc::c_uint = 28;
pub const VIR_FROM_PHYP: ::libc::c_uint = 29;
pub const VIR_FROM_SECRET: ::libc::c_uint = 30;
pub const VIR_FROM_CPU: ::libc::c_uint = 31;
pub const VIR_FROM_XENAPI: ::libc::c_uint = 32;
pub const VIR_FROM_NWFILTER: ::libc::c_uint = 33;
pub const VIR_FROM_HOOK: ::libc::c_uint = 34;
pub const VIR_FROM_DOMAIN_SNAPSHOT: ::libc::c_uint = 35;
pub const VIR_FROM_AUDIT: ::libc::c_uint = 36;
pub const VIR_FROM_SYSINFO: ::libc::c_uint = 37;
pub const VIR_FROM_STREAMS: ::libc::c_uint = 38;
pub const VIR_FROM_VMWARE: ::libc::c_uint = 39;
pub const VIR_FROM_EVENT: ::libc::c_uint = 40;
pub const VIR_FROM_LIBXL: ::libc::c_uint = 41;
pub const VIR_FROM_LOCKING: ::libc::c_uint = 42;
pub const VIR_FROM_HYPERV: ::libc::c_uint = 43;
pub const VIR_FROM_CAPABILITIES: ::libc::c_uint = 44;
pub const VIR_FROM_URI: ::libc::c_uint = 45;
pub const VIR_FROM_AUTH: ::libc::c_uint = 46;
pub const VIR_FROM_DBUS: ::libc::c_uint = 47;
pub const VIR_FROM_PARALLELS: ::libc::c_uint = 48;
pub const VIR_FROM_DEVICE: ::libc::c_uint = 49;
pub const VIR_FROM_SSH: ::libc::c_uint = 50;
pub const VIR_FROM_LOCKSPACE: ::libc::c_uint = 51;
pub const VIR_FROM_INITCTL: ::libc::c_uint = 52;
pub const VIR_FROM_IDENTITY: ::libc::c_uint = 53;
pub const VIR_FROM_CGROUP: ::libc::c_uint = 54;
pub const VIR_FROM_ACCESS: ::libc::c_uint = 55;
pub const VIR_FROM_SYSTEMD: ::libc::c_uint = 56;
pub const VIR_FROM_BHYVE: ::libc::c_uint = 57;
pub const VIR_FROM_CRYPTO: ::libc::c_uint = 58;
pub const VIR_FROM_FIREWALL: ::libc::c_uint = 59;
pub const VIR_FROM_POLKIT: ::libc::c_uint = 60;
pub const VIR_FROM_THREAD: ::libc::c_uint = 61;
pub const VIR_FROM_ADMIN: ::libc::c_uint = 62;

pub type Enum_Unnamed137 = ::libc::c_uint;
pub const VIR_ERR_OK: ::libc::c_uint = 0;
pub const VIR_ERR_INTERNAL_ERROR: ::libc::c_uint = 1;
pub const VIR_ERR_NO_MEMORY: ::libc::c_uint = 2;
pub const VIR_ERR_NO_SUPPORT: ::libc::c_uint = 3;
pub const VIR_ERR_UNKNOWN_HOST: ::libc::c_uint = 4;
pub const VIR_ERR_NO_CONNECT: ::libc::c_uint = 5;
pub const VIR_ERR_INVALID_CONN: ::libc::c_uint = 6;
pub const VIR_ERR_INVALID_DOMAIN: ::libc::c_uint = 7;
pub const VIR_ERR_INVALID_ARG: ::libc::c_uint = 8;
pub const VIR_ERR_OPERATION_FAILED: ::libc::c_uint = 9;
pub const VIR_ERR_GET_FAILED: ::libc::c_uint = 10;
pub const VIR_ERR_POST_FAILED: ::libc::c_uint = 11;
pub const VIR_ERR_HTTP_ERROR: ::libc::c_uint = 12;
pub const VIR_ERR_SEXPR_SERIAL: ::libc::c_uint = 13;
pub const VIR_ERR_NO_XEN: ::libc::c_uint = 14;
pub const VIR_ERR_XEN_CALL: ::libc::c_uint = 15;
pub const VIR_ERR_OS_TYPE: ::libc::c_uint = 16;
pub const VIR_ERR_NO_KERNEL: ::libc::c_uint = 17;
pub const VIR_ERR_NO_ROOT: ::libc::c_uint = 18;
pub const VIR_ERR_NO_SOURCE: ::libc::c_uint = 19;
pub const VIR_ERR_NO_TARGET: ::libc::c_uint = 20;
pub const VIR_ERR_NO_NAME: ::libc::c_uint = 21;
pub const VIR_ERR_NO_OS: ::libc::c_uint = 22;
pub const VIR_ERR_NO_DEVICE: ::libc::c_uint = 23;
pub const VIR_ERR_NO_XENSTORE: ::libc::c_uint = 24;
pub const VIR_ERR_DRIVER_FULL: ::libc::c_uint = 25;
pub const VIR_ERR_CALL_FAILED: ::libc::c_uint = 26;
pub const VIR_ERR_XML_ERROR: ::libc::c_uint = 27;
pub const VIR_ERR_DOM_EXIST: ::libc::c_uint = 28;
pub const VIR_ERR_OPERATION_DENIED: ::libc::c_uint = 29;
pub const VIR_ERR_OPEN_FAILED: ::libc::c_uint = 30;
pub const VIR_ERR_READ_FAILED: ::libc::c_uint = 31;
pub const VIR_ERR_PARSE_FAILED: ::libc::c_uint = 32;
pub const VIR_ERR_CONF_SYNTAX: ::libc::c_uint = 33;
pub const VIR_ERR_WRITE_FAILED: ::libc::c_uint = 34;
pub const VIR_ERR_XML_DETAIL: ::libc::c_uint = 35;
pub const VIR_ERR_INVALID_NETWORK: ::libc::c_uint = 36;
pub const VIR_ERR_NETWORK_EXIST: ::libc::c_uint = 37;
pub const VIR_ERR_SYSTEM_ERROR: ::libc::c_uint = 38;
pub const VIR_ERR_RPC: ::libc::c_uint = 39;
pub const VIR_ERR_GNUTLS_ERROR: ::libc::c_uint = 40;
pub const VIR_WAR_NO_NETWORK: ::libc::c_uint = 41;
pub const VIR_ERR_NO_DOMAIN: ::libc::c_uint = 42;
pub const VIR_ERR_NO_NETWORK: ::libc::c_uint = 43;
pub const VIR_ERR_INVALID_MAC: ::libc::c_uint = 44;
pub const VIR_ERR_AUTH_FAILED: ::libc::c_uint = 45;
pub const VIR_ERR_INVALID_STORAGE_POOL: ::libc::c_uint = 46;
pub const VIR_ERR_INVALID_STORAGE_VOL: ::libc::c_uint = 47;
pub const VIR_WAR_NO_STORAGE: ::libc::c_uint = 48;
pub const VIR_ERR_NO_STORAGE_POOL: ::libc::c_uint = 49;
pub const VIR_ERR_NO_STORAGE_VOL: ::libc::c_uint = 50;
pub const VIR_WAR_NO_NODE: ::libc::c_uint = 51;
pub const VIR_ERR_INVALID_NODE_DEVICE: ::libc::c_uint = 52;
pub const VIR_ERR_NO_NODE_DEVICE: ::libc::c_uint = 53;
pub const VIR_ERR_NO_SECURITY_MODEL: ::libc::c_uint = 54;
pub const VIR_ERR_OPERATION_INVALID: ::libc::c_uint = 55;
pub const VIR_WAR_NO_INTERFACE: ::libc::c_uint = 56;
pub const VIR_ERR_NO_INTERFACE: ::libc::c_uint = 57;
pub const VIR_ERR_INVALID_INTERFACE: ::libc::c_uint = 58;
pub const VIR_ERR_MULTIPLE_INTERFACES: ::libc::c_uint = 59;
pub const VIR_WAR_NO_NWFILTER: ::libc::c_uint = 60;
pub const VIR_ERR_INVALID_NWFILTER: ::libc::c_uint = 61;
pub const VIR_ERR_NO_NWFILTER: ::libc::c_uint = 62;
pub const VIR_ERR_BUILD_FIREWALL: ::libc::c_uint = 63;
pub const VIR_WAR_NO_SECRET: ::libc::c_uint = 64;
pub const VIR_ERR_INVALID_SECRET: ::libc::c_uint = 65;
pub const VIR_ERR_NO_SECRET: ::libc::c_uint = 66;
pub const VIR_ERR_CONFIG_UNSUPPORTED: ::libc::c_uint = 67;
pub const VIR_ERR_OPERATION_TIMEOUT: ::libc::c_uint = 68;
pub const VIR_ERR_MIGRATE_PERSIST_FAILED: ::libc::c_uint = 69;
pub const VIR_ERR_HOOK_SCRIPT_FAILED: ::libc::c_uint = 70;
pub const VIR_ERR_INVALID_DOMAIN_SNAPSHOT: ::libc::c_uint = 71;
pub const VIR_ERR_NO_DOMAIN_SNAPSHOT: ::libc::c_uint = 72;
pub const VIR_ERR_INVALID_STREAM: ::libc::c_uint = 73;
pub const VIR_ERR_ARGUMENT_UNSUPPORTED: ::libc::c_uint = 74;
pub const VIR_ERR_STORAGE_PROBE_FAILED: ::libc::c_uint = 75;
pub const VIR_ERR_STORAGE_POOL_BUILT: ::libc::c_uint = 76;
pub const VIR_ERR_SNAPSHOT_REVERT_RISKY: ::libc::c_uint = 77;
pub const VIR_ERR_OPERATION_ABORTED: ::libc::c_uint = 78;
pub const VIR_ERR_AUTH_CANCELLED: ::libc::c_uint = 79;
pub const VIR_ERR_NO_DOMAIN_METADATA: ::libc::c_uint = 80;
pub const VIR_ERR_MIGRATE_UNSAFE: ::libc::c_uint = 81;
pub const VIR_ERR_OVERFLOW: ::libc::c_uint = 82;
pub const VIR_ERR_BLOCK_COPY_ACTIVE: ::libc::c_uint = 83;
pub const VIR_ERR_OPERATION_UNSUPPORTED: ::libc::c_uint = 84;
pub const VIR_ERR_SSH: ::libc::c_uint = 85;
pub const VIR_ERR_AGENT_UNRESPONSIVE: ::libc::c_uint = 86;
pub const VIR_ERR_RESOURCE_BUSY: ::libc::c_uint = 87;
pub const VIR_ERR_ACCESS_DENIED: ::libc::c_uint = 88;
pub const VIR_ERR_DBUS_SERVICE: ::libc::c_uint = 89;
pub const VIR_ERR_STORAGE_VOL_EXIST: ::libc::c_uint = 90;
pub const VIR_ERR_CPU_INCOMPATIBLE: ::libc::c_uint = 91;
pub const VIR_ERR_XML_INVALID_SCHEMA: ::libc::c_uint = 92;
pub const VIR_ERR_MIGRATE_FINISH_OK: ::libc::c_uint = 93;
pub type virErrorNumber = Enum_Unnamed137;
pub type virErrorFunc = ::std::option::Option<extern "C" fn(userData: *mut ::libc::c_void,
                                                              error: virErrorPtr)
                                                              -> ()>;
pub type virErrorDomain = Enum_Unnamed136;
pub type virError = Struct__virError;
pub type virErrorPtr = *mut virError;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__virError {
    pub code: ::libc::c_int,
    pub domain: ::libc::c_int,
    pub message: *mut ::libc::c_char,
    pub level: virErrorLevel,
    pub conn: virConnectPtr,
    pub dom: virDomainPtr,
    pub str1: *mut ::libc::c_char,
    pub str2: *mut ::libc::c_char,
    pub str3: *mut ::libc::c_char,
    pub int1: ::libc::c_int,
    pub int2: ::libc::c_int,
    pub net: virNetworkPtr,
}
impl ::std::clone::Clone for Struct__virError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__virError {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[link(name = "virt")]
extern "C" {
    pub static mut virConnectAuthPtrDefault: virConnectAuthPtr;
}
#[link(name = "virt")]
extern "C" {
    pub fn virTypedParamsGet(params: virTypedParameterPtr,
                             nparams: self::libc::c_int,
                             name: *const self::libc::c_char)
                             -> virTypedParameterPtr;
    pub fn virTypedParamsGetInt(params: virTypedParameterPtr,
                                nparams: self::libc::c_int,
                                name: *const self::libc::c_char,
                                value: *mut self::libc::c_int)
                                -> self::libc::c_int;
    pub fn virTypedParamsGetUInt(params: virTypedParameterPtr,
                                 nparams: self::libc::c_int,
                                 name: *const self::libc::c_char,
                                 value: *mut self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virTypedParamsGetLLong(params: virTypedParameterPtr,
                                  nparams: self::libc::c_int,
                                  name: *const self::libc::c_char,
                                  value: *mut self::libc::c_longlong)
                                  -> self::libc::c_int;
    pub fn virTypedParamsGetULLong(params: virTypedParameterPtr,
                                   nparams: self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: *mut self::libc::c_ulonglong)
                                   -> self::libc::c_int;
    pub fn virTypedParamsGetDouble(params: virTypedParameterPtr,
                                   nparams: self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: *mut self::libc::c_double)
                                   -> self::libc::c_int;
    pub fn virTypedParamsGetBoolean(params: virTypedParameterPtr,
                                    nparams: self::libc::c_int,
                                    name: *const self::libc::c_char,
                                    value: *mut self::libc::c_int)
                                    -> self::libc::c_int;
    pub fn virTypedParamsGetString(params: virTypedParameterPtr,
                                   nparams: self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: *mut *const self::libc::c_char)
                                   -> self::libc::c_int;
    pub fn virTypedParamsAddInt(params: *mut virTypedParameterPtr,
                                nparams: *mut self::libc::c_int,
                                maxparams: *mut self::libc::c_int,
                                name: *const self::libc::c_char,
                                value: self::libc::c_int)
                                -> self::libc::c_int;
    pub fn virTypedParamsAddUInt(params: *mut virTypedParameterPtr,
                                 nparams: *mut self::libc::c_int,
                                 maxparams: *mut self::libc::c_int,
                                 name: *const self::libc::c_char,
                                 value: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virTypedParamsAddLLong(params: *mut virTypedParameterPtr,
                                  nparams: *mut self::libc::c_int,
                                  maxparams: *mut self::libc::c_int,
                                  name: *const self::libc::c_char,
                                  value: self::libc::c_longlong)
                                  -> self::libc::c_int;
    pub fn virTypedParamsAddULLong(params: *mut virTypedParameterPtr,
                                   nparams: *mut self::libc::c_int,
                                   maxparams: *mut self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: self::libc::c_ulonglong)
                                   -> self::libc::c_int;
    pub fn virTypedParamsAddDouble(params: *mut virTypedParameterPtr,
                                   nparams: *mut self::libc::c_int,
                                   maxparams: *mut self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: self::libc::c_double)
                                   -> self::libc::c_int;
    pub fn virTypedParamsAddBoolean(params: *mut virTypedParameterPtr,
                                    nparams: *mut self::libc::c_int,
                                    maxparams: *mut self::libc::c_int,
                                    name: *const self::libc::c_char,
                                    value: self::libc::c_int)
                                    -> self::libc::c_int;
    pub fn virTypedParamsAddString(params: *mut virTypedParameterPtr,
                                   nparams: *mut self::libc::c_int,
                                   maxparams: *mut self::libc::c_int,
                                   name: *const self::libc::c_char,
                                   value: *const self::libc::c_char)
                                   -> self::libc::c_int;
    pub fn virTypedParamsAddFromString(params: *mut virTypedParameterPtr,
                                       nparams: *mut self::libc::c_int,
                                       maxparams: *mut self::libc::c_int,
                                       name: *const self::libc::c_char,
                                       _type: self::libc::c_int,
                                       value: *const self::libc::c_char)
                                       -> self::libc::c_int;
    pub fn virTypedParamsClear(params: virTypedParameterPtr, nparams: self::libc::c_int) -> ();
    pub fn virTypedParamsFree(params: virTypedParameterPtr, nparams: self::libc::c_int) -> ();
    pub fn virNodeGetMemoryParameters(conn: virConnectPtr,
                                      params: virTypedParameterPtr,
                                      nparams: *mut self::libc::c_int,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virNodeSetMemoryParameters(conn: virConnectPtr,
                                      params: virTypedParameterPtr,
                                      nparams: self::libc::c_int,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virNodeGetCPUMap(conn: virConnectPtr,
                            cpumap: *mut *mut self::libc::c_uchar,
                            online: *mut self::libc::c_uint,
                            flags: self::libc::c_uint)
                            -> self::libc::c_int;
    pub fn virDomainGetSchedulerParameters(domain: virDomainPtr,
                                           params: virTypedParameterPtr,
                                           nparams: *mut self::libc::c_int)
                                           -> self::libc::c_int;
    pub fn virDomainGetSchedulerParametersFlags(domain: virDomainPtr,
                                                params: virTypedParameterPtr,
                                                nparams: *mut self::libc::c_int,
                                                flags: self::libc::c_uint)
                                                -> self::libc::c_int;
    pub fn virDomainSetSchedulerParameters(domain: virDomainPtr,
                                           params: virTypedParameterPtr,
                                           nparams: self::libc::c_int)
                                           -> self::libc::c_int;
    pub fn virDomainSetSchedulerParametersFlags(domain: virDomainPtr,
                                                params: virTypedParameterPtr,
                                                nparams: self::libc::c_int,
                                                flags: self::libc::c_uint)
                                                -> self::libc::c_int;
    pub fn virDomainMigrate(domain: virDomainPtr,
                            dconn: virConnectPtr,
                            flags: self::libc::c_ulong,
                            dname: *const self::libc::c_char,
                            uri: *const self::libc::c_char,
                            bandwidth: self::libc::c_ulong)
                            -> virDomainPtr;
    pub fn virDomainMigrate2(domain: virDomainPtr,
                             dconn: virConnectPtr,
                             dxml: *const self::libc::c_char,
                             flags: self::libc::c_ulong,
                             dname: *const self::libc::c_char,
                             uri: *const self::libc::c_char,
                             bandwidth: self::libc::c_ulong)
                             -> virDomainPtr;
    pub fn virDomainMigrate3(domain: virDomainPtr,
                             dconn: virConnectPtr,
                             params: virTypedParameterPtr,
                             nparams: self::libc::c_uint,
                             flags: self::libc::c_uint)
                             -> virDomainPtr;
    pub fn virDomainMigrateToURI(domain: virDomainPtr,
                                 duri: *const self::libc::c_char,
                                 flags: self::libc::c_ulong,
                                 dname: *const self::libc::c_char,
                                 bandwidth: self::libc::c_ulong)
                                 -> self::libc::c_int;
    pub fn virDomainMigrateToURI2(domain: virDomainPtr,
                                  dconnuri: *const self::libc::c_char,
                                  miguri: *const self::libc::c_char,
                                  dxml: *const self::libc::c_char,
                                  flags: self::libc::c_ulong,
                                  dname: *const self::libc::c_char,
                                  bandwidth: self::libc::c_ulong)
                                  -> self::libc::c_int;
    pub fn virDomainMigrateToURI3(domain: virDomainPtr,
                                  dconnuri: *const self::libc::c_char,
                                  params: virTypedParameterPtr,
                                  nparams: self::libc::c_uint,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virDomainMigrateSetMaxDowntime(domain: virDomainPtr,
                                          downtime: self::libc::c_ulonglong,
                                          flags: self::libc::c_uint)
                                          -> self::libc::c_int;
    pub fn virDomainMigrateGetCompressionCache(domain: virDomainPtr,
                                               cacheSize: *mut self::libc::c_ulonglong,
                                               flags: self::libc::c_uint)
                                               -> self::libc::c_int;
    pub fn virDomainMigrateSetCompressionCache(domain: virDomainPtr,
                                               cacheSize: self::libc::c_ulonglong,
                                               flags: self::libc::c_uint)
                                               -> self::libc::c_int;
    pub fn virDomainMigrateSetMaxSpeed(domain: virDomainPtr,
                                       bandwidth: self::libc::c_ulong,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainMigrateGetMaxSpeed(domain: virDomainPtr,
                                       bandwidth: *mut self::libc::c_ulong,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virGetVersion(libVer: *mut self::libc::c_ulong,
                         _type: *const self::libc::c_char,
                         typeVer: *mut self::libc::c_ulong)
                         -> self::libc::c_int;
    pub fn virInitialize() -> self::libc::c_int;
    pub fn virConnectOpen(name: *const self::libc::c_char) -> virConnectPtr;
    pub fn virConnectOpenReadOnly(name: *const self::libc::c_char) -> virConnectPtr;
    pub fn virConnectOpenAuth(name: *const self::libc::c_char,
                              auth: virConnectAuthPtr,
                              flags: self::libc::c_uint)
                              -> virConnectPtr;
    pub fn virConnectRef(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectClose(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectGetType(conn: virConnectPtr) -> *const self::libc::c_char;
    pub fn virConnectGetVersion(conn: virConnectPtr,
                                hvVer: *mut self::libc::c_ulong)
                                -> self::libc::c_int;
    pub fn virConnectGetLibVersion(conn: virConnectPtr,
                                   libVer: *mut self::libc::c_ulong)
                                   -> self::libc::c_int;
    pub fn virConnectGetHostname(conn: virConnectPtr) -> *mut self::libc::c_char;
    pub fn virConnectGetURI(conn: virConnectPtr) -> *mut self::libc::c_char;
    pub fn virConnectGetSysinfo(conn: virConnectPtr,
                                flags: self::libc::c_uint)
                                -> *mut self::libc::c_char;
    pub fn virConnectSetKeepAlive(conn: virConnectPtr,
                                  interval: self::libc::c_int,
                                  count: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virConnectRegisterCloseCallback(conn: virConnectPtr,
                                           cb: virConnectCloseFunc,
                                           opaque: *mut self::libc::c_void,
                                           freecb: virFreeCallback)
                                           -> self::libc::c_int;
    pub fn virConnectUnregisterCloseCallback(conn: virConnectPtr,
                                             cb: virConnectCloseFunc)
                                             -> self::libc::c_int;
    pub fn virConnectGetMaxVcpus(conn: virConnectPtr,
                                 _type: *const self::libc::c_char)
                                 -> self::libc::c_int;
    pub fn virNodeGetInfo(conn: virConnectPtr, info: virNodeInfoPtr) -> self::libc::c_int;
    pub fn virConnectGetCapabilities(conn: virConnectPtr) -> *mut self::libc::c_char;
    pub fn virNodeGetCPUStats(conn: virConnectPtr,
                              cpuNum: self::libc::c_int,
                              params: virNodeCPUStatsPtr,
                              nparams: *mut self::libc::c_int,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virNodeGetMemoryStats(conn: virConnectPtr,
                                 cellNum: self::libc::c_int,
                                 params: virNodeMemoryStatsPtr,
                                 nparams: *mut self::libc::c_int,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virNodeGetFreeMemory(conn: virConnectPtr) -> self::libc::c_ulonglong;
    pub fn virNodeGetSecurityModel(conn: virConnectPtr,
                                   secmodel: virSecurityModelPtr)
                                   -> self::libc::c_int;
    pub fn virNodeSuspendForDuration(conn: virConnectPtr,
                                     target: self::libc::c_uint,
                                     duration: self::libc::c_ulonglong,
                                     flags: self::libc::c_uint)
                                     -> self::libc::c_int;
    pub fn virConnectListDomains(conn: virConnectPtr,
                                 ids: *mut self::libc::c_int,
                                 maxids: self::libc::c_int)
                                 -> self::libc::c_int;
    pub fn virConnectNumOfDomains(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virDomainGetConnect(domain: virDomainPtr) -> virConnectPtr;
    pub fn virDomainCreateXML(conn: virConnectPtr,
                              xmlDesc: *const self::libc::c_char,
                              flags: self::libc::c_uint)
                              -> virDomainPtr;
    pub fn virDomainCreateXMLWithFiles(conn: virConnectPtr,
                                       xmlDesc: *const self::libc::c_char,
                                       nfiles: self::libc::c_uint,
                                       files: *mut self::libc::c_int,
                                       flags: self::libc::c_uint)
                                       -> virDomainPtr;
    pub fn virDomainLookupByName(conn: virConnectPtr,
                                 name: *const self::libc::c_char)
                                 -> virDomainPtr;
    pub fn virDomainLookupByID(conn: virConnectPtr, id: self::libc::c_int) -> virDomainPtr;
    pub fn virDomainLookupByUUID(conn: virConnectPtr,
                                 uuid: *const self::libc::c_uchar)
                                 -> virDomainPtr;
    pub fn virDomainLookupByUUIDString(conn: virConnectPtr,
                                       uuid: *const self::libc::c_char)
                                       -> virDomainPtr;
    pub fn virDomainShutdown(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainShutdownFlags(domain: virDomainPtr,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virDomainReboot(domain: virDomainPtr, flags: self::libc::c_uint) -> self::libc::c_int;
    pub fn virDomainReset(domain: virDomainPtr, flags: self::libc::c_uint) -> self::libc::c_int;
    pub fn virDomainDestroy(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainDestroyFlags(domain: virDomainPtr,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virDomainRef(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainFree(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainSuspend(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainResume(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainPMSuspendForDuration(domain: virDomainPtr,
                                         target: self::libc::c_uint,
                                         duration: self::libc::c_ulonglong,
                                         flags: self::libc::c_uint)
                                         -> self::libc::c_int;
    pub fn virDomainPMWakeup(domain: virDomainPtr, flags: self::libc::c_uint) -> self::libc::c_int;
    pub fn virDomainSave(domain: virDomainPtr, to: *const self::libc::c_char) -> self::libc::c_int;
    pub fn virDomainSaveFlags(domain: virDomainPtr,
                              to: *const self::libc::c_char,
                              dxml: *const self::libc::c_char,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virDomainRestore(conn: virConnectPtr,
                            from: *const self::libc::c_char)
                            -> self::libc::c_int;
    pub fn virDomainRestoreFlags(conn: virConnectPtr,
                                 from: *const self::libc::c_char,
                                 dxml: *const self::libc::c_char,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virDomainSaveImageGetXMLDesc(conn: virConnectPtr,
                                        file: *const self::libc::c_char,
                                        flags: self::libc::c_uint)
                                        -> *mut self::libc::c_char;
    pub fn virDomainSaveImageDefineXML(conn: virConnectPtr,
                                       file: *const self::libc::c_char,
                                       dxml: *const self::libc::c_char,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainManagedSave(dom: virDomainPtr, flags: self::libc::c_uint) -> self::libc::c_int;
    pub fn virDomainHasManagedSaveImage(dom: virDomainPtr,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virDomainManagedSaveRemove(dom: virDomainPtr,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainCoreDump(domain: virDomainPtr,
                             to: *const self::libc::c_char,
                             flags: self::libc::c_uint)
                             -> self::libc::c_int;
    pub fn virDomainScreenshot(domain: virDomainPtr,
                               stream: virStreamPtr,
                               screen: self::libc::c_uint,
                               flags: self::libc::c_uint)
                               -> *mut self::libc::c_char;
    pub fn virDomainGetInfo(domain: virDomainPtr, info: virDomainInfoPtr) -> self::libc::c_int;
    pub fn virDomainGetState(domain: virDomainPtr,
                             state: *mut self::libc::c_int,
                             reason: *mut self::libc::c_int,
                             flags: self::libc::c_uint)
                             -> self::libc::c_int;
    pub fn virDomainGetCPUStats(domain: virDomainPtr,
                                params: virTypedParameterPtr,
                                nparams: self::libc::c_uint,
                                start_cpu: self::libc::c_int,
                                ncpus: self::libc::c_uint,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainGetControlInfo(domain: virDomainPtr,
                                   info: virDomainControlInfoPtr,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainGetSchedulerType(domain: virDomainPtr,
                                     nparams: *mut self::libc::c_int)
                                     -> *mut self::libc::c_char;
    pub fn virDomainSetBlkioParameters(domain: virDomainPtr,
                                       params: virTypedParameterPtr,
                                       nparams: self::libc::c_int,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainGetBlkioParameters(domain: virDomainPtr,
                                       params: virTypedParameterPtr,
                                       nparams: *mut self::libc::c_int,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainSetMemoryParameters(domain: virDomainPtr,
                                        params: virTypedParameterPtr,
                                        nparams: self::libc::c_int,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virDomainGetMemoryParameters(domain: virDomainPtr,
                                        params: virTypedParameterPtr,
                                        nparams: *mut self::libc::c_int,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virDomainSetNumaParameters(domain: virDomainPtr,
                                      params: virTypedParameterPtr,
                                      nparams: self::libc::c_int,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainGetNumaParameters(domain: virDomainPtr,
                                      params: virTypedParameterPtr,
                                      nparams: *mut self::libc::c_int,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainGetName(domain: virDomainPtr) -> *const self::libc::c_char;
    pub fn virDomainGetID(domain: virDomainPtr) -> self::libc::c_uint;
    pub fn virDomainGetUUID(domain: virDomainPtr,
                            uuid: *mut self::libc::c_uchar)
                            -> self::libc::c_int;
    pub fn virDomainGetUUIDString(domain: virDomainPtr,
                                  buf: *mut self::libc::c_char)
                                  -> self::libc::c_int;
    pub fn virDomainGetOSType(domain: virDomainPtr) -> *mut self::libc::c_char;
    pub fn virDomainGetMaxMemory(domain: virDomainPtr) -> self::libc::c_ulong;
    pub fn virDomainSetMaxMemory(domain: virDomainPtr,
                                 memory: self::libc::c_ulong)
                                 -> self::libc::c_int;
    pub fn virDomainSetMemory(domain: virDomainPtr,
                              memory: self::libc::c_ulong)
                              -> self::libc::c_int;
    pub fn virDomainSetMemoryFlags(domain: virDomainPtr,
                                   memory: self::libc::c_ulong,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainSetMemoryStatsPeriod(domain: virDomainPtr,
                                         period: self::libc::c_int,
                                         flags: self::libc::c_uint)
                                         -> self::libc::c_int;
    pub fn virDomainGetMaxVcpus(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainGetSecurityLabel(domain: virDomainPtr,
                                     seclabel: virSecurityLabelPtr)
                                     -> self::libc::c_int;
    pub fn virDomainGetHostname(domain: virDomainPtr,
                                flags: self::libc::c_uint)
                                -> *mut self::libc::c_char;
    pub fn virDomainGetSecurityLabelList(domain: virDomainPtr,
                                         seclabels: *mut virSecurityLabelPtr)
                                         -> self::libc::c_int;
    pub fn virDomainSetMetadata(domain: virDomainPtr,
                                _type: self::libc::c_int,
                                metadata: *const self::libc::c_char,
                                key: *const self::libc::c_char,
                                uri: *const self::libc::c_char,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainGetMetadata(domain: virDomainPtr,
                                _type: self::libc::c_int,
                                uri: *const self::libc::c_char,
                                flags: self::libc::c_uint)
                                -> *mut self::libc::c_char;
    pub fn virDomainGetXMLDesc(domain: virDomainPtr,
                               flags: self::libc::c_uint)
                               -> *mut self::libc::c_char;
    pub fn virConnectDomainXMLFromNative(conn: virConnectPtr,
                                         nativeFormat: *const self::libc::c_char,
                                         nativeConfig: *const self::libc::c_char,
                                         flags: self::libc::c_uint)
                                         -> *mut self::libc::c_char;
    pub fn virConnectDomainXMLToNative(conn: virConnectPtr,
                                       nativeFormat: *const self::libc::c_char,
                                       domainXml: *const self::libc::c_char,
                                       flags: self::libc::c_uint)
                                       -> *mut self::libc::c_char;
    pub fn virDomainBlockStats(dom: virDomainPtr,
                               disk: *const self::libc::c_char,
                               stats: virDomainBlockStatsPtr,
                               size: size_t)
                               -> self::libc::c_int;
    pub fn virDomainBlockStatsFlags(dom: virDomainPtr,
                                    disk: *const self::libc::c_char,
                                    params: virTypedParameterPtr,
                                    nparams: *mut self::libc::c_int,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virDomainInterfaceStats(dom: virDomainPtr,
                                   path: *const self::libc::c_char,
                                   stats: virDomainInterfaceStatsPtr,
                                   size: size_t)
                                   -> self::libc::c_int;
    pub fn virDomainSetInterfaceParameters(dom: virDomainPtr,
                                           device: *const self::libc::c_char,
                                           params: virTypedParameterPtr,
                                           nparams: self::libc::c_int,
                                           flags: self::libc::c_uint)
                                           -> self::libc::c_int;
    pub fn virDomainGetInterfaceParameters(dom: virDomainPtr,
                                           device: *const self::libc::c_char,
                                           params: virTypedParameterPtr,
                                           nparams: *mut self::libc::c_int,
                                           flags: self::libc::c_uint)
                                           -> self::libc::c_int;
    pub fn virDomainBlockPeek(dom: virDomainPtr,
                              disk: *const self::libc::c_char,
                              offset: self::libc::c_ulonglong,
                              size: size_t,
                              buffer: *mut self::libc::c_void,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virDomainBlockResize(dom: virDomainPtr,
                                disk: *const self::libc::c_char,
                                size: self::libc::c_ulonglong,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainGetBlockInfo(dom: virDomainPtr,
                                 disk: *const self::libc::c_char,
                                 info: virDomainBlockInfoPtr,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virDomainMemoryStats(dom: virDomainPtr,
                                stats: virDomainMemoryStatPtr,
                                nr_stats: self::libc::c_uint,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainMemoryPeek(dom: virDomainPtr,
                               start: self::libc::c_ulonglong,
                               size: size_t,
                               buffer: *mut self::libc::c_void,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virDomainDefineXML(conn: virConnectPtr, xml: *const self::libc::c_char) -> virDomainPtr;
    pub fn virDomainUndefine(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainUndefineFlags(domain: virDomainPtr,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virConnectNumOfDefinedDomains(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListDefinedDomains(conn: virConnectPtr,
                                        names: *mut *mut self::libc::c_char,
                                        maxnames: self::libc::c_int)
                                        -> self::libc::c_int;
    pub fn virConnectListAllDomains(conn: virConnectPtr,
                                    domains: *mut *mut virDomainPtr,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virDomainCreate(domain: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainCreateWithFlags(domain: virDomainPtr,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virDomainCreateWithFiles(domain: virDomainPtr,
                                    nfiles: self::libc::c_uint,
                                    files: *mut self::libc::c_int,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virDomainGetAutostart(domain: virDomainPtr,
                                 autostart: *mut self::libc::c_int)
                                 -> self::libc::c_int;
    pub fn virDomainSetAutostart(domain: virDomainPtr,
                                 autostart: self::libc::c_int)
                                 -> self::libc::c_int;
    pub fn virDomainSetVcpus(domain: virDomainPtr,
                             nvcpus: self::libc::c_uint)
                             -> self::libc::c_int;
    pub fn virDomainSetVcpusFlags(domain: virDomainPtr,
                                  nvcpus: self::libc::c_uint,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virDomainGetVcpusFlags(domain: virDomainPtr,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virDomainPinVcpu(domain: virDomainPtr,
                            vcpu: self::libc::c_uint,
                            cpumap: *mut self::libc::c_uchar,
                            maplen: self::libc::c_int)
                            -> self::libc::c_int;
    pub fn virDomainPinVcpuFlags(domain: virDomainPtr,
                                 vcpu: self::libc::c_uint,
                                 cpumap: *mut self::libc::c_uchar,
                                 maplen: self::libc::c_int,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virDomainGetVcpuPinInfo(domain: virDomainPtr,
                                   ncpumaps: self::libc::c_int,
                                   cpumaps: *mut self::libc::c_uchar,
                                   maplen: self::libc::c_int,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainPinEmulator(domain: virDomainPtr,
                                cpumap: *mut self::libc::c_uchar,
                                maplen: self::libc::c_int,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainGetEmulatorPinInfo(domain: virDomainPtr,
                                       cpumaps: *mut self::libc::c_uchar,
                                       maplen: self::libc::c_int,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainGetVcpus(domain: virDomainPtr,
                             info: virVcpuInfoPtr,
                             maxinfo: self::libc::c_int,
                             cpumaps: *mut self::libc::c_uchar,
                             maplen: self::libc::c_int)
                             -> self::libc::c_int;
    pub fn virDomainAttachDevice(domain: virDomainPtr,
                                 xml: *const self::libc::c_char)
                                 -> self::libc::c_int;
    pub fn virDomainDetachDevice(domain: virDomainPtr,
                                 xml: *const self::libc::c_char)
                                 -> self::libc::c_int;
    pub fn virDomainAttachDeviceFlags(domain: virDomainPtr,
                                      xml: *const self::libc::c_char,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainDetachDeviceFlags(domain: virDomainPtr,
                                      xml: *const self::libc::c_char,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainUpdateDeviceFlags(domain: virDomainPtr,
                                      xml: *const self::libc::c_char,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainBlockJobAbort(dom: virDomainPtr,
                                  disk: *const self::libc::c_char,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virDomainGetBlockJobInfo(dom: virDomainPtr,
                                    disk: *const self::libc::c_char,
                                    info: virDomainBlockJobInfoPtr,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virDomainBlockJobSetSpeed(dom: virDomainPtr,
                                     disk: *const self::libc::c_char,
                                     bandwidth: self::libc::c_ulong,
                                     flags: self::libc::c_uint)
                                     -> self::libc::c_int;
    pub fn virDomainBlockPull(dom: virDomainPtr,
                              disk: *const self::libc::c_char,
                              bandwidth: self::libc::c_ulong,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virDomainBlockRebase(dom: virDomainPtr,
                                disk: *const self::libc::c_char,
                                base: *const self::libc::c_char,
                                bandwidth: self::libc::c_ulong,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainBlockCommit(dom: virDomainPtr,
                                disk: *const self::libc::c_char,
                                base: *const self::libc::c_char,
                                top: *const self::libc::c_char,
                                bandwidth: self::libc::c_ulong,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainSetBlockIoTune(dom: virDomainPtr,
                                   disk: *const self::libc::c_char,
                                   params: virTypedParameterPtr,
                                   nparams: self::libc::c_int,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainGetBlockIoTune(dom: virDomainPtr,
                                   disk: *const self::libc::c_char,
                                   params: virTypedParameterPtr,
                                   nparams: *mut self::libc::c_int,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainGetDiskErrors(dom: virDomainPtr,
                                  errors: virDomainDiskErrorPtr,
                                  maxerrors: self::libc::c_uint,
                                  flags: self::libc::c_uint)
                                  -> self::libc::c_int;
    pub fn virNodeGetCellsFreeMemory(conn: virConnectPtr,
                                     freeMems: *mut self::libc::c_ulonglong,
                                     startCell: self::libc::c_int,
                                     maxCells: self::libc::c_int)
                                     -> self::libc::c_int;
    pub fn virNetworkGetConnect(network: virNetworkPtr) -> virConnectPtr;
    pub fn virConnectNumOfNetworks(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListNetworks(conn: virConnectPtr,
                                  names: *mut *mut self::libc::c_char,
                                  maxnames: self::libc::c_int)
                                  -> self::libc::c_int;
    pub fn virConnectNumOfDefinedNetworks(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListDefinedNetworks(conn: virConnectPtr,
                                         names: *mut *mut self::libc::c_char,
                                         maxnames: self::libc::c_int)
                                         -> self::libc::c_int;
    pub fn virConnectListAllNetworks(conn: virConnectPtr,
                                     nets: *mut *mut virNetworkPtr,
                                     flags: self::libc::c_uint)
                                     -> self::libc::c_int;
    pub fn virNetworkLookupByName(conn: virConnectPtr,
                                  name: *const self::libc::c_char)
                                  -> virNetworkPtr;
    pub fn virNetworkLookupByUUID(conn: virConnectPtr,
                                  uuid: *const self::libc::c_uchar)
                                  -> virNetworkPtr;
    pub fn virNetworkLookupByUUIDString(conn: virConnectPtr,
                                        uuid: *const self::libc::c_char)
                                        -> virNetworkPtr;
    pub fn virNetworkCreateXML(conn: virConnectPtr,
                               xmlDesc: *const self::libc::c_char)
                               -> virNetworkPtr;
    pub fn virNetworkDefineXML(conn: virConnectPtr,
                               xmlDesc: *const self::libc::c_char)
                               -> virNetworkPtr;
    pub fn virNetworkUndefine(network: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkUpdate(network: virNetworkPtr,
                            command: self::libc::c_uint,
                            section: self::libc::c_uint,
                            parentIndex: self::libc::c_int,
                            xml: *const self::libc::c_char,
                            flags: self::libc::c_uint)
                            -> self::libc::c_int;
    pub fn virNetworkCreate(network: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkDestroy(network: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkRef(network: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkFree(network: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkGetName(network: virNetworkPtr) -> *const self::libc::c_char;
    pub fn virNetworkGetUUID(network: virNetworkPtr,
                             uuid: *mut self::libc::c_uchar)
                             -> self::libc::c_int;
    pub fn virNetworkGetUUIDString(network: virNetworkPtr,
                                   buf: *mut self::libc::c_char)
                                   -> self::libc::c_int;
    pub fn virNetworkGetXMLDesc(network: virNetworkPtr,
                                flags: self::libc::c_uint)
                                -> *mut self::libc::c_char;
    pub fn virNetworkGetBridgeName(network: virNetworkPtr) -> *mut self::libc::c_char;
    pub fn virNetworkGetAutostart(network: virNetworkPtr,
                                  autostart: *mut self::libc::c_int)
                                  -> self::libc::c_int;
    pub fn virNetworkSetAutostart(network: virNetworkPtr,
                                  autostart: self::libc::c_int)
                                  -> self::libc::c_int;
    pub fn virInterfaceGetConnect(iface: virInterfacePtr) -> virConnectPtr;
    pub fn virConnectNumOfInterfaces(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListInterfaces(conn: virConnectPtr,
                                    names: *mut *mut self::libc::c_char,
                                    maxnames: self::libc::c_int)
                                    -> self::libc::c_int;
    pub fn virConnectNumOfDefinedInterfaces(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListDefinedInterfaces(conn: virConnectPtr,
                                           names: *mut *mut self::libc::c_char,
                                           maxnames: self::libc::c_int)
                                           -> self::libc::c_int;
    pub fn virConnectListAllInterfaces(conn: virConnectPtr,
                                       ifaces: *mut *mut virInterfacePtr,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virInterfaceLookupByName(conn: virConnectPtr,
                                    name: *const self::libc::c_char)
                                    -> virInterfacePtr;
    pub fn virInterfaceLookupByMACString(conn: virConnectPtr,
                                         mac: *const self::libc::c_char)
                                         -> virInterfacePtr;
    pub fn virInterfaceGetName(iface: virInterfacePtr) -> *const self::libc::c_char;
    pub fn virInterfaceGetMACString(iface: virInterfacePtr) -> *const self::libc::c_char;
    pub fn virInterfaceGetXMLDesc(iface: virInterfacePtr,
                                  flags: self::libc::c_uint)
                                  -> *mut self::libc::c_char;
    pub fn virInterfaceDefineXML(conn: virConnectPtr,
                                 xmlDesc: *const self::libc::c_char,
                                 flags: self::libc::c_uint)
                                 -> virInterfacePtr;
    pub fn virInterfaceUndefine(iface: virInterfacePtr) -> self::libc::c_int;
    pub fn virInterfaceCreate(iface: virInterfacePtr,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virInterfaceDestroy(iface: virInterfacePtr,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virInterfaceRef(iface: virInterfacePtr) -> self::libc::c_int;
    pub fn virInterfaceFree(iface: virInterfacePtr) -> self::libc::c_int;
    pub fn virInterfaceChangeBegin(conn: virConnectPtr,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virInterfaceChangeCommit(conn: virConnectPtr,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virInterfaceChangeRollback(conn: virConnectPtr,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virStoragePoolGetConnect(pool: virStoragePoolPtr) -> virConnectPtr;
    pub fn virConnectNumOfStoragePools(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListStoragePools(conn: virConnectPtr,
                                      names: *mut *mut self::libc::c_char,
                                      maxnames: self::libc::c_int)
                                      -> self::libc::c_int;
    pub fn virConnectNumOfDefinedStoragePools(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListDefinedStoragePools(conn: virConnectPtr,
                                             names: *mut *mut self::libc::c_char,
                                             maxnames: self::libc::c_int)
                                             -> self::libc::c_int;
    pub fn virConnectListAllStoragePools(conn: virConnectPtr,
                                         pools: *mut *mut virStoragePoolPtr,
                                         flags: self::libc::c_uint)
                                         -> self::libc::c_int;
    pub fn virConnectFindStoragePoolSources(conn: virConnectPtr,
                                            _type: *const self::libc::c_char,
                                            srcSpec: *const self::libc::c_char,
                                            flags: self::libc::c_uint)
                                            -> *mut self::libc::c_char;
    pub fn virStoragePoolLookupByName(conn: virConnectPtr,
                                      name: *const self::libc::c_char)
                                      -> virStoragePoolPtr;
    pub fn virStoragePoolLookupByUUID(conn: virConnectPtr,
                                      uuid: *const self::libc::c_uchar)
                                      -> virStoragePoolPtr;
    pub fn virStoragePoolLookupByUUIDString(conn: virConnectPtr,
                                            uuid: *const self::libc::c_char)
                                            -> virStoragePoolPtr;
    pub fn virStoragePoolLookupByVolume(vol: virStorageVolPtr) -> virStoragePoolPtr;
    pub fn virStoragePoolCreateXML(conn: virConnectPtr,
                                   xmlDesc: *const self::libc::c_char,
                                   flags: self::libc::c_uint)
                                   -> virStoragePoolPtr;
    pub fn virStoragePoolDefineXML(conn: virConnectPtr,
                                   xmlDesc: *const self::libc::c_char,
                                   flags: self::libc::c_uint)
                                   -> virStoragePoolPtr;
    pub fn virStoragePoolBuild(pool: virStoragePoolPtr,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virStoragePoolUndefine(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolCreate(pool: virStoragePoolPtr,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virStoragePoolDestroy(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolDelete(pool: virStoragePoolPtr,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virStoragePoolRef(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolFree(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolRefresh(pool: virStoragePoolPtr,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virStoragePoolGetName(pool: virStoragePoolPtr) -> *const self::libc::c_char;
    pub fn virStoragePoolGetUUID(pool: virStoragePoolPtr,
                                 uuid: *mut self::libc::c_uchar)
                                 -> self::libc::c_int;
    pub fn virStoragePoolGetUUIDString(pool: virStoragePoolPtr,
                                       buf: *mut self::libc::c_char)
                                       -> self::libc::c_int;
    pub fn virStoragePoolGetInfo(vol: virStoragePoolPtr,
                                 info: virStoragePoolInfoPtr)
                                 -> self::libc::c_int;
    pub fn virStoragePoolGetXMLDesc(pool: virStoragePoolPtr,
                                    flags: self::libc::c_uint)
                                    -> *mut self::libc::c_char;
    pub fn virStoragePoolGetAutostart(pool: virStoragePoolPtr,
                                      autostart: *mut self::libc::c_int)
                                      -> self::libc::c_int;
    pub fn virStoragePoolSetAutostart(pool: virStoragePoolPtr,
                                      autostart: self::libc::c_int)
                                      -> self::libc::c_int;
    pub fn virStoragePoolNumOfVolumes(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolListVolumes(pool: virStoragePoolPtr,
                                     names: *mut *mut self::libc::c_char,
                                     maxnames: self::libc::c_int)
                                     -> self::libc::c_int;
    pub fn virStoragePoolListAllVolumes(pool: virStoragePoolPtr,
                                        vols: *mut *mut virStorageVolPtr,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virStorageVolGetConnect(vol: virStorageVolPtr) -> virConnectPtr;
    pub fn virStorageVolLookupByName(pool: virStoragePoolPtr,
                                     name: *const self::libc::c_char)
                                     -> virStorageVolPtr;
    pub fn virStorageVolLookupByKey(conn: virConnectPtr,
                                    key: *const self::libc::c_char)
                                    -> virStorageVolPtr;
    pub fn virStorageVolLookupByPath(conn: virConnectPtr,
                                     path: *const self::libc::c_char)
                                     -> virStorageVolPtr;
    pub fn virStorageVolGetName(vol: virStorageVolPtr) -> *const self::libc::c_char;
    pub fn virStorageVolGetKey(vol: virStorageVolPtr) -> *const self::libc::c_char;
    pub fn virStorageVolCreateXML(pool: virStoragePoolPtr,
                                  xmldesc: *const self::libc::c_char,
                                  flags: self::libc::c_uint)
                                  -> virStorageVolPtr;
    pub fn virStorageVolCreateXMLFrom(pool: virStoragePoolPtr,
                                      xmldesc: *const self::libc::c_char,
                                      clonevol: virStorageVolPtr,
                                      flags: self::libc::c_uint)
                                      -> virStorageVolPtr;
    pub fn virStorageVolDownload(vol: virStorageVolPtr,
                                 stream: virStreamPtr,
                                 offset: self::libc::c_ulonglong,
                                 length: self::libc::c_ulonglong,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virStorageVolUpload(vol: virStorageVolPtr,
                               stream: virStreamPtr,
                               offset: self::libc::c_ulonglong,
                               length: self::libc::c_ulonglong,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virStorageVolDelete(vol: virStorageVolPtr,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virStorageVolWipe(vol: virStorageVolPtr,
                             flags: self::libc::c_uint)
                             -> self::libc::c_int;
    pub fn virStorageVolWipePattern(vol: virStorageVolPtr,
                                    algorithm: self::libc::c_uint,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virStorageVolRef(vol: virStorageVolPtr) -> self::libc::c_int;
    pub fn virStorageVolFree(vol: virStorageVolPtr) -> self::libc::c_int;
    pub fn virStorageVolGetInfo(vol: virStorageVolPtr,
                                info: virStorageVolInfoPtr)
                                -> self::libc::c_int;
    pub fn virStorageVolGetXMLDesc(pool: virStorageVolPtr,
                                   flags: self::libc::c_uint)
                                   -> *mut self::libc::c_char;
    pub fn virStorageVolGetPath(vol: virStorageVolPtr) -> *mut self::libc::c_char;
    pub fn virStorageVolResize(vol: virStorageVolPtr,
                               capacity: self::libc::c_ulonglong,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virDomainSendKey(domain: virDomainPtr,
                            codeset: self::libc::c_uint,
                            holdtime: self::libc::c_uint,
                            keycodes: *mut self::libc::c_uint,
                            nkeycodes: self::libc::c_int,
                            flags: self::libc::c_uint)
                            -> self::libc::c_int;
    pub fn virDomainSendProcessSignal(domain: virDomainPtr,
                                      pid_value: self::libc::c_longlong,
                                      signum: self::libc::c_uint,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainCreateLinux(conn: virConnectPtr,
                                xmlDesc: *const self::libc::c_char,
                                flags: self::libc::c_uint)
                                -> virDomainPtr;
    pub fn virNodeNumOfDevices(conn: virConnectPtr,
                               cap: *const self::libc::c_char,
                               flags: self::libc::c_uint)
                               -> self::libc::c_int;
    pub fn virNodeListDevices(conn: virConnectPtr,
                              cap: *const self::libc::c_char,
                              names: *mut *mut self::libc::c_char,
                              maxnames: self::libc::c_int,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virConnectListAllNodeDevices(conn: virConnectPtr,
                                        devices: *mut *mut virNodeDevicePtr,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virNodeDeviceLookupByName(conn: virConnectPtr,
                                     name: *const self::libc::c_char)
                                     -> virNodeDevicePtr;
    pub fn virNodeDeviceLookupSCSIHostByWWN(conn: virConnectPtr,
                                            wwnn: *const self::libc::c_char,
                                            wwpn: *const self::libc::c_char,
                                            flags: self::libc::c_uint)
                                            -> virNodeDevicePtr;
    pub fn virNodeDeviceGetName(dev: virNodeDevicePtr) -> *const self::libc::c_char;
    pub fn virNodeDeviceGetParent(dev: virNodeDevicePtr) -> *const self::libc::c_char;
    pub fn virNodeDeviceNumOfCaps(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceListCaps(dev: virNodeDevicePtr,
                                 names: *mut *mut self::libc::c_char,
                                 maxnames: self::libc::c_int)
                                 -> self::libc::c_int;
    pub fn virNodeDeviceGetXMLDesc(dev: virNodeDevicePtr,
                                   flags: self::libc::c_uint)
                                   -> *mut self::libc::c_char;
    pub fn virNodeDeviceRef(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceFree(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceDettach(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceDetachFlags(dev: virNodeDevicePtr,
                                    driverName: *const self::libc::c_char,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virNodeDeviceReAttach(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceReset(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virNodeDeviceCreateXML(conn: virConnectPtr,
                                  xmlDesc: *const self::libc::c_char,
                                  flags: self::libc::c_uint)
                                  -> virNodeDevicePtr;
    pub fn virNodeDeviceDestroy(dev: virNodeDevicePtr) -> self::libc::c_int;
    pub fn virConnectDomainEventRegister(conn: virConnectPtr,
                                         cb: virConnectDomainEventCallback,
                                         opaque: *mut self::libc::c_void,
                                         freecb: virFreeCallback)
                                         -> self::libc::c_int;
    pub fn virConnectDomainEventDeregister(conn: virConnectPtr,
                                           cb: virConnectDomainEventCallback)
                                           -> self::libc::c_int;
    pub fn virEventRegisterImpl(addHandle: virEventAddHandleFunc,
                                updateHandle: virEventUpdateHandleFunc,
                                removeHandle: virEventRemoveHandleFunc,
                                addTimeout: virEventAddTimeoutFunc,
                                updateTimeout: virEventUpdateTimeoutFunc,
                                removeTimeout: virEventRemoveTimeoutFunc)
                                -> ();
    pub fn virEventRegisterDefaultImpl() -> self::libc::c_int;
    pub fn virEventRunDefaultImpl() -> self::libc::c_int;
    pub fn virEventAddHandle(fd: self::libc::c_int,
                             events: self::libc::c_int,
                             cb: virEventHandleCallback,
                             opaque: *mut self::libc::c_void,
                             ff: virFreeCallback)
                             -> self::libc::c_int;
    pub fn virEventUpdateHandle(watch: self::libc::c_int, events: self::libc::c_int) -> ();
    pub fn virEventRemoveHandle(watch: self::libc::c_int) -> self::libc::c_int;
    pub fn virEventAddTimeout(frequency: self::libc::c_int,
                              cb: virEventTimeoutCallback,
                              opaque: *mut self::libc::c_void,
                              ff: virFreeCallback)
                              -> self::libc::c_int;
    pub fn virEventUpdateTimeout(timer: self::libc::c_int, frequency: self::libc::c_int) -> ();
    pub fn virEventRemoveTimeout(timer: self::libc::c_int) -> self::libc::c_int;
    pub fn virSecretGetConnect(secret: virSecretPtr) -> virConnectPtr;
    pub fn virConnectNumOfSecrets(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListSecrets(conn: virConnectPtr,
                                 uuids: *mut *mut self::libc::c_char,
                                 maxuuids: self::libc::c_int)
                                 -> self::libc::c_int;
    pub fn virConnectListAllSecrets(conn: virConnectPtr,
                                    secrets: *mut *mut virSecretPtr,
                                    flags: self::libc::c_uint)
                                    -> self::libc::c_int;
    pub fn virSecretLookupByUUID(conn: virConnectPtr,
                                 uuid: *const self::libc::c_uchar)
                                 -> virSecretPtr;
    pub fn virSecretLookupByUUIDString(conn: virConnectPtr,
                                       uuid: *const self::libc::c_char)
                                       -> virSecretPtr;
    pub fn virSecretLookupByUsage(conn: virConnectPtr,
                                  usageType: self::libc::c_int,
                                  usageID: *const self::libc::c_char)
                                  -> virSecretPtr;
    pub fn virSecretDefineXML(conn: virConnectPtr,
                              xml: *const self::libc::c_char,
                              flags: self::libc::c_uint)
                              -> virSecretPtr;
    pub fn virSecretGetUUID(secret: virSecretPtr,
                            buf: *mut self::libc::c_uchar)
                            -> self::libc::c_int;
    pub fn virSecretGetUUIDString(secret: virSecretPtr,
                                  buf: *mut self::libc::c_char)
                                  -> self::libc::c_int;
    pub fn virSecretGetUsageType(secret: virSecretPtr) -> self::libc::c_int;
    pub fn virSecretGetUsageID(secret: virSecretPtr) -> *const self::libc::c_char;
    pub fn virSecretGetXMLDesc(secret: virSecretPtr,
                               flags: self::libc::c_uint)
                               -> *mut self::libc::c_char;
    pub fn virSecretSetValue(secret: virSecretPtr,
                             value: *const self::libc::c_uchar,
                             value_size: size_t,
                             flags: self::libc::c_uint)
                             -> self::libc::c_int;
    pub fn virSecretGetValue(secret: virSecretPtr,
                             value_size: *mut size_t,
                             flags: self::libc::c_uint)
                             -> *mut self::libc::c_uchar;
    pub fn virSecretUndefine(secret: virSecretPtr) -> self::libc::c_int;
    pub fn virSecretRef(secret: virSecretPtr) -> self::libc::c_int;
    pub fn virSecretFree(secret: virSecretPtr) -> self::libc::c_int;
    pub fn virStreamNew(conn: virConnectPtr, flags: self::libc::c_uint) -> virStreamPtr;
    pub fn virStreamRef(st: virStreamPtr) -> self::libc::c_int;
    pub fn virStreamSend(st: virStreamPtr,
                         data: *const self::libc::c_char,
                         nbytes: size_t)
                         -> self::libc::c_int;
    pub fn virStreamRecv(st: virStreamPtr,
                         data: *mut self::libc::c_char,
                         nbytes: size_t)
                         -> self::libc::c_int;
    pub fn virStreamSendAll(st: virStreamPtr,
                            handler: virStreamSourceFunc,
                            opaque: *mut self::libc::c_void)
                            -> self::libc::c_int;
    pub fn virStreamRecvAll(st: virStreamPtr,
                            handler: virStreamSinkFunc,
                            opaque: *mut self::libc::c_void)
                            -> self::libc::c_int;
    pub fn virStreamEventAddCallback(stream: virStreamPtr,
                                     events: self::libc::c_int,
                                     cb: virStreamEventCallback,
                                     opaque: *mut self::libc::c_void,
                                     ff: virFreeCallback)
                                     -> self::libc::c_int;
    pub fn virStreamEventUpdateCallback(stream: virStreamPtr,
                                        events: self::libc::c_int)
                                        -> self::libc::c_int;
    pub fn virStreamEventRemoveCallback(stream: virStreamPtr) -> self::libc::c_int;
    pub fn virStreamFinish(st: virStreamPtr) -> self::libc::c_int;
    pub fn virStreamAbort(st: virStreamPtr) -> self::libc::c_int;
    pub fn virStreamFree(st: virStreamPtr) -> self::libc::c_int;
    pub fn virDomainIsActive(dom: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainIsPersistent(dom: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainIsUpdated(dom: virDomainPtr) -> self::libc::c_int;
    pub fn virNetworkIsActive(net: virNetworkPtr) -> self::libc::c_int;
    pub fn virNetworkIsPersistent(net: virNetworkPtr) -> self::libc::c_int;
    pub fn virStoragePoolIsActive(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virStoragePoolIsPersistent(pool: virStoragePoolPtr) -> self::libc::c_int;
    pub fn virInterfaceIsActive(iface: virInterfacePtr) -> self::libc::c_int;
    pub fn virConnectIsEncrypted(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectIsSecure(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectIsAlive(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectCompareCPU(conn: virConnectPtr,
                                xmlDesc: *const self::libc::c_char,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virConnectGetCPUModelNames(conn: virConnectPtr,
                                      arch: *const self::libc::c_char,
                                      models: *mut *mut *mut self::libc::c_char,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virConnectBaselineCPU(conn: virConnectPtr,
                                 xmlCPUs: *mut *const self::libc::c_char,
                                 ncpus: self::libc::c_uint,
                                 flags: self::libc::c_uint)
                                 -> *mut self::libc::c_char;
    pub fn virDomainGetJobInfo(dom: virDomainPtr, info: virDomainJobInfoPtr) -> self::libc::c_int;
    pub fn virDomainGetJobStats(domain: virDomainPtr,
                                _type: *mut self::libc::c_int,
                                params: *mut virTypedParameterPtr,
                                nparams: *mut self::libc::c_int,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainAbortJob(dom: virDomainPtr) -> self::libc::c_int;
    pub fn virDomainSnapshotGetName(snapshot: virDomainSnapshotPtr) -> *const self::libc::c_char;
    pub fn virDomainSnapshotGetDomain(snapshot: virDomainSnapshotPtr) -> virDomainPtr;
    pub fn virDomainSnapshotGetConnect(snapshot: virDomainSnapshotPtr) -> virConnectPtr;
    pub fn virDomainSnapshotCreateXML(domain: virDomainPtr,
                                      xmlDesc: *const self::libc::c_char,
                                      flags: self::libc::c_uint)
                                      -> virDomainSnapshotPtr;
    pub fn virDomainSnapshotGetXMLDesc(snapshot: virDomainSnapshotPtr,
                                       flags: self::libc::c_uint)
                                       -> *mut self::libc::c_char;
    pub fn virDomainSnapshotNum(domain: virDomainPtr,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainSnapshotListNames(domain: virDomainPtr,
                                      names: *mut *mut self::libc::c_char,
                                      nameslen: self::libc::c_int,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainListAllSnapshots(domain: virDomainPtr,
                                     snaps: *mut *mut virDomainSnapshotPtr,
                                     flags: self::libc::c_uint)
                                     -> self::libc::c_int;
    pub fn virDomainSnapshotNumChildren(snapshot: virDomainSnapshotPtr,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virDomainSnapshotListChildrenNames(snapshot: virDomainSnapshotPtr,
                                              names: *mut *mut self::libc::c_char,
                                              nameslen: self::libc::c_int,
                                              flags: self::libc::c_uint)
                                              -> self::libc::c_int;
    pub fn virDomainSnapshotListAllChildren(snapshot: virDomainSnapshotPtr,
                                            snaps: *mut *mut virDomainSnapshotPtr,
                                            flags: self::libc::c_uint)
                                            -> self::libc::c_int;
    pub fn virDomainSnapshotLookupByName(domain: virDomainPtr,
                                         name: *const self::libc::c_char,
                                         flags: self::libc::c_uint)
                                         -> virDomainSnapshotPtr;
    pub fn virDomainHasCurrentSnapshot(domain: virDomainPtr,
                                       flags: self::libc::c_uint)
                                       -> self::libc::c_int;
    pub fn virDomainSnapshotCurrent(domain: virDomainPtr,
                                    flags: self::libc::c_uint)
                                    -> virDomainSnapshotPtr;
    pub fn virDomainSnapshotGetParent(snapshot: virDomainSnapshotPtr,
                                      flags: self::libc::c_uint)
                                      -> virDomainSnapshotPtr;
    pub fn virDomainSnapshotIsCurrent(snapshot: virDomainSnapshotPtr,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virDomainSnapshotHasMetadata(snapshot: virDomainSnapshotPtr,
                                        flags: self::libc::c_uint)
                                        -> self::libc::c_int;
    pub fn virDomainRevertToSnapshot(snapshot: virDomainSnapshotPtr,
                                     flags: self::libc::c_uint)
                                     -> self::libc::c_int;
    pub fn virDomainSnapshotDelete(snapshot: virDomainSnapshotPtr,
                                   flags: self::libc::c_uint)
                                   -> self::libc::c_int;
    pub fn virDomainSnapshotRef(snapshot: virDomainSnapshotPtr) -> self::libc::c_int;
    pub fn virDomainSnapshotFree(snapshot: virDomainSnapshotPtr) -> self::libc::c_int;
    pub fn virConnectDomainEventRegisterAny(conn: virConnectPtr,
                                            dom: virDomainPtr,
                                            eventID: self::libc::c_int,
                                            cb: virConnectDomainEventGenericCallback,
                                            opaque: *mut self::libc::c_void,
                                            freecb: virFreeCallback)
                                            -> self::libc::c_int;
    pub fn virConnectDomainEventDeregisterAny(conn: virConnectPtr,
                                              callbackID: self::libc::c_int)
                                              -> self::libc::c_int;
    pub fn virConnectNetworkEventRegisterAny(conn: virConnectPtr,
                                             net: virNetworkPtr,
                                             eventID: self::libc::c_int,
                                             cb: virConnectNetworkEventGenericCallback,
                                             opaque: *mut self::libc::c_void,
                                             freecb: virFreeCallback)
                                             -> self::libc::c_int;
    pub fn virConnectNetworkEventDeregisterAny(conn: virConnectPtr,
                                               callbackID: self::libc::c_int)
                                               -> self::libc::c_int;
    pub fn virConnectNumOfNWFilters(conn: virConnectPtr) -> self::libc::c_int;
    pub fn virConnectListNWFilters(conn: virConnectPtr,
                                   names: *mut *mut self::libc::c_char,
                                   maxnames: self::libc::c_int)
                                   -> self::libc::c_int;
    pub fn virConnectListAllNWFilters(conn: virConnectPtr,
                                      filters: *mut *mut virNWFilterPtr,
                                      flags: self::libc::c_uint)
                                      -> self::libc::c_int;
    pub fn virNWFilterLookupByName(conn: virConnectPtr,
                                   name: *const self::libc::c_char)
                                   -> virNWFilterPtr;
    pub fn virNWFilterLookupByUUID(conn: virConnectPtr,
                                   uuid: *const self::libc::c_uchar)
                                   -> virNWFilterPtr;
    pub fn virNWFilterLookupByUUIDString(conn: virConnectPtr,
                                         uuid: *const self::libc::c_char)
                                         -> virNWFilterPtr;
    pub fn virNWFilterDefineXML(conn: virConnectPtr,
                                xmlDesc: *const self::libc::c_char)
                                -> virNWFilterPtr;
    pub fn virNWFilterUndefine(nwfilter: virNWFilterPtr) -> self::libc::c_int;
    pub fn virNWFilterRef(nwfilter: virNWFilterPtr) -> self::libc::c_int;
    pub fn virNWFilterFree(nwfilter: virNWFilterPtr) -> self::libc::c_int;
    pub fn virNWFilterGetName(nwfilter: virNWFilterPtr) -> *const self::libc::c_char;
    pub fn virNWFilterGetUUID(nwfilter: virNWFilterPtr,
                              uuid: *mut self::libc::c_uchar)
                              -> self::libc::c_int;
    pub fn virNWFilterGetUUIDString(nwfilter: virNWFilterPtr,
                                    buf: *mut self::libc::c_char)
                                    -> self::libc::c_int;
    pub fn virNWFilterGetXMLDesc(nwfilter: virNWFilterPtr,
                                 flags: self::libc::c_uint)
                                 -> *mut self::libc::c_char;
    pub fn virDomainOpenConsole(dom: virDomainPtr,
                                devname: *const self::libc::c_char,
                                st: virStreamPtr,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainOpenChannel(dom: virDomainPtr,
                                name: *const self::libc::c_char,
                                st: virStreamPtr,
                                flags: self::libc::c_uint)
                                -> self::libc::c_int;
    pub fn virDomainOpenGraphics(dom: virDomainPtr,
                                 idx: self::libc::c_uint,
                                 fd: self::libc::c_int,
                                 flags: self::libc::c_uint)
                                 -> self::libc::c_int;
    pub fn virDomainInjectNMI(domain: virDomainPtr,
                              flags: self::libc::c_uint)
                              -> self::libc::c_int;
    pub fn virDomainFSTrim(dom: virDomainPtr,
                           mountPoint: *const self::libc::c_char,
                           minimum: self::libc::c_ulonglong,
                           flags: self::libc::c_uint)
                           -> self::libc::c_int;
    //    pub const VIR_ERR_NONE: ::libc::c_uint = 0;
    //    pub const VIR_ERR_WARNING: ::libc::c_uint = 1;
    //    pub const VIR_ERR_ERROR: ::libc::c_uint = 2;



    pub fn virGetLastError() -> virErrorPtr;
    pub fn virSaveLastError() -> virErrorPtr;
    pub fn virResetLastError() -> ();
    pub fn virResetError(err: virErrorPtr) -> ();
    pub fn virFreeError(err: virErrorPtr) -> ();
    pub fn virGetLastErrorMessage() -> *const ::libc::c_char;
    pub fn virConnGetLastError(conn: virConnectPtr) -> virErrorPtr;
    pub fn virConnResetLastError(conn: virConnectPtr) -> ();
    pub fn virCopyLastError(to: virErrorPtr) -> ::libc::c_int;
    pub fn virDefaultErrorFunc(err: virErrorPtr) -> ();
    pub fn virSetErrorFunc(userData: *mut ::libc::c_void, handler: virErrorFunc) -> ();
    pub fn virConnSetErrorFunc(conn: virConnectPtr,
                               userData: *mut ::libc::c_void,
                               handler: virErrorFunc)
                               -> ();
    pub fn virConnCopyLastError(conn: virConnectPtr, to: virErrorPtr) -> ::libc::c_int;


}
