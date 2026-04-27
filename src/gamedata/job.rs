#![allow(unused_imports)]

use unity2::{Array, Class, ClassIdentity, Il2CppString};

use crate::{
    BattleStyleTypes,
    bit::{BitFieldCommon, IBitFieldCommon, BitField32,
        IBitField32, BitFieldTemplate32, IBitFieldTemplate32},
    SkillArray,
    SkillDataAttrs,
    WeaponLevelKind,
    WeaponMask,
    capability::{Capability, CapabilitySbyte}, 
    data::{IStructData, IStructDataMethods, StructData}
};

#[unity2::class(namespace = "App")]
#[parent(StructData<JobData>)]
pub struct JobData {
    #[backing]
    pub jid: Il2CppString,
    #[backing]
    pub name: Il2CppString,
    #[backing]
    pub aid: Il2CppString,
    #[backing]
    pub help: Il2CppString,
    #[backing(name = "UnitIconID_M")]
    pub unit_icon_id_m: Il2CppString,
    #[backing(name = "UnitIconID_F")]
    pub unit_icon_id_f: Il2CppString,
    #[backing(name = "UnitIconWeaponID")]
    pub unit_icon_weapon_id: Il2CppString,
    #[backing]
    pub rank: i32,
    #[backing]
    pub style_name: Il2CppString,
    #[backing]
    pub move_type: JobDataMoveTypes,
    #[backing]
    pub step_frame: i32,
    #[backing]
    pub max_level: u8,
    #[backing]
    pub internal_level: i8,
    #[backing]
    pub sort: u16,
    #[backing]
    pub flag: JobDataFlagField,
    #[backing(name = "CCItems")]
    pub cc_items: Array<Il2CppString>,
    #[backing]
    pub unique_items: Array<Il2CppString>,
    #[backing]
    pub style: BattleStyleTypes,
    pub weapons: Array<i8>,
    pub max_weapon_levels: Array<Il2CppString>,
    pub weapon_levels: Array<WeaponLevelKind>,
    pub weapon_level_plus_mask: WeaponMask,
    pub high_jobs: Array<Il2CppString>,
    #[backing]
    pub low_job: Il2CppString,
    #[backing]
    pub base: Capability,
    #[backing]
    pub limit: Capability,
    #[backing]
    pub base_grow: Capability,
    #[backing]
    pub diff_grow: CapabilitySbyte,
    #[backing]
    pub diff_grow_normal: CapabilitySbyte,
    #[backing]
    pub diff_grow_hard: CapabilitySbyte,
    #[backing]
    pub diff_grow_lunatic: CapabilitySbyte,
    #[backing]
    pub short_name: Il2CppString,
    #[backing]
    pub skills: Array<Il2CppString>,
    #[backing]
    pub learning_skill: Il2CppString,
    #[backing]
    pub lunatic_skill: Il2CppString,
    #[backing]
    pub attrs: SkillDataAttrs,
    #[backing]
    pub mask_skill: SkillArray,
}

impl JobData {
    pub const MAX_HIGH_JOB: i32 = 2;
}

#[unity2::methods]
impl JobData {
    #[method(name = "get_Jid")]
    fn jid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Aid")]
    fn aid(self) -> Il2CppString;

    #[method(name = "get_Help")]
    fn help(self) -> Il2CppString;

    #[method(name = "get_UnitIconID_M")]
    fn unit_icon_id_m(self) -> Il2CppString;

    #[method(name = "get_UnitIconID_F")]
    fn unit_icon_id_f(self) -> Il2CppString;

    #[method(name = "get_Rank")]
    fn rank(self) -> i8;

    #[method(name = "get_StyleName")]
    fn style_name(self) -> Il2CppString;
}

#[unity2::enumeration(namespace = "", name = "JobData.WeaponValues")]
#[repr(i32)]
pub enum JobDataWeaponValues {
    None = 0,
    Equippable = 1,
    Selectable1 = 2,
    Selectable2 = 3,
}

#[unity2::enumeration(namespace = "", name = "JobData.MoveTypes")]
#[repr(i32)]
pub enum JobDataMoveTypes {
    None = 0,
    Foot = 1,
    Horse = 2,
    Fly = 3,
    Dragon = 4,
    Pad = 5,
    Num = 6,
}

#[unity2::enumeration(namespace = "", name = "JobData.Ranks")]
#[repr(i32)]
pub enum JobDataRanks {
    Low = 0,
    High = 1,
}

#[unity2::class(namespace = "", name = "JobData.FlagField")]
#[parent(BitFieldTemplate32<JobDataFlags>)]
pub struct JobDataFlagField {}



bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct JobDataFlags: i32 {
        const CanCC = 1 << 0;
        const AnyoneCC = 1 << 1;
        const FemaleOnly = 1 << 2;
        const EncountMap = 1 << 3;
    }
}

impl ClassIdentity for JobDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "JobData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}
