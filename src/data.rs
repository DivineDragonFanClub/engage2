#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App", name = "StructData`1")]
pub struct StructData<T: ClassIdentity> {}

#[unity2::class(namespace = "App", name = "SingletonClass`1")]
pub struct SingletonClass<T: ClassIdentity> {}

#[unity2::methods]
impl<T: ClassIdentity> SingletonClass<T> {
    #[method(name = "get_Instance")]
    fn instance() -> T;
}

#[unity2::enumeration]
#[repr(i32)]
pub enum Gender {
    None = 0,
    Male = 1,
    Female = 2,
    Other = 3,
    Num = 4,
}

#[unity2::methods]
impl<T: ClassIdentity> StructData<T> {
    #[method(name = "Get")]
    fn get(name: Il2CppString) -> T;

    #[method(name = "TryGet", args = 1)]
    fn try_get(name: Il2CppString) -> T;

    #[method(name = "TryGetFromHash")]
    fn try_get_from_hash(hash: i32) -> T;

    #[method(name = "UnsafeGet")]
    fn unsafe_get(index: i32) -> T;

    #[method(name = "GetIndex")]
    fn get_index(name: Il2CppString) -> i32;

    #[method(name = "TryGetIndex")]
    fn try_get_index(name: Il2CppString) -> i32;
}

#[unity2::class(namespace = "App")]
#[parent(StructData<PersonData>)]
pub struct PersonData {
    #[backing]
    pub pid: Il2CppString,
    #[backing]
    pub gender: Gender,
    #[backing]
    pub level: u8,
    #[backing(name = "UnitIconID")]
    pub unit_icon_id: Il2CppString,
}
