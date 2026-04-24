#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString, IlInstance};

use crate::gamedata::unit::Unit;
use crate::menu::{BasicMenu, IBasicMenu, IBasicMenuMethods};
use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(BasicMenu)]
pub struct GodRoomUnitSelectMenu {
    #[static_field]
    #[rename(name = "s_ForceMask")]
    pub s_force_mask: u32,
    #[static_field]
    #[rename(name = "s_ScrollIndex")]
    pub s_scroll_index: i32,
}

#[unity2::class(namespace = "App", name = "GodRoomUnitSelectMenu.GodRoomUnitSelectMenuItem")]
pub struct GodRoomUnitSelectMenuItem {
    #[rename(name = "m_Index")]
    #[readonly]
    pub index: i32,
    #[rename(name = "m_Unit")]
    #[readonly]
    pub unit: Unit,
    #[rename(name = "m_DecideEventHandler")]
    pub decide_event_handler: IlInstance,
}

#[unity2::methods]
impl GodRoomUnitSelectMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, index: i32, unit: Unit, decide_event_handler: IlInstance);
}
