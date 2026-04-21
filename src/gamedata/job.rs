#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};

#[unity2::class(namespace = "App")]
#[parent(StructData<JobData>)]
pub struct JobData {}

#[unity2::methods]
impl JobData {
    #[method(name = "get_Jid")]
    fn jid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Aid")]
    fn aid(self) -> Il2CppString;

    #[method(name = "get_Help")]
    fn help(self) -> Il2CppString;

    #[method(name = "get_UnitIconID_M")]
    fn unit_icon_id_m(self) -> Il2CppString;

    #[method(name = "get_UnitIconID_F")]
    fn unit_icon_id_f(self) -> Il2CppString;

    #[method(name = "get_Rank")]
    fn rank(self) -> i8;

    #[method(name = "get_StyleName")]
    fn style_name(self) -> Il2CppString;
}
