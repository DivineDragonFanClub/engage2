use unity2::Array;
use unity2::Il2CppString;
use crate::data::StructData;
use crate::data::IStructData;


#[unity2::class(namespace = "App")]
#[parent(StructData<BattleStyle>)]
pub struct BattleStyle {
    #[static_field]
    pub begin: BattleStyleTypes,
    #[static_field]
    pub end: BattleStyleTypes,
    #[static_field]
    pub count: i32,
    #[static_field]
    pub names: Array<Il2CppString>,

    #[backing]
    pub style: Il2CppString,
    #[backing]
    pub name: Il2CppString,
    #[backing]
    pub help: Il2CppString,
    #[backing]
    pub skills: Array<Il2CppString>,
}

#[unity2::enumeration(namespace = "", name = "BattleStyle.Types")]
#[repr(i32)]
pub enum BattleStyleTypes {
    None = 0,
    Cooperation = 1,
    Horse = 2,
    Covert = 3,
    Heavy = 4,
    Fly = 5,
    Magic = 6,
    Prana = 7,
    Dragon = 8,
}