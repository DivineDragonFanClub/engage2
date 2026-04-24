#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "")]
#[parent(ProcInst)]
pub struct ConfigSequence {}

#[unity2::methods]
impl ConfigSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: ProcInst);
}
