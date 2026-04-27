#![allow(unused_imports)]

use unity2::system::List;
use unity2::{ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App", name = "StructData`1")]
pub struct StructData<T: ClassIdentity> {
    #[rename(name = "Index")]
    pub index: i32,
    #[rename(name = "Hash")]
    pub hash: i32,
    #[rename(name = "Key")]
    pub key: Il2CppString,
}

#[unity2::class(namespace = "App", name = "SingletonClass`1")]
pub struct SingletonClass<T: ClassIdentity> {}

#[unity2::methods]
impl<T: ClassIdentity> SingletonClass<T> {
    #[method(name = "get_Instance")]
    pub fn instance() -> T;
}

#[unity2::methods]
impl<T: ClassIdentity> StructData<T> {
    #[method(name = "Get")]
    pub fn get(name: Il2CppString) -> T;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(name: Il2CppString) -> T;

    #[method(name = "TryGetFromHash")]
    pub fn try_get_from_hash(hash: i32) -> T;

    #[method(name = "UnsafeGet")]
    pub fn unsafe_get(index: i32) -> T;

    #[method(name = "GetIndex")]
    pub fn get_index(name: Il2CppString) -> i32;

    #[method(name = "TryGetIndex")]
    pub fn try_get_index(name: Il2CppString) -> i32;

    #[method(name = "GetList")]
    pub fn get_list() -> List<T>;

    #[method(name = "GetCount")]
    pub fn get_count() -> i32;
}