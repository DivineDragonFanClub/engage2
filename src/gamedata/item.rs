#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<ItemData>)]
pub struct ItemData {}

#[unity2::methods]
impl ItemData {
    #[method(name = "get_Iid")]
    fn iid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Help")]
    fn help(self) -> Il2CppString;

    #[method(name = "get_Tutorial")]
    fn tutorial(self) -> Il2CppString;

    #[method(name = "get_Aid")]
    fn aid(self) -> Il2CppString;

    #[method(name = "get_Icon", args = 0)]
    pub fn icon(self) -> Il2CppString;
}
