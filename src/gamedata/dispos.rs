#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<ChapterData>)]
pub struct ChapterData {}

#[unity2::methods]
impl ChapterData {
    #[method(name = "get_Cid")]
    fn cid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Mess")]
    fn mess(self) -> Il2CppString;

    #[method(name = "get_Field")]
    fn field(self) -> Il2CppString;
}
