#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::data::{ISingletonClass, SingletonClass};

#[unity2::enumeration]
#[repr(i32)]
pub enum GameConfigMode {
    Quality = 0,
    Performance = 1,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum GameSpeed {
    Slow = 0,
    Normal = 1,
    Fast = 2,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum GameToggle {
    Off = 0,
    On = 1,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum AnimeType {
    Off = 0,
    On = 1,
    PlayerUnit = 2,
    PlayerTurn = 3,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum MinimapType {
    Large = 0,
    Small = 1,
    Off = 2,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum EngageAnimeType {
    Off = 0,
    Once = 1,
    On = 2,
}

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<GameConfig>)]
pub struct GameConfig {
    #[backing] pub mode: GameConfigMode,
    #[backing] pub map_grid_alpha: f32,
    #[backing] pub battle_scene_type: AnimeType,
    #[backing] pub support_scene_type: AnimeType,
    #[backing] pub battle_camera_reverse_horizontal: bool,
    #[backing] pub battle_camera_reverse_vertical: bool,
    #[backing] pub hub_camera_reverse_horizontal: bool,
    #[backing] pub hub_camera_reverse_vertical: bool,
    #[backing] pub game_speed: GameSpeed,
    #[backing] pub map_minimap: MinimapType,
    #[backing] pub map_unit_gauge: GameToggle,
    #[backing] pub engage_anim: EngageAnimeType,
    #[backing(name = "AISkip")] pub ai_skip: GameToggle,
    #[backing] pub tutorial_show: GameToggle,
    #[backing] pub volume_bgm: f32,
    #[backing] pub volume_se: f32,
    #[backing] pub volume_env: f32,
    #[backing] pub volume_voice: f32,
    #[backing(name = "IsEnableNetwork")] pub is_enable_network: bool,
}
