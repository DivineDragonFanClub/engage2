#![allow(unused_imports)]

use unity2::{Cast, ClassIdentity};

use crate::menu::{BasicMenuContent, IBasicMenuContent};

#[unity2::class(namespace = "App")]
#[parent(BasicMenuContent)]
pub struct ShopTopMenuContent {}

#[unity2::methods]
impl ShopTopMenuContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab();

    #[method(name = "Create", args = 0)]
    pub fn create() -> ShopTopMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: ShopTopMenuContent);
}
