#![allow(unused_imports)]

use unity2::{Array, ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App")]
pub struct TextAssetBundle {}

#[unity2::methods]
impl TextAssetBundle {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, path: Il2CppString);

    #[method(name = "get_Bytes", args = 0)]
    pub fn bytes(self) -> Array<u8>;

    #[method(name = "set_Bytes", args = 1)]
    pub fn set_bytes(self, value: Array<u8>);
}
