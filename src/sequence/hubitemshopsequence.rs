#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct HubItemShopSequence {}

#[unity2::methods]
impl HubItemShopSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(parent: ProcInst);
}

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct HubWeaponShopSequence {}

#[unity2::methods]
impl HubWeaponShopSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(parent: ProcInst);
}
