#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::ISingletonClassMethods;
use crate::gameuserdata::{GameUserData, IGameUserDataMethods};

#[unity2::class(namespace = "App")]
pub struct GameVariable {}

#[unity2::methods]
impl GameVariable {
    #[method(name = "GetBool")]
    fn get_bool(self, key: Il2CppString) -> bool;

    #[method(name = "SetBool")]
    fn set_bool(self, key: Il2CppString, enable: bool);

    #[method(name = "GetNumber")]
    fn get_number(self, key: Il2CppString) -> i32;

    #[method(name = "SetNumber")]
    fn set_number(self, key: Il2CppString, value: i32);

    #[method(name = "GetString")]
    fn get_string(self, key: Il2CppString) -> Il2CppString;

    #[method(name = "SetString")]
    fn set_string(self, key: Il2CppString, value: Il2CppString);

    #[method(name = "Entry", args = 2)]
    fn entry(self, key: Il2CppString, num: i32) -> bool;

    #[method(name = "EntryNoRewind", args = 2)]
    fn entry_no_rewind(self, key: Il2CppString, num: i32) -> bool;

    #[method(name = "IsExist")]
    fn is_exist(self, key: Il2CppString) -> bool;

    #[method(name = "IsString")]
    fn is_string(self, key: Il2CppString) -> bool;

    #[method(name = "Remove")]
    fn remove(self, key: Il2CppString) -> bool;
}

pub struct GameVariableManager;

impl GameVariableManager {
    #[inline]
    fn variable() -> GameVariable {
        GameUserData::instance().get_variable()
    }

    pub fn get_bool(key: impl Into<Il2CppString>) -> bool {
        Self::variable().get_bool(key.into())
    }

    pub fn set_bool(key: impl Into<Il2CppString>, value: bool) {
        Self::variable().set_bool(key.into(), value);
    }

    pub fn get_number(key: impl Into<Il2CppString>) -> i32 {
        Self::variable().get_number(key.into())
    }

    pub fn set_number(key: impl Into<Il2CppString>, value: i32) {
        Self::variable().set_number(key.into(), value);
    }

    pub fn get_string(key: impl Into<Il2CppString>) -> Il2CppString {
        Self::variable().get_string(key.into())
    }

    pub fn set_string(key: impl Into<Il2CppString>, value: impl Into<Il2CppString>) {
        Self::variable().set_string(key.into(), value.into());
    }

    // Creates the entry if missing, seeded with `num`. Mirrors the old engage crate's name.
    pub fn make_entry(key: impl Into<Il2CppString>, num: i32) -> bool {
        Self::variable().entry(key.into(), num)
    }

    // Like `make_entry` but marked no-rewind so the save's rewind-tracking won't roll it back.
    pub fn make_entry_norewind(key: impl Into<Il2CppString>, num: i32) -> bool {
        Self::variable().entry_no_rewind(key.into(), num)
    }

    pub fn is_exist(key: impl Into<Il2CppString>) -> bool {
        Self::variable().is_exist(key.into())
    }

    pub fn is_string(key: impl Into<Il2CppString>) -> bool {
        Self::variable().is_string(key.into())
    }

    pub fn remove(key: impl Into<Il2CppString>) -> bool {
        Self::variable().remove(key.into())
    }
}
