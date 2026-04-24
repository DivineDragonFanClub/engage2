#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::gamedata::item::ItemData;
use crate::gamedata::unititem::UnitItem;

#[unity2::class(namespace = "App")]
pub struct Transporter {}

#[unity2::methods]
impl Transporter {
    #[method(offset = 0x22A1B70)]
    pub fn add(unit_item: UnitItem) -> i32;
}
