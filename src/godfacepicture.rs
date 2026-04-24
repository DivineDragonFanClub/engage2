#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::{ClassIdentity, Il2CppString};

use crate::gamedata::{ring::RingData, GodData};
use crate::spriteatlasmanager::SpriteAtlasManager;

#[unity2::class(namespace = "App")]
pub struct GodFacePicture {
    #[static_field]
    #[rename(name = "s_FacePicture")]
    #[readonly]
    pub face_picture: SpriteAtlasManager,
}

#[unity2::methods]
impl GodFacePicture {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    #[method(name = "Get", args = 1)]
    pub fn get(god_data: GodData) -> Sprite;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path(god_data: GodData) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
pub struct BondsRingFacePicture {
    #[static_field]
    #[rename(name = "s_FacePicture")]
    #[readonly]
    pub face_picture: SpriteAtlasManager,
}

#[unity2::methods]
impl BondsRingFacePicture {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    #[method(name = "Get", args = 1)]
    pub fn get(ring_data: RingData) -> Sprite;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path(ring_data: RingData) -> Il2CppString;
}
