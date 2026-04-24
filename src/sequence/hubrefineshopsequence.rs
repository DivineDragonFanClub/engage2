#![allow(unused_imports)]

use unity2::{Array, ClassIdentity};

use crate::proc::{IProcInst, ProcDesc, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct HubRefineShopSequence {}

#[unity2::methods]
impl HubRefineShopSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self);

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> Array<ProcDesc>;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind_static(parent: ProcInst);
}
