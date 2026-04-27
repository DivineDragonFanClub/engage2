use unity2::Array;
use unity2::ClassIdentity;


#[unity2::class(namespace = "App")]
pub struct BattleSide {
    #[static_field]
    #[readonly]
    pub parents: Array<BattleSideType>,
    #[static_field]
    #[readonly]
    pub reverses: Array<BattleSideType>,
    #[static_field]
    #[readonly]
    pub stands: Array<BattleSideType>,
}

impl BattleSide {
    pub const MAIN_NUM: i32 = 2;
    pub const CHAIN_OFFENSE_NUM: i32 = 24;
    pub const CHAIN_DEFENSE_NUM: i32 = 4;
    pub const WHOLE: i32 = 30;
    pub const LINK_OFFENSE: BattleSideType = BattleSideType::ChainOffense1;
    pub const MAIN_BEGIN: BattleSideType = BattleSideType::Offense;
    pub const MAIN_END: BattleSideType = BattleSideType::Defense;
    pub const CHAIN_OFFENSE_MIN: BattleSideType = BattleSideType::ChainOffense1;
    pub const CHAIN_OFFENSE_MAX: BattleSideType = BattleSideType::ChainOffense24;
    pub const CHAIN_DEFENSE_MIN: BattleSideType = BattleSideType::ChainDefense1;
    pub const CHAIN_DEFENSE_MAX: BattleSideType = BattleSideType::ChainDefense4;
    pub const SUPPORT_BEGIN: BattleSideType = BattleSideType::ChainOffense1;
    pub const SUPPORT_END: BattleSideType = BattleSideType::ChainDefense4;
}

#[unity2::class(namespace = "", name = "BattleSide.StructArray`1")]
pub struct BattleSideStructArray<T: ClassIdentity> {
    #[rename(name = "m_Array")]
    pub array: Array<T>,
}

#[unity2::class(namespace = "", name = "BattleSide.ContainerArray`1")]
pub struct BattleSideContainerArray<T: ClassIdentity> {
    #[rename(name = "m_Array")]
    pub array: Array<T>,
}

#[unity2::class(namespace = "", name = "BattleSide.ClassArray")]
#[parent(BattleSideContainerArray<T>)]
pub struct BattleSideClassArray<T: ClassIdentity> { }

#[unity2::class(namespace = "", name = "BattleSide.IntArray")]
#[parent(BattleSideStructArray<i32>)]
pub struct BattleSideIntArray {}

#[unity2::class(namespace = "", name = "BattleSide.FloatArray")]
#[parent(BattleSideStructArray<f32>)]
pub struct BattleSideFloatArray {}

#[unity2::class(namespace = "", name = "BattleSide.ShortArray")]
#[parent(BattleSideStructArray<i16>)]
pub struct BattleSideShortArray {}

#[unity2::class(namespace = "", name = "BattleSide.SbyteArray")]
#[parent(BattleSideStructArray<i8>)]
pub struct BattleSideSbyteArray {}

#[unity2::enumeration(namespace = "", name = "BattleSide.Type")]
#[repr(i32)]
pub enum BattleSideType {
    None = (-1),
    Offense = 0,
    Defense = 1,
    ChainOffense1 = 2,
    ChainOffense2 = 3,
    ChainOffense3 = 4,
    ChainOffense4 = 5,
    ChainOffense5 = 6,
    ChainOffense6 = 7,
    ChainOffense7 = 8,
    ChainOffense8 = 9,
    ChainOffense9 = 10,
    ChainOffense10 = 11,
    ChainOffense11 = 12,
    ChainOffense12 = 13,
    ChainOffense13 = 14,
    ChainOffense14 = 15,
    ChainOffense15 = 16,
    ChainOffense16 = 17,
    ChainOffense17 = 18,
    ChainOffense18 = 19,
    ChainOffense19 = 20,
    ChainOffense20 = 21,
    ChainOffense21 = 22,
    ChainOffense22 = 23,
    ChainOffense23 = 24,
    ChainOffense24 = 25,
    ChainDefense1 = 26,
    ChainDefense2 = 27,
    ChainDefense3 = 28,
    ChainDefense4 = 29, 
}