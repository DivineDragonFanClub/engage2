#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::engine::ui::Image;
use unity2::{ClassIdentity, Il2CppString};

use crate::gamedata::dispos::ChapterData;

#[unity2::class(namespace = "App")]
pub struct GmapSpot {}

#[unity2::methods]
impl GmapSpot {
    #[method(name = "get_Chapter", args = 0)]
    pub fn chapter(self) -> ChapterData;

    #[method(name = "get_GmapSpotID", args = 0)]
    pub fn gmap_spot_id(self) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
pub struct GmapMapInfoContent {
    #[rename(name = "m_MapInfoImage")]
    pub map_info_image: Image,
    #[rename(name = "m_MapInfoSprite")]
    pub map_info_sprite: Sprite,
}

#[unity2::methods]
impl GmapMapInfoContent {
    #[method(name = "SetMapInfo", args = 1)]
    pub fn set_map_info(self, gmap_spot: GmapSpot);
}
