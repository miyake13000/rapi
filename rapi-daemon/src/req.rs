use serde::{Deserialize, Serialize};
use std::ffi::c_int;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Request {
    pub req_type: ReqType,
    pub pid: c_int,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReqType {
    Initialize = 0,
    Finalize = 1,
    Stop = 2,
    Cont = 3,
    CommBegin = 4,
    CommEnd = 5,
    WaitBegin = 6,
    WaitEnd = 7,
}
