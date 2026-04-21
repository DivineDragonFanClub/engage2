#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<RingData>)]
pub struct RingData {}

#[unity2::methods]
impl RingData {
    #[method(name = "get_Rnid")]
    fn rnid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Help")]
    fn help(self) -> Il2CppString;

    #[method(name = "get_Gid")]
    fn gid(self) -> Il2CppString;

    #[method(name = "get_RingModel")]
    fn ring_model(self) -> Il2CppString;
}
