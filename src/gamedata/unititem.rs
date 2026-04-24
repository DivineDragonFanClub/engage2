#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::gamedata::item::ItemData;

#[unity2::class(namespace = "App")]
pub struct UnitItem {
    #[rename(name = "m_Index")]
    pub index: i32,
    #[rename(name = "m_Item")]
    pub item: ItemData,
}

#[unity2::methods]
impl UnitItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor_with_item(self, item: ItemData);
}
