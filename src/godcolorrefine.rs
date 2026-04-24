#![allow(unused_imports)]

use unity2::engine::Color;
use unity2::ClassIdentity;

use crate::gamedata::GodData;

#[unity2::class(namespace = "App")]
pub struct GodColorRefineEmblem {}

#[unity2::methods]
impl GodColorRefineEmblem {
    #[method(name = "GetColor", args = 1)]
    pub fn get_color(self, god: GodData) -> Color;
}
