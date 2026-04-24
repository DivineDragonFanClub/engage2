#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::system::collections::Dictionary;
use unity2::{ClassIdentity, Il2CppString};

use crate::data::PersonData;
use crate::gamedata::{GodData, ring::RingData, unit::Unit};

#[unity2::class(namespace = "App")]
pub struct SpriteAtlasManager {
    #[rename(name = "m_CacheTable")]
    #[readonly]
    pub cache_table: Dictionary<Il2CppString, Sprite>,
}

#[unity2::methods]
impl SpriteAtlasManager {
    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, path: Il2CppString);

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded(self) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self);

    #[method(name = "Get", args = 1)]
    pub fn get(self, name: Il2CppString) -> Sprite;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(self, name: Il2CppString) -> Sprite;

    #[method(name = "ClearCache", args = 0)]
    pub fn clear_cache(self);
}

#[unity2::class(namespace = "App")]
pub struct FaceThumbnail {
    #[static_field]
    #[rename(name = "s_FaceThumb")]
    #[readonly]
    pub face_thumb: SpriteAtlasManager,
}

#[unity2::methods]
impl FaceThumbnail {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async();

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    // Get(Unit)
    #[method(offset = 0x2D51CB0)]
    pub fn get_from_unit(unit: Unit) -> Sprite;

    #[method(offset = 0x2D52090)]
    pub fn get_from_person(person: PersonData) -> Sprite;

    #[method(offset = 0x2D52270)]
    pub fn get_from_god(god: GodData) -> Sprite;

    #[method(offset = 0x2D52620)]
    pub fn get_from_ring(ring: RingData) -> Sprite;

    #[method(name = "GetDLCGodPath", args = 1)]
    pub fn get_dlc_god_path(god_ascii_name: Il2CppString) -> Il2CppString;
}
