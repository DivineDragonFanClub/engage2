#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App")]
pub struct Mess {}

#[unity2::methods]
impl Mess {
    #[method(name = "Get", args = 1)]
    fn get_raw(label: Il2CppString) -> Il2CppString;

    #[method(name = "Load", args = 1)]
    fn load_raw(file_name: Il2CppString) -> bool;

    #[method(name = "GetLanguageDirectoryName", args = 0)]
    fn get_language_directory_name_raw() -> Il2CppString;

    #[method(name = "GetFilePath", args = 1)]
    fn get_file_path_raw(label: Il2CppString) -> Il2CppString;
}

impl Mess {
    #[inline]
    pub fn get(label: impl Into<Il2CppString>) -> Il2CppString {
        Self::get_raw(label.into())
    }

    #[inline]
    pub fn load(file_name: impl Into<Il2CppString>) -> bool {
        Self::load_raw(file_name.into())
    }

    #[inline]
    pub fn get_language_directory_name() -> Il2CppString {
        Self::get_language_directory_name_raw()
    }

    #[inline]
    pub fn get_file_path(label: impl Into<Il2CppString>) -> Il2CppString {
        Self::get_file_path_raw(label.into())
    }
}
