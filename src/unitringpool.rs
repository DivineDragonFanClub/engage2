#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::gamedata::unit::Unit;

#[unity2::class(namespace = "App")]
pub struct UnitRingPool {}

#[unity2::methods]
impl UnitRingPool {
    #[method(offset = 0x01c5d420)]
    pub fn add(rnid: Il2CppString, owner: Unit, stock_count: i32) -> unity2::IlInstance;
}
