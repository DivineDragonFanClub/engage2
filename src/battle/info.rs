use unity2::Class;
use unity2::ClassIdentity;
use unity2::system::collections::List;
use unity2::system::collections::IList;

use crate::Unit;
use crate::battle::infoside::BattleInfoSide;
use crate::battle::infoside::BattleInfoSideStatus;
use crate::battle::scene::BattleSceneResult;
use crate::battle::side::BattleSideType;
use crate::battle::side::BattleSideContainerArray;
use crate::battle::side::IBattleSideContainerArray;
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::gamedata::skill::SkillData;
use crate::gamedata::person::PersonDataColors;
use crate::gamedata::person::PersonDataRanks;
use crate::map::mapfor::MapForRangeFunction;
use crate::map::mapmind::MapMindType;


#[unity2::class(namespace = "App")]
pub struct BattleInfo {
    #[rename(name = "m_Flag")]
    pub flag: BattleInfoFlagField,
    #[rename(name = "m_Sides")]
    pub sides: BattleInfoBattleInfoSideArray,
    #[rename(name = "m_Supports")]
    pub supports: BattleInfoSupportList,
    #[rename(name = "m_MainUnitEnum")]
    pub main_unit_enum: BattleInfoEnum,
    #[rename(name = "m_WholeUnitEnum")]
    pub whole_unit_enum: BattleInfoEnum,
    #[rename(name = "m_ChainOffenseEnum")]
    pub chain_offense_enum: BattleInfoEnum,
    #[rename(name = "m_ChainDefenseEnum")]
    pub chain_defense_enum: BattleInfoEnum,
    #[rename(name = "m_ChainUnitEnum")]
    pub chain_unit_enum: BattleInfoEnum,
    #[rename(name = "m_TempSkills")]
    pub temp_skills: List<SkillData>,
    #[backing]
    pub range: i32,
    #[backing]
    pub battle_count: i32,
    #[backing]
    pub scene_result: BattleSceneResult,
    #[backing]
    pub chain_attack_side: BattleSideType,
    #[backing]
    pub chain_attack_count: i32,
    #[backing]
    pub chain_guard_count: i32,
    #[backing]
    pub chain_attack_defeat: i32,
    #[backing]
    pub chain_attack_hit: i32,
    #[backing]
    pub chain_attack_critical: i32,
    #[backing]
    pub chain_attack_damage: i32,
    #[backing]
    pub summon_rank: PersonDataRanks,
    #[backing]
    pub summon_color: PersonDataColors,
    #[rename(name = "m_GuardSide")]
    pub guard_side: BattleSideType,
    #[rename(name = "m_GuardFunc")]
    pub guard_func: MapForRangeFunction,
}

impl BattleInfo {
    pub const CRITICAL_FACTOR: i32 = 3;
}

#[unity2::class(namespace = "App")]
pub struct BattleInfoEnum {
    #[rename(name = "m_Info")]
    pub info: BattleInfo,
    #[rename(name = "m_Min")]
    pub min: BattleSideType,
    #[rename(name = "m_Max")]
    pub max: BattleSideType,
    #[rename(name = "m_Current")]
    pub current: BattleInfoSide,
}

#[unity2::class(namespace = "", name = "BattleInfo.FlagField")]
#[parent(BitFieldTemplate32<BattleInfoFlags>)]
pub struct BattleInfoFlagField {}

#[unity2::class(namespace = "", name = "BattleInfo.BattleInfoSideArray")]
#[parent(BattleSideContainerArray<BattleInfoSide>)]
pub struct BattleInfoBattleInfoSideArray {}

#[unity2::class(namespace = "", name = "BattleInfo.SupportData")]
pub struct BattleInfoSupportData {
    pub unit: Unit,
    pub status: BattleInfoSideStatus,
}

#[unity2::class(namespace = "", name = "BattleInfo.SupportList")]
#[parent(unity2::system::collections::List<BattleInfoSupportData>)]
pub struct BattleInfoSupportList {
    #[rename(name = "m_Offense")]
    pub offense: Unit,
    #[rename(name = "m_Defense")]
    pub defense: Unit,
}

#[unity2::class(namespace = "", name = "BattleInfo.SetupScope")]
pub struct BattleInfoSetupScope {
    #[rename(name = "m_Info")]
    pub info: BattleInfo,
    #[rename(name = "m_Offense")]
    pub offense: Unit,
    #[rename(name = "m_Defense")]
    pub defense: Unit,
    #[rename(name = "m_Updated")]
    pub updated: u8,
}

#[unity2::class(namespace = "", name = "BattleInfo.MindScope")]
pub struct BattleInfoMindScope {
    #[rename(name = "m_Unit")]
    pub unit: Unit,
    #[rename(name = "m_Skill")]
    pub skill: SkillData,
    #[rename(name = "m_Mind")]
    pub mind: MapMindType,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BattleInfoFlags: i32 {
        const Simulation = 1 << 0;
        const Warmup = 1 << 1;
        const Alone = 1 << 2;
        const Event = 1 << 3;
        const Dance = 1 << 4;
        const Training = 1 << 5;
        const Rod = 1 << 6;
        const Destroy = 1 << 7;
        const MultiBattle = 1 << 8;
        const BowCannon = 1 << 9;
        const MagicCannon = 1 << 10;
        const FireCannon = 1 << 11;
        const EngageAttack = 1 << 12;
        const Traial = 1 << 13; // Note the typo
        const IgnoreRange = 1 << 14;
        const IgnorePosition = 1 << 15;
        const IgnoreOffensePosition = 1 << 16;
        const IgnoreRevenge = 1 << 17;
        const IgnoreBreak = 1 << 18;
        const IgnoreTerrain = 1 << 19;
        const IgnoreBlow = 1 << 21;
        const IgnoreChain = 1 << 22;
        const IgnoreSkill = 1 << 23;
        const HideCombatGauge = 1 << 24;
        const SkipCombatGrow = 1 << 25;
        const FullBullet = 1 << 26;
        const Summon = 1 << 27;
        const Enchant = 1 << 28;
        const CannonMask = 3584;
    }
}

impl ClassIdentity for BattleInfoFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "BattleInfo.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}