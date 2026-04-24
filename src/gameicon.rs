#![allow(unused_imports)]

use unity2::engine::object::Sprite;
use unity2::{ClassIdentity, Il2CppString};

use crate::gamedata::{item::ItemData, ring::RingData, unit::Unit, GodData};
use crate::spriteatlasmanager::SpriteAtlasManager;

#[unity2::class(namespace = "App")]
pub struct GameIcon {
    #[static_field]
    #[rename(name = "s_Skill")]
    #[readonly]
    pub skill: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_Item")]
    #[readonly]
    pub item: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_Efficacy")]
    #[readonly]
    pub efficacy: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_EfficacyOutline")]
    #[readonly]
    pub efficacy_outline: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_ItemKinds")]
    #[readonly]
    pub item_kinds: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_ItemOutlineKinds")]
    #[readonly]
    pub item_kinds_outline: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_GodSymbol")]
    #[readonly]
    pub god_symbol: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_GodRing")]
    #[readonly]
    pub god_ring: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_System")]
    #[readonly]
    pub system: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_UnitIconIndex")]
    #[readonly]
    pub unit_icon_index: SpriteAtlasManager,
    #[static_field]
    #[rename(name = "s_UnitIconPallete")]
    #[readonly]
    pub unit_icon_pallete: SpriteAtlasManager,
}

#[unity2::methods]
impl GameIcon {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async();

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    #[method(name = "TryGetSystem", args = 1)]
    pub fn try_get_system(icon_name: Il2CppString) -> Sprite;

    #[method(name = "TryGetSkill", args = 1)]
    pub fn try_get_skill(skill_icon_name: Il2CppString) -> Sprite;

    #[method(name = "TryGetSkillEmpty", args = 0)]
    pub fn try_get_skill_empty() -> Sprite;

    #[method(name = "TryGetItem", args = 1)]
    pub fn try_get_item(icon_name: Il2CppString) -> Sprite;

    #[method(name = "TryGetGodSymbol", args = 1)]
    pub fn try_get_god_symbol(god_data: GodData) -> Sprite;

    #[method(offset = 0x227D2F0)]
    pub fn try_get_god_ring_from_unit(unit: Unit) -> Sprite;

    #[method(offset = 0x227D480)]
    pub fn try_get_god_ring_from_god(god_data: GodData) -> Sprite;

    #[method(offset = 0x227D4F0)]
    pub fn try_get_god_ring_from_ring(ring: RingData) -> Sprite;

    #[method(offset = 0x227CDD0)]
    pub fn try_get_item_from_data(item: crate::gamedata::item::ItemData) -> Sprite;

    #[method(name = "TyrGetUnitIconIndex", args = 1)]
    pub fn try_get_unit_icon_index(icon_name: Il2CppString) -> Sprite;

    #[method(name = "TyrGetUnitIconPallete", args = 1)]
    pub fn try_get_unit_icon_pallete(name: Il2CppString) -> Sprite;

    #[method(name = "ClearUnitIconCache", args = 0)]
    pub fn clear_unit_icon_cache();
}
