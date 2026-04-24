#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::{ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App")]
pub struct UnitIcon {
    #[rename(name = "m_IconName")]
    pub icon_name: Il2CppString,
    #[rename(name = "m_PalleteName")]
    pub pallete_name: Il2CppString,
    #[rename(name = "m_Brightness")]
    pub brightness: f32,
    #[rename(name = "m_PalleteSprite")]
    pub pallete_sprite: unity2::engine::object::Sprite,
}

#[unity2::methods]
impl UnitIcon {
    #[method(name = "TrySet", args = 2)]
    pub fn try_set(self, index_name: Il2CppString, palette_name: Il2CppString) -> bool;

    #[method(name = "UpdateIcon", args = 0)]
    pub fn update_icon(self);

    #[method(name = "ConvertPersonIconID", args = 2)]
    pub fn convert_person_icon_id(original: Il2CppString, is_female: bool) -> Il2CppString;

    #[method(name = "set_sprite", args = 1)]
    pub fn set_sprite(self, value: unity2::engine::object::Sprite);

    #[method(name = "SetAllDirty", args = 0)]
    pub fn set_all_dirty(self);

    #[method(name = "SetVerticesDirty", args = 0)]
    pub fn set_vertices_dirty(self);
}
