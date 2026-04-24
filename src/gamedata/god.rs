#![allow(unused_imports)]

use unity2::system::collections::List;
use unity2::{Array, ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};
use crate::proc::ProcInst;

#[unity2::class(namespace = "App")]
#[parent(StructData<GodData>)]
pub struct GodData {}

#[unity2::methods]
impl GodData {
    #[method(name = "get_Gid")]
    fn gid(self) -> Il2CppString;

    #[method(name = "get_Mid")]
    fn mid(self) -> Il2CppString;

    #[method(name = "get_Nickname")]
    fn nickname(self) -> Il2CppString;

    #[method(name = "get_AsciiName")]
    fn ascii_name(self) -> Il2CppString;

    #[method(name = "get_FaceIconName")]
    fn face_icon_name(self) -> Il2CppString;

    #[method(name = "get_FaceIconNameDarkness")]
    fn face_icon_name_darkness(self) -> Il2CppString;

    #[method(name = "get_MainData")]
    fn main_data(self) -> GodData;
}

#[unity2::class(namespace = "App")]
pub struct GodUnit {}

#[unity2::methods]
impl GodUnit {
    #[method(name = "get_Data")]
    fn data(self) -> GodData;

    #[method(name = "get_ActualData")]
    fn actual_data(self) -> GodData;

    #[method(name = "get_Gid")]
    fn gid(self) -> Il2CppString;

    #[method(name = "get_ActualGid")]
    fn actual_gid(self) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
pub struct GodPool {}

#[unity2::methods]
impl GodPool {
    #[method(name = "TryGet", args = 2)]
    fn try_get_by_gid_raw(gid: Il2CppString, include_reserved: bool) -> GodUnit;

    #[method(offset = 0x2334600)]
    fn try_get_by_data_raw(data: GodData, include_reserved: bool) -> GodUnit;

    #[method(name = "Create", args = 1)]
    fn create_raw(data: GodData) -> GodUnit;

    #[method(name = "Delete", args = 1)]
    fn delete_raw(god: GodUnit);

    #[method(name = "HasArmlet", args = 0)]
    pub fn has_armlet() -> bool;
}

impl GodPool {
    pub fn try_get_by_gid(gid: impl Into<Il2CppString>, include_reserved: bool) -> Option<GodUnit> {
        let unit = Self::try_get_by_gid_raw(gid.into(), include_reserved);
        (!unity2::Cast::is_null(unit)).then_some(unit)
    }

    pub fn try_get_by_data(data: GodData, include_reserved: bool) -> Option<GodUnit> {
        let unit = Self::try_get_by_data_raw(data, include_reserved);
        (!unity2::Cast::is_null(unit)).then_some(unit)
    }

    pub fn create(data: GodData) -> Option<GodUnit> {
        let unit = Self::create_raw(data);
        (!unity2::Cast::is_null(unit)).then_some(unit)
    }

    pub fn delete(god: GodUnit) {
        Self::delete_raw(god);
    }
}
