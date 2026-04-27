#![allow(unused_imports)]

use unity2::{Array, ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<RelianceData>)]
pub struct RelianceData {
    #[backing] pub pid: Il2CppString,
    #[backing] pub exp_types: Array<u8>,
}

impl RelianceData {
    pub const NO_SUPPORT: u8 = 0;
    pub const MAX_EXP: i32 = 100;
    pub const MAX_DATA: i32 = 42;
}

#[unity2::methods]
impl RelianceData {
    #[method(name = "get_Pid")]
    fn pid(self) -> Il2CppString;

    #[method(name = "set_Pid")]
    fn set_pid(self, value: Il2CppString);

    #[method(name = "get_ExpTypes")]
    fn exp_types(self) -> Array<u8>;

    #[method(name = "set_ExpTypes")]
    fn set_exp_types(self, value: Array<u8>);
}

#[unity2::class(namespace = "App")]
#[parent(StructData<RelianceExpData>)]
pub struct RelianceExpData {}

#[unity2::methods]
impl RelianceExpData {
    #[method(name = "get_Rexid")]
    fn rexid(self) -> Il2CppString;

    #[method(name = "set_Rexid")]
    fn set_rexid(self, value: Il2CppString);

    #[method(name = "get_ExpC")]
    fn exp_c(self) -> u8;

    #[method(name = "set_ExpC")]
    fn set_exp_c(self, value: u8);

    #[method(name = "get_ExpB")]
    fn exp_b(self) -> u8;

    #[method(name = "set_ExpB")]
    fn set_exp_b(self, value: u8);

    #[method(name = "get_ExpA")]
    fn exp_a(self) -> u8;

    #[method(name = "set_ExpA")]
    fn set_exp_a(self, value: u8);
}

#[unity2::enumeration(namespace = "", name = "RelianceData.Level")]
#[repr(i32)]
pub enum RelianceDataLevel {
    None = 0,
    C = 1,
    B = 2,
    A = 3,
    APlus = 4,
    Num = 5,
}