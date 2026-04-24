#![allow(unused_imports)]

use unity2::system::collections::Dictionary;
use unity2::{ClassIdentity, Il2CppString, IntPtr};

#[unity2::class(namespace = "App")]
pub struct MsgFile {
    #[rename(name = "m_reference")]
    pub reference_count: i32,
}

#[unity2::methods]
impl MsgFile {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self);

    #[method(name = "Load", args = 1)]
    pub fn load(self, bytes: unity2::Array<u8>);

    #[method(name = "GetTextNum", args = 0)]
    pub fn get_text_num(self) -> i32;

    #[method(name = "GetLabel", args = 1)]
    pub fn get_label(self, index: i32) -> Il2CppString;

    #[method(offset = 0x1E97770)]
    pub fn get_text(self, index: i32) -> IntPtr;

    #[method(name = "GetReference", args = 0)]
    pub fn get_reference(self) -> i32;

    #[method(name = "SetReference", args = 1)]
    pub fn set_reference(self, reference: i32);
}

#[unity2::class(namespace = "App")]
pub struct Mess {
    #[static_field]
    #[rename(name = "s_messFileDictionary")]
    pub mess_file_dictionary: Dictionary<Il2CppString, MsgFile>,

    #[static_field]
    #[rename(name = "s_messDataDictionary")]
    pub mess_data_dictionary: Dictionary<Il2CppString, IntPtr>,

    #[static_field]
    #[rename(name = "s_pathDictionary")]
    pub path_dictionary: Dictionary<Il2CppString, Il2CppString>,
}

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
