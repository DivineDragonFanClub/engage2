#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{ISingletonClass, ISingletonClassMethods, SingletonClass};
use crate::gamedata::dispos::ChapterData;
use crate::gamevariable::GameVariable;

#[unity2::enumeration]
#[repr(i32)]
pub enum GameMode {
    Casual = 0,
    Classic = 1,
    Phoenix = 2,
    Num = 3,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum Sequence {
    None = 0,
    ChapterSave = 1,
    Sortie = 2,
    Map = 3,
    Hub = 4,
    Kizuna = 5,
    Gmap = 6,
    Chapter = 7,
    Num = 8,
}

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<GameUserData>)]
pub struct GameUserData {}

#[unity2::methods]
impl GameUserData {
    #[method(name = "get_Variable")]
    fn get_variable(self) -> GameVariable;

    #[method(name = "get_Sequence")]
    fn get_sequence_raw(self) -> Sequence;

    #[method(name = "set_Sequence")]
    fn set_sequence_raw(self, value: Sequence);

    #[method(name = "GetGameMode")]
    fn get_game_mode_raw(self) -> GameMode;

    #[method(name = "SetGameMode")]
    fn set_game_mode_raw(self, mode: GameMode);

    #[method(name = "get_Chapter")]
    fn get_chapter_raw(self) -> ChapterData;

    #[method(name = "get_Gold")]
    fn get_gold_raw(self) -> i32;

    #[method(name = "set_Gold")]
    fn set_gold_raw(self, value: i32);
}

impl GameUserData {
    pub fn get_sequence() -> Sequence {
        GameUserData::instance().get_sequence_raw()
    }

    pub fn set_sequence(value: Sequence) {
        GameUserData::instance().set_sequence_raw(value);
    }

    pub fn get_game_mode() -> GameMode {
        GameUserData::instance().get_game_mode_raw()
    }

    pub fn set_game_mode(mode: GameMode) {
        GameUserData::instance().set_game_mode_raw(mode);
    }

    pub fn get_chapter() -> ChapterData {
        GameUserData::instance().get_chapter_raw()
    }

    pub fn get_gold() -> i32 {
        GameUserData::instance().get_gold_raw()
    }

    pub fn set_gold(value: i32) {
        GameUserData::instance().set_gold_raw(value);
    }
}
