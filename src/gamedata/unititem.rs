#![allow(unused_imports)]

use unity2::{Array, Class, ClassIdentity, Il2CppString};

use crate::gamedata::god::GodData;
use crate::gamedata::god::GodUnit;
use crate::gamedata::item::ItemData;

#[unity2::class(namespace = "App")]
pub struct UnitItem {
    #[static_field]
    #[rename(name = "s_EnchantHash")]
    pub enchant_hash: i32,

    #[rename(name = "m_Index")]
    pub index: i32,
    #[rename(name = "m_Item")]
    pub item: ItemData,
    #[rename(name = "m_Endurance")]
    pub endurance: u8,
    #[rename(name = "m_RefineLevel")]
    pub refine_level: u8,
    #[rename(name = "m_Flags")]
    pub flags: UnitItemFlags,
    #[rename(name = "m_Engrave")]
    pub god_data: GodData,
    #[rename(name = "m_GodUnit")]
    pub god_unit: GodUnit,
}

#[unity2::methods]
impl UnitItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor_with_item(self, item: ItemData);
}

#[unity2::class(namespace = "App")]
pub struct WeaponLevels {
    #[rename(name = "m_Levels")]
    pub levels: Array<i8>,
}

#[unity2::class(namespace = "App")]
pub struct WeaponLevel {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct UnitItemFlags: i32 {
        const Equipped = 1 << 0;
        const Drop = 1 << 1;
        const SkipLog = 1 << 2;
    }
}

impl ClassIdentity for UnitItemFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "UnitItem.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "WeaponLevel.Kind")]
#[repr(i32)]
pub enum WeaponLevelKind {
    None = 0,
    D = 1,
    C = 2,
    B = 3,
    A = 4,
    S = 5,
}