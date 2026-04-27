#![allow(unused_imports)]

use unity2::ClassIdentity;
use unity2::Array;
use unity2::Class;
use unity2::system::collections::List;
use unity2::system::collections::IList;
use unity2::system::collections::Stack;

use crate::data::{ISingletonClass, ISingletonClassMethods, SingletonClass};
use crate::gamedata::unit::Unit;
use crate::ItemData;
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::dir::DirType;
use crate::gamedata::unititem::UnitItem;
use crate::gamedata::skill::SkillData;
use crate::gamedata::terrain::TerrainData;


#[unity2::class(namespace = "App")]
#[parent(SingletonClass<MapMind>)]
pub struct MapMind {
    #[rename(name = "m_UnitIndex")]
    pub unit_index: u8,
    #[rename(name = "m_FirstUnitIndex")]
    pub first_unit_index: u8,
    #[rename(name = "m_FirstX")]
    pub first_x: i8,
    #[rename(name = "m_FirstZ")]
    pub first_z: i8,
    #[rename(name = "m_UnitShowX")]
    pub unit_show_x: i8,
    #[rename(name = "m_UnitShowZ")]
    pub unit_show_z: i8,
    #[rename(name = "m_X")]
    pub x: i8,
    #[rename(name = "m_Z")]
    pub z: i8,
    #[rename(name = "m_Mind")]
    pub mind: MapMindType,
    #[rename(name = "m_AttackX")]
    pub attack_x: i8,
    #[rename(name = "m_AttackZ")]
    pub attack_z: i8,
    #[rename(name = "m_ItemIndex")]
    pub item_index: i8,
    #[rename(name = "m_TargetUnitIndex")]
    pub target_unit_index: u8,
    #[rename(name = "m_TargetX")]
    pub target_x: i8,
    #[rename(name = "m_TargetZ")]
    pub target_z: i8,
    #[rename(name = "m_FocusX")]
    pub focus_x: i8,
    #[rename(name = "m_FocusZ")]
    pub focus_z: i8,
    #[rename(name = "m_TargetArgument")]
    pub target_argument: i16,
    #[rename(name = "m_TradeUnitIndex")]
    pub trade_unit_index: u8,
    #[rename(name = "m_EventUnitIndex")]
    pub event_unit_index: u8,
    #[rename(name = "m_Done")]
    pub done: MapMindDoneField,
    #[rename(name = "m_MovePower")]
    pub move_power: u8,
    #[rename(name = "m_TransporterIndex")]
    pub transporter_index: i16,
    #[rename(name = "m_CommandSkill")]
    pub command_skill: SkillData,
    #[rename(name = "m_SpecifiedItem")]
    pub specified_item: ItemData,
    #[rename(name = "m_AIEngageRewarpX")]
    pub ai_engage_rewarp_x: i8,
    #[rename(name = "m_AIEngageRewarpZ")]
    pub ai_engage_rewarp_z: i8,
    #[rename(name = "m_Routes")]
    pub routes: Array<DirType>,
    #[backing]
    pub targets: MapMindMultiTargets,
    #[backing]
    pub stack: MapMindCommandStack,
}

#[unity2::methods]
impl MapMind {
    #[method(name = "get_Unit", args = 0)]
    pub fn unit(self) -> Unit;

    #[method(name = "get_TargetUnit", args = 0)]
    pub fn target_unit(self) -> Unit;
}

impl MapMind {
    pub fn get_unit() -> Unit {
        Self::instance().unit()
    }

    pub fn get_target_unit() -> Unit {
        Self::instance().target_unit()
    }
}

#[unity2::class(namespace = "", name = "MapMind.MultiTargets")]
#[parent(unity2::system::collections::List<MapMindTarget>)]
pub struct MapMindMultiTargets {}

#[unity2::class(namespace = "", name = "MapMind.Target")]
pub struct MapMindTarget {
    pub unit: Unit,
    pub terrain: TerrainData,
    pub x: i8,
    pub z: i8,
}

#[unity2::class(namespace = "", name = "MapMind.CommandStack")]
pub struct MapMindCommandStack {
    #[rename(name = "m_Statck")] // Note the typo
    pub stack: Stack<MapMindRecord>,
}

#[unity2::class(namespace = "", name = "MapMind.Record")]
pub struct MapMindRecord {
    #[rename(name = "type")]
    pub ty: MapMindType,
    pub main: MapMindRecordValue,
    pub link: MapMindRecordValue,
}

#[unity2::class(namespace = "", name = "MapMind.Record.Value")]
pub struct MapMindRecordValue {
    pub unit: Unit,
    pub x: i32,
    pub z: i32,
    #[rename(name = "isChanged")]
    pub is_changed: bool,
    #[rename(name = "engageCount")]
    pub engage_count: i32,
    #[rename(name = "unitItem")]
    pub unit_item: UnitItem,
}

#[unity2::class(namespace = "", name = "MapMind.DoneField")]
#[parent(BitFieldTemplate32<MapMindDone>)]
pub struct MapMindDoneField {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct MapMindDone: i32 {
        const Talk = 1 << 0;
        const Trade = 1 << 1;
        const Putoff = 1 << 2;
        const Transporter = 1 << 3;
        const Action = 1 << 4;
        const Sight = 1 << 5;
    }
}

impl ClassIdentity for MapMindDone {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "MapMind.Done";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "MapMind.Type")]
#[repr(i32)]
pub enum MapMindType {
    None = 0,
    Fixed = 1,
    Talk = 2,
    Attack = 3,
    EngageStart = 4,
    EngageLink = 5,
    EngageAttack = 6,
    EngageRod = 7,
    EngageRewarp = 8,
    EngageCharge = 9,
    Cannon = 10,
    Destroy = 11,
    Rod = 12,
    ItemUse = 13,
    Item = 14,
    Trade = 15,
    Visit = 16,
    Breakdown = 17,
    BreakdownEnemy = 18,
    Escape = 19,
    Breakthrough = 20,
    Door = 21,
    Torch = 22,
    TreasureBox = 23,
    Transporter = 24,
    RodWarp = 25,
    RodWarpDest = 26,
    RodRewarp = 27,
    RodRewarpDest = 28,
    RodRescue = 29,
    RodInterference = 30,
    RodTorch = 31,
    RodCreation = 32,
    RodNodus = 33,
    Dance = 34,
    Guard = 35,
    DragonVein = 36,
    OverlapSkill = 37,
    CommandSkill = 38,
    VisionCreate = 39,
    VisionDelete = 40,
    GodChange = 41,
    DestroyVillage = 42,
    TurnEnd = 43,
    Surrender = 44,
    Informal = 45,
    RodHeal = 46,
    RodMagicShield = 47,
    FullBullet = 48,
    EngageWait = 49,
    EngageSummon = 50,
    ItemMenu = 51,
    EnchantMenu = 52,
    Enchant = 53,
    Contract = 54,
    SubMenu = 55,
}