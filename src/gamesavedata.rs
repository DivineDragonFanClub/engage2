#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::gameuserdata::GameMode;

#[unity2::class(namespace = "App")]
pub struct GameSaveDataHeader {
    #[rename(name = "m_GameVersion")]
    pub game_version: i32,
    #[rename(name = "m_GameMode")]
    pub gamemode: GameMode,
    #[rename(name = "m_Turn")]
    pub turn: i32,
    #[rename(name = "m_PlayTime")]
    pub play_time: f32,
}

#[unity2::class(namespace = "App", name = "GameSaveDataHeaderReader.Handle")]
pub struct GameSaveDataHeaderReaderHandle {
    #[rename(name = "<Type>k__BackingField")]
    pub ty: i32,
    #[rename(name = "<Index>k__BackingField")]
    pub index: i32,
    #[rename(name = "<Header>k__BackingField")]
    pub header: GameSaveDataHeader,
}

#[unity2::methods]
impl GameSaveDataHeaderReaderHandle {
    #[method(name = "IsRunning", args = 0)]
    pub fn is_running(self) -> bool;

    #[method(name = "IsSucceeded", args = 0)]
    pub fn is_succeeded(self) -> bool;
}

#[unity2::class(namespace = "App")]
pub struct GameSaveData {
    #[backing(name = "Type")]
    pub ty: i32,
    #[backing(name = "Index")]
    pub index: i32,
    #[backing(name = "FromType")]
    pub from_type: i32,
    #[backing(name = "FromIndex")]
    pub from_index: i32,
    #[backing(name = "Header")]
    pub header: GameSaveDataHeader,
    #[backing(name = "IsSuccess")]
    pub is_success: bool,
    #[backing(name = "IsExcludeHeaderAndTime")]
    pub is_exclude_header_and_time: bool,
}

#[unity2::class(namespace = "App", name = "GameSaveData.ProcBase")]
pub struct GameSaveDataProcBase {
    #[rename(name = "m_SaveData")]
    pub save_data: GameSaveData,
}

#[unity2::class(namespace = "App", name = "SaveDataMenu.MenuItem")]
pub struct SaveDataMenuMenuItem {
    #[rename(name = "m_SaveDataHeaderHandle")]
    pub save_data_header_handle: GameSaveDataHeaderReaderHandle,
    #[rename(name = "m_IsSelected")]
    pub is_selected: bool,
}

#[unity2::class(namespace = "App", name = "SaveDataMenu.MenuItemContent")]
pub struct SaveDataMenuItemContent {
    #[rename(name = "m_ModeText")]
    pub mode_text: unity2::engine::TextMeshProUGUI,
    #[rename(name = "m_GameModeImage")]
    pub game_mode_image: unity2::engine::Image,
}
