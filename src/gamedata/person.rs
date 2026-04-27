use unity2::Class;
use unity2::ClassIdentity;
use unity2::Il2CppString;

use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::data::StructData;
use crate::data::IStructData;


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

#[unity2::methods]
impl PersonData {
    #[method(name = "get_AsciiName")]
    pub fn ascii_name(self) -> Il2CppString;

    #[method(name = "get_Fid")]
    pub fn fid(self) -> Il2CppString;

    #[method(name = "get_Jid")]
    pub fn jid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    pub fn name(self) -> Il2CppString;

    #[method(name = "GetJob", args = 0)]
    pub fn job(self) -> crate::gamedata::job::JobData;
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

#[unity2::class(namespace = "", name = "PersonData.FlagField")]
#[parent(BitFieldTemplate32<PersonDataFlags>)]
pub struct PersonDataFlagField {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct PersonDataFlags: i32 {
        const CandidateForFriend = 1 << 0;
        const BelongName = 1 << 1;
        const Talent = 1 << 2;
        const IgnoreJobSkillRemove = 1 << 3;
        const DarkWarp = 1 << 4;
        const DressReverse = 1 << 5;
        const SimpleUI = 1 << 6;
        const DerivedHero = 1 << 7;
        const SummonWarp = 1 << 8;
    }
}

impl ClassIdentity for PersonDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "PersonData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "PersonData.Country")]
#[repr(i32)]
pub enum PersonDataCountry {
    Free = 0,
    Lithos = 1,
    Filene = 2,
    Brodia = 3,
    Ircion = 4,
    Solum = 5,
    Gradlon = 6,
}

#[unity2::enumeration(namespace = "", name = "PersonData.Timing")]
#[repr(i32)]
pub enum PersonDataTiming {
    None = 0,
    Begin = 1,
    End = 2,
    Chapter = 3,
    Eternal = 4,
}

#[unity2::enumeration(namespace = "", name = "PersonData.Colors")]
#[repr(i32)]
pub enum PersonDataColors {
    None = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[unity2::enumeration(namespace = "", name = "PersonData.Ranks")]
#[repr(i32)]
pub enum PersonDataRanks {
    None = 0,
    Rank1 = 1,
    Rank2 = 2,
    Rank3 = 3,
}