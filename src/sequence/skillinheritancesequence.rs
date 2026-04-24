#![allow(unused_imports)]

use unity2::system::collections::List;
use unity2::{Array, ClassIdentity};

use crate::gamedata::god::GodUnit;
use crate::gamedata::unit::Unit;
use crate::proc::{IProcInst, ProcDesc, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct SkillInheritanceSequence {}

#[unity2::methods]
impl SkillInheritanceSequence {
    #[method(name = "CreateBind", args = 1)]
    fn create_bind(super_: ProcInst);

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self);

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> Array<ProcDesc>;

    #[method(name = "get_SelectUnit")]
    fn select_unit(self) -> Unit;

    #[method(name = "set_SelectUnit")]
    fn set_select_unit(self, value: Unit);

    #[method(name = "get_SelectUnitGodList")]
    fn select_unit_god_list(self) -> List<GodUnit>;

    #[method(name = "set_SelectUnitGodList")]
    fn set_select_unit_god_list(self, value: List<GodUnit>);
}
