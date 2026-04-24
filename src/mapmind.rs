#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::data::{ISingletonClass, ISingletonClassMethods, SingletonClass};
use crate::gamedata::unit::Unit;

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<MapMind>)]
pub struct MapMind {
    #[rename(name = "m_UnitIndex")]
    pub unit_index: u8,
    #[rename(name = "m_X")]
    pub x: i8,
    #[rename(name = "m_Z")]
    pub z: i8,
    #[rename(name = "m_Mind")]
    pub mind: i32,
    #[rename(name = "m_AttackX")]
    pub attack_x: i8,
    #[rename(name = "m_AttackZ")]
    pub attack_z: i8,
    #[rename(name = "m_ItemIndex")]
    pub item_index: i8,
    #[rename(name = "m_TargetUnitIndex")]
    pub target_unit_index: u8,
    #[rename(name = "m_TargetX")]
    pub target_x: i8,
    #[rename(name = "m_TargetZ")]
    pub target_z: i8,
}

#[unity2::methods]
impl MapMind {
    #[method(name = "get_Unit", args = 0)]
    pub fn unit(self) -> Unit;

    #[method(name = "get_TargetUnit", args = 0)]
    pub fn target_unit(self) -> Unit;
}

impl MapMind {
    pub fn get_unit() -> Unit {
        Self::instance().unit()
    }

    pub fn get_target_unit() -> Unit {
        Self::instance().target_unit()
    }
}
