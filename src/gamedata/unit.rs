#![allow(unused_imports)]

use unity2::{ClassIdentity, IlInstance, Il2CppString};

use super::god::GodUnit;
use super::item::ItemData;
use super::job::JobData;
use crate::gamedata::person::PersonData;

#[unity2::class(namespace = "App")]
pub struct Unit {
    #[rename(name = "m_Prev")]
    #[readonly]
    pub prev: Unit,
    #[rename(name = "m_Next")]
    #[readonly]
    pub next: Unit,
    #[rename(name = "m_LastPickVoice")]
    pub last_pick_voice: i8,
}

#[unity2::methods]
impl Unit {
    #[method(name = "get_Person")]
    pub fn person(self) -> PersonData;

    #[method(name = "get_Job")]
    pub fn job(self) -> JobData;

    #[method(name = "get_GodUnit")]
    pub fn god_unit(self) -> GodUnit;

    #[method(name = "get_Hp")]
    pub fn hp(self) -> i32;

    #[method(name = "get_Level")]
    pub fn level(self) -> u8;

    #[method(name = "get_Ident")]
    pub fn ident(self) -> i32;

    #[method(name = "get_IsEngaging", args = 0)]
    pub fn is_engaging(self) -> bool;

    #[method(name = "IsEngageOwner", args = 0)]
    pub fn is_engage_owner(self) -> bool;

    #[method(name = "ClassChange", args = 2)]
    pub fn class_change_raw(self, job: JobData, item: ItemData);

    #[method(name = "LearnJobSkill", args = 1)]
    pub fn learn_job_skill(self, job: JobData);

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, level: i32);

    #[method(offset = 0x1a3f400)]
    pub fn add_item(self, item: ItemData);

    #[method(name = "SetSelectedWeapon", args = 1)]
    pub fn set_selected_weapon(self, weapon_mask: crate::gamedata::weaponmask::WeaponMask);
}

impl Unit {
    /// 1-arg wrapper passing null for the optional ItemData slot
    pub fn class_change(self, job: JobData) {
        self.class_change_raw(job, ItemData::from_il_instance_null());
    }
}

impl ItemData {
    pub fn from_il_instance_null() -> ItemData {
        <ItemData as unity2::FromIlInstance>::from_il_instance(IlInstance::null())
    }
}
