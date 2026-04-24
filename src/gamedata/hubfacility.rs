#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<HubFacilityData>)]
pub struct HubFacilityData {
    #[backing]
    pub aid: Il2CppString,
    #[backing]
    pub mid: Il2CppString,
    #[backing]
    pub condition_cid: Il2CppString,
    #[backing]
    pub icon_name: Il2CppString,
}

#[unity2::methods]
impl HubFacilityData {
    #[method(name = "IsComplete", args = 0)]
    pub fn is_complete(self) -> bool;

    #[method(name = "GetPrefixlessAid", args = 0)]
    pub fn get_prefixless_aid(self) -> Il2CppString;
}
