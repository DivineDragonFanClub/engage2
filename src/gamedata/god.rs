#![allow(unused_imports)]

use unity2::system::Dictionary;
use unity2::system::collections::List;
use unity2::{Array, Class, ClassIdentity, Il2CppString};

use crate::{ItemDataKinds, Unit};
use crate::bit::{
    BitFieldCommon, BitField32, BitFieldTemplate32,
    IBitFieldCommon, IBitField32, IBitFieldTemplate32,
};
use crate::capability::CapabilitySbyte;
use crate::data::{IStructData, IStructDataMethods, StructData};
use crate::proc::ProcInst;
use crate::unitpool::ForceType;

#[unity2::class(namespace = "App")]
#[parent(StructData<GodData>)]
pub struct GodData {
    #[backing] pub gid: Il2CppString,
    #[backing] pub mid: Il2CppString,
    #[backing] pub nickname: Il2CppString,
    #[backing] pub help: Il2CppString,
    #[backing(name = "SoundID")] pub sound_id: Il2CppString,
    #[backing(name = "AssetID")] pub asset_id: Il2CppString,
    #[backing] pub face_icon_name: Il2CppString,
    #[backing] pub face_icon_name_darkness: Il2CppString,
    #[backing] pub ringname: Il2CppString,
    #[backing] pub ringhelp: Il2CppString,
    #[backing(name = "UnitIconID")] pub unit_icon_id: Il2CppString,
    #[backing] pub change: Array<Il2CppString>,
    #[backing] pub link: Il2CppString,
    #[backing] pub engage_haunt: Il2CppString,
    #[backing] pub level: i32,
    #[backing] pub force_type: ForceType,
    #[backing] pub female: i8,
    #[backing] pub good_weapon: ItemDataKinds,
    #[backing] pub sort: i16,
    #[backing] pub engage_count: u8,
    #[backing] pub engage_attack: Il2CppString,
    #[backing] pub engage_attack_rampage: Il2CppString,
    #[backing] pub engage_attack_link: Il2CppString,
    #[backing] pub link_gid: Il2CppString,
    #[backing] pub gbid: Il2CppString,
    #[backing] pub grow_table: Il2CppString,
    #[backing] pub level_cap: u8,
    #[backing] pub unlock_level_cap_var_name: Il2CppString,
    #[backing] pub engrave_word: Il2CppString,
    #[backing] pub engrave_power: i8,
    #[backing] pub engrave_weight: i8,
    #[backing] pub engrave_hit: i8,
    #[backing] pub engrave_critical: i8,
    #[backing] pub engrave_avoid: i8,
    #[backing] pub engrave_secure: i8,
    #[backing] pub synchro_enhance: CapabilitySbyte,
    #[backing] pub main_data: GodData,
    #[backing] pub change_data: Array<GodData>,
    #[backing] pub ascii_name: Il2CppString,
    #[backing] pub flag: GodDataFlagField,
    #[backing] pub net_ranking_index: u8,
    #[backing] pub hero_face_icon_name: Il2CppString,
    #[backing] pub heroine_face_icon_name: Il2CppString,
    #[backing(name = "AIEngageAttackType")]
    pub ai_engage_attack_type: GodDataAIEngageAttackTypes,
    #[rename(name = "m_EngageHauntUnit")]
    pub engage_haunt_unit: Unit,

    #[static_field]
    #[rename(name = "s_LinkDics")]
    pub link_dics: Dictionary<Il2CppString, GodData>,
}

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

#[unity2::enumeration(namespace = "", name = "GodData.AIEngageAttackType")]
#[repr(i32)]
pub enum GodDataAIEngageAttackTypes {
    None = 0,
    Attack = 1,
    AttackPierce = 2,
    AttackCharge = 3,
    Heal = 4,
    Dance = 5,
    Bless = 6,
    AttackWait = 7,
    Overlap = 8,
    Summon = 9,
}

#[unity2::class(namespace = "", name = "GodData.FlagField")]
#[parent(BitFieldTemplate32<GodDataFlags>)]
pub struct GodDataFlagField {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct GodDataFlags: i32 {
        const NoAddExp = 1 << 0;
        const EnableRingList = 1 << 1;
        const UnitIconDarkness = 1 << 2;
        const GaugeDarkness = 1 << 3;
        const OnlyEngageWeapon = 1 << 4;
        const Armlet = 1 << 5;
        const Hero = -2147483648; // i32::MIN
    }
}

impl ClassIdentity for GodDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "GodData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "GodData.RelianceLevel")]
#[repr(i32)]
pub enum GodDataRelianceLevel {
    D = 0,
    C = 1,
    B = 2,
    A = 3,
    S = 4,
}