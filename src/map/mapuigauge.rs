#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::engine::u2d::SpriteAtlas;
use unity2::system::collections::Dictionary;
use unity2::{Array, ClassIdentity, Il2CppString};

#[unity2::class(namespace = "")]
pub struct MapUIGauge {
    #[static_field]
    #[rename(name = "IconNames")]
    pub icon_names: Array<Il2CppString>,

    #[rename(name = "m_SpriteAtlas")]
    pub sprite_atlas: SpriteAtlas,

    #[rename(name = "m_Sprites")]
    pub sprites: Array<Sprite>,

    #[rename(name = "m_Dictionary")]
    pub dictionary: Dictionary<Il2CppString, Sprite>,
}

#[unity2::methods]
impl MapUIGauge {
    #[method(name = "GetSprite", args = 1)]
    pub fn get_sprite_by_index(self, index: i32) -> Sprite;
}
