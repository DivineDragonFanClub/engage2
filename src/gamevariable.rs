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

    #[method(name = "IsExist")]
    fn is_exist(self, key: Il2CppString) -> bool;
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

    pub fn is_exist(key: impl Into<Il2CppString>) -> bool {
        Self::variable().is_exist(key.into())
    }
}
