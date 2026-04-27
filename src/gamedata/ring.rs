#![allow(unused_imports)]

use unity2::{Array, ClassIdentity, Il2CppString};
use unity2::engine::Color;

use crate::SkillArray;
use crate::{capability::CapabilitySbyte, data::{IStructData, IStructDataMethods, StructData}};

#[unity2::class(namespace = "App")]
#[parent(StructData<RingData>)]
pub struct RingData {
    #[backing] pub rnid: Il2CppString,
    #[backing] pub name: Il2CppString,
    #[backing] pub help: Il2CppString,
    #[backing] pub gid: Il2CppString,
    #[backing] pub ring_model: Il2CppString,
    #[backing] pub kind: RingDataKinds,
    #[backing] pub rank: RingDataRanks,
    #[backing] pub icon: Il2CppString,
    #[backing] pub enhance: CapabilitySbyte,
    #[backing] pub equip_sids: Array<Il2CppString>,
    #[backing] pub equip_skills: SkillArray,
    #[backing] pub is_single_rank: bool,
    #[backing] pub jewel_color_r: u8,
    #[backing] pub jewel_color_g: u8,
    #[backing] pub jewel_color_b: u8,
    #[backing] pub rim_color_r: u8,
    #[backing] pub rim_color_g: u8,
    #[backing] pub rim_color_b: u8,
    #[rename(name = "m_Group")] pub group: Il2CppString,
    #[rename(name = "m_FlagName")] pub flag_name: Il2CppString,
    #[rename(name = "m_JewelColor")] pub jewel_color: Color,
    #[rename(name = "m_RimColor")] pub rim_color: Color,
}

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

    #[method(name = "get_Icon", args = 0)]
    pub fn icon(self) -> Il2CppString;
}

#[unity2::enumeration(namespace = "", name = "RingData.Ranks")]
#[repr(i32)]
pub enum RingDataRanks {
    C = 0,
    B = 1,
    A = 2,
    S = 3,
    Max = 4,
}

#[unity2::enumeration(namespace = "", name = "RingData.Kinds")]
#[repr(i32)]
pub enum RingDataKinds {
    Common = 0,
    Emblem = 1,
    Color = 2,
    Num = 3,
}