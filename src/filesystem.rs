#![allow(unused_imports)]

use unity2::system::collections::{Dictionary, List};
use unity2::{Array, ClassIdentity, Il2CppString, OptionalMethod};

#[unity2::class(namespace = "App")]
pub struct FileCommon {
    #[static_field]
    #[rename(name = "s_Dictionary")]
    pub dictionary: Dictionary<Il2CppString, FileData>,
}

#[unity2::methods]
impl FileCommon {
    #[method(name = "GetFullPath", args = 1)]
    pub fn get_full_path(path: Il2CppString) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
pub struct FileData {
    #[rename(name = "m_State")]
    pub state: i32,
    #[rename(name = "m_Path")]
    pub path: Il2CppString,
    #[rename(name = "m_Data")]
    pub data: Array<u8>,
    #[rename(name = "m_Refer")]
    pub refer: BindHolder,
}

#[unity2::class(namespace = "App")]
pub struct BindHolder {
    #[rename(name = "m_Bind")]
    pub bind: i32,
}

#[unity2::methods]
impl BindHolder {
    #[method(name = "Bind", args = 0)]
    pub fn bind(self) -> bool;

    #[method(name = "Unbind", args = 0)]
    pub fn unbind(self) -> bool;
}

#[unity2::class(namespace = "App", name = "FileHandle`1")]
pub struct FileHandle {
    #[rename(name = "m_Data")]
    pub data: FileData,
}

impl FileHandle {
    pub fn unload(self) {
        use unity2::{Cast, MethodInfo};
        let class = self.get_class();
        let method = class
            .raw()
            .get_method_from_name("Unload", 0)
            .expect("FileHandle::Unload missing from runtime class");
        let unload: extern "C" fn(Self, &MethodInfo) = unsafe { std::mem::transmute(method.method_ptr) };
        unload(self, method);
    }
}
