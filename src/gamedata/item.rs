#![allow(unused_imports)]

use unity2::{Array, Class, ClassIdentity, Il2CppString};

use crate::{SkillArray, UnitItem, WeaponLevelKind, capability::CapabilitySbyte, data::{IStructData, IStructDataMethods, StructData}, unitanim::UnitAnimTypes};
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;

#[unity2::class(namespace = "App")]
#[parent(StructData<ItemData>)]
pub struct ItemData {
    #[backing] pub iid: Il2CppString,
    #[backing] pub name: Il2CppString,
    #[backing] pub help: Il2CppString,
    #[backing] pub tutorial: Il2CppString,
    #[backing] pub kind: ItemDataKinds,
    #[backing] pub attr: ItemDataAttrs,
    #[backing] pub use_type: ItemDataUseTypes,
    #[backing] pub weapon_attr: ItemDataWeaponAttrs,
    #[backing] pub icon: Il2CppString,
    #[backing] pub endurance: u8,
    #[backing] pub power: u8,
    #[backing] pub weight: u8,
    #[backing] pub range_i: u8,
    #[backing] pub range_o: u8,
    #[backing] pub distance: u8,
    #[backing] pub hit: i16,
    #[backing] pub critical: i16,
    #[backing] pub avoid: i16,
    #[backing] pub secure: i16,
    #[backing] pub price: i32,
    #[backing] pub weapon_level: Il2CppString,
    #[backing] pub rod_type: ItemDataRodTypes,
    #[backing] pub rod_exp: u8,
    #[backing] pub rate_arena: u8,
    #[backing] pub shoot_effect: Il2CppString,
    #[backing] pub hit_effect: Il2CppString,
    #[backing] pub cannon_effect: Il2CppString,
    #[backing] pub overlap_terrain: Il2CppString,
    #[backing] pub flag: ItemDataFlagField,
    #[backing] pub enhance: CapabilitySbyte,
    #[backing] pub grow_ratio: CapabilitySbyte,
    #[backing] pub equip_condition: Il2CppString,
    #[backing] pub equip_sids: Array<Il2CppString>,
    #[backing] pub passive_sids: Array<Il2CppString>,
    #[backing] pub give_sids: Array<Il2CppString>,
    #[backing] pub add_target: ItemDataAddTargets,
    #[backing] pub add_type: ItemDataUseTypes,
    #[backing] pub add_power: u8,
    #[backing] pub add_range: u8,
    #[backing] pub add_sids: Array<Il2CppString>,
    #[backing] pub add_effect: Il2CppString,
    #[backing] pub add_help: Il2CppString,
    #[backing] pub high_rank_item: Il2CppString,
    #[rename(name = "m_IsWeapon")] pub is_weapon: bool,
    #[rename] pub flag_name: Il2CppString,
    #[rename(name = "m_PrefixlessIid")] pub prefixless_iid: Il2CppString,
    #[rename(name = "m_EnchantHash")] pub enchant_hash: i32,
    // Note the "m_" prefix for this field, required because there would be two weapon_level fields
    #[rename(name = "m_WeaponLevel")] pub m_weapon_level: WeaponLevelKind,
    #[rename(name = "m_UnitItem")] pub unit_item: UnitItem,
    #[backing] #[readonly] pub equip_skills: SkillArray,
    #[backing] #[readonly] pub passive_skills: SkillArray,
    #[backing] #[readonly] pub give_skills: SkillArray,
    #[backing(name = "EnchantSkills1")] #[readonly] pub enchant_skills_1: SkillArray,
    #[backing(name = "EnchantSkills2")] #[readonly] pub enchant_skills_2: SkillArray,
    #[backing(name = "EnchantSkills3")] #[readonly] pub enchant_skills_3: SkillArray,
    #[backing(name = "EnchantSkills4")] #[readonly] pub enchant_skills_4: SkillArray,
    #[backing] pub attack_motion: UnitAnimTypes,
}

impl ItemData {
    pub const MAX_NAME_LENGTH: i32 = 8;
    pub const RANGE_INF: i32 = 255;
    pub const ENDURANCE_INF: i32 = 255;
    pub const HIT_INF: i32 = 255;
    pub const EMPTY_ENCHANT_HASH: i32 = 0;
    pub const MAX_INVENTORY: i32 = 999;
    pub const MAX_REFINE: i32 = 9999;
}

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

#[unity2::enumeration(namespace = "", name = "ItemData.AddTargets")]
#[repr(i32)]
pub enum ItemDataAddTargets {
    None = 0,
    Slf = 1, // Renamed from Self
    Around = 2,
    Whole = 3,
}

#[unity2::enumeration(namespace = "", name = "ItemData.AddTargets")]
#[repr(i32)]
pub enum ItemDataRodTypes {
    None = 0,
    Basic = 1,
    Heal = 2,
    Interference = 3,
}

#[unity2::enumeration(namespace = "", name = "ItemData.AddTargets")]
#[repr(i32)]
pub enum ItemDataUseTypes {
    None = 0,
    Attack = 1,
    Heal = 2,
    RestHeal = 3,
    Revive = 4,
    Warp = 5,
    Rescue = 6,
    EngageAdd = 7,
    Rewarp = 8,
    Freeze = 9,
    Sleep = 10,
    Silence = 11,
    Charm = 12,
    Berserk = 13,
    Weakness = 14,
    Again = 15,
    Torch = 16,
    Food = 17,
    Rest = 18,
    SightUp = 19,
    WeaponLevelUp = 20,
    GrowUp = 21,
    Enhance = 22,
    CCMaster = 23,
    CCChange = 24,
    CCExtra = 25,
    Creation = 26,
    Draw = 27,
    GainExp = 28,
    Stun = 29,
    Detox = 30,
    GiveSkill = 31,
    Foodstuff = 32,
    Gift = 33,
    Material = 34,
    FishingRod = 35,
    Bless = 36,
    BlessRest = 37,
    BlessPlus = 38,
    BlessRestPlus = 39,
    CCEnchant = 40,
    CCGunner = 41,
    GainSkillPoint = 42,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ItemDataFlags: i32 {
        const Rarity = 1 << 0;
        const NotTrade = 1 << 1;
        const CanUse = 1 << 2;
        const OnlyChapter = 1 << 3;
        const OnlyEnemy = 1 << 4;
        const OnlyMale = 1 << 5;
        const OnlyFemale = 1 << 6;
        const Engage = 1 << 7;
        const IgnoreWeaponLevel = 1 << 8;
        const Unpublic = 1 << 9;
        const NotEntrust = 1 << 10;
        const InvertInteract = 1 << 11;
        const Download = 1 << 12;
        const KeyDoor = 1 << 13;
        const KeyTreasureBox = 1 << 14;
        const AIUnequipable = 1 << 15;
        const ReverseAttribute = 1 << 16;
        const LunchBox = 1 << 17;
        const SimpleHelp = 1 << 18;
        const RangeTarget = 1 << 19;
        const IgnoreCombat = 1 << 20;
        const ForcedCombat = 1 << 21;
        // No entries for 2^22 and 2^23
        const Bless = 1 << 24;
        const Breath = 1 << 25;
        const Dragon = 1 << 26;
        const Bullet = 1 << 27;
    }
}

impl ClassIdentity for ItemDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "ItemData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::class(namespace = "", name = "ItemData.FlagField")]
#[parent(BitFieldTemplate32<ItemDataFlags>)]
pub struct ItemDataFlagField {}

#[unity2::enumeration(namespace = "", name = "ItemData.WeaponAttrs")]
#[repr(i32)]
pub enum ItemDataWeaponAttrs {
    None = 0,
    Fire = 1,
    Thunder = 2,
    Wind = 3,
    Ice = 4,
    Light = 5,
    Dark = 6,
}

#[unity2::enumeration(namespace = "", name = "ItemData.Attrs")]
#[repr(i32)]
pub enum ItemDataAttrs {
    None = 0,
    Physical = 1,
    Magic = 2,
}

#[unity2::enumeration(namespace = "", name = "ItemData.Kinds")]
#[repr(i32)]
pub enum ItemDataKinds {
    None = 0,
    Sword = 1,
    Lance = 2,
    Axe = 3,
    Bow = 4,
    Dagger = 5,
    Magic = 6,
    Rod = 7,
    Fist = 8,
    Special = 9,
    Tool = 10,
    Shield = 11,
    Accessory = 12,
    Precious = 13,
    RefineIron = 14,
    RefineSteel = 15,
    RefineSilver = 16,
    PieceOfBond = 17,
    Gold = 18,
    Num = 19,
    // WeaponNum = 10,
}