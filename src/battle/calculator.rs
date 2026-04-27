use unity2::Array;
use unity2::Class;
use unity2::ClassIdentity;
use unity2::Il2CppString;
use unity2::system::delegate::Delegate;
use unity2::system::delegate::IDelegate;
use unity2::system::delegate::MulticastDelegate;
use unity2::system::delegate::IMulticastDelegate;
use unity2::system::collections::HashSet;
use unity2::system::List;

use crate::battle::info::BattleInfo;
use crate::battle::infoside::BattleInfoSide;
use crate::battle::scene::BattleSceneKind;
use crate::battle::scene::BattleSceneList;
use crate::battle::side::BattleSideType;
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::gamedata::skill::SkillData;
use crate::gamedata::skill::SkillDataActions;
use crate::pool::PoolNode;
use crate::pool::IPoolNode;
use crate::pool::PoolList;
use crate::pool::IPoolList;


#[unity2::class(namespace = "App")]
pub struct BattleCalculator {
    #[static_field]
    #[readonly]
    pub cannon_conditions: Array<Il2CppString>,
    #[static_field]
    #[readonly]
    pub dance_conditions: Array<Il2CppString>,
    #[static_field]
    #[readonly]
    #[rename(name = "EngageSummon3")]
    pub engage_summon_3: Array<Il2CppString>,
    #[static_field]
    #[readonly]
    #[rename(name = "EngageSummon5")]
    pub engage_summon_5: Array<Il2CppString>,

    #[rename(name = "m_Mode")]
    pub mode: BattleCalculatorMode,
    #[rename(name = "m_Info")]
    pub info: BattleInfo,
    #[rename(name = "m_Flag")]
    pub flag: BattleCalculatorFlagField,
    #[rename(name = "m_SceneList")]
    pub scene_list: BattleSceneList,
    #[rename(name = "m_Orders")]
    pub orders: BattleCalculatorOrderList,
    #[rename(name = "m_NextOrderIndex")]
    pub next_order_index: i32,
    #[rename(name = "m_EquipSkill")]
    pub equip_skill: SkillData,
    #[rename(name = "m_ChainOffenses")]
    pub chain_offenses: List<BattleInfoSide>,
    #[rename(name = "m_ChainDefenses")]
    pub chain_defenses: List<BattleInfoSide>,
    #[rename(name = "m_HitSkillPool")]
    pub hit_skill_pool: BattleCalculatorHitSkillPool,
    #[rename(name = "m_CommitSkillUnits")]
    pub commit_skill_units: HashSet<i32>,
}

#[unity2::class(namespace = "", name = "BattleCalculator.FuncExp1")]
#[parent(unity2::system::delegate::MulticastDelegate)]
pub struct BattleCalculatorFuncExp1 {}

#[unity2::class(namespace = "", name = "BattleCalculator.FuncExp2")]
#[parent(unity2::system::delegate::MulticastDelegate)]
pub struct BattleCalculatorFuncExp2 {}

#[unity2::class(namespace = "", name = "BattleCalculator.DetailScope")]
pub struct BattleCalculatorDetailScope {
    #[rename(name = "m_Info")]
    pub info: BattleInfo,
}

#[unity2::class(namespace = "", name = "BattleCalculator.SeparatorScope")]
pub struct BattleCalculatorSeparatorScope {
    #[rename(name = "m_Calc")]
    pub calc: BattleCalculator,
    #[rename(name = "m_Push")]
    pub push: BattleSceneKind,
    #[rename(name = "m_Pop")]
    pub pop: BattleSceneKind,
    #[rename(name = "m_Side")]
    pub side: BattleSideType,
    #[rename(name = "m_EquipSkill")]
    pub equip_skill: SkillData,
    #[rename(name = "m_IsDump")]
    pub is_dump: bool,
}

#[unity2::class(namespace = "", name = "BattleCalculator.TargetScope")]
pub struct BattleCalculatorTargetScope {
    #[rename(name = "m_Current")]
    pub current: BattleInfoSide,
    #[rename(name = "m_Reverse")]
    pub reverse: BattleInfoSide,
    #[rename(name = "m_IsDump")]
    pub is_dump: bool,
}

#[unity2::class(namespace = "", name = "BattleCalculator.HitSkillPool")]
#[parent(PoolList<BattleCalculatorHitSkill>)]
pub struct BattleCalculatorHitSkillPool {}

#[unity2::class(namespace = "", name = "BattleCalculator.HitSkill")]
#[parent(PoolNode)]
pub struct BattleCalculatorHitSkill {
    pub side: BattleInfoSide,
    pub action: SkillDataActions,
    pub skill: SkillData,
}

#[unity2::class(namespace = "", name = "BattleCalculator.OrderList")]
#[parent(PoolList<BattleCalculatorOrder>)]
pub struct BattleCalculatorOrderList {}

impl BattleCalculatorOrderList {
    pub const MAX_ORDER: i32 = 32;
}

#[unity2::class(namespace = "", name = "BattleCalculator.Order")]
#[parent(PoolNode)]
pub struct BattleCalculatorOrder {
    pub side: BattleSideType,
}

#[unity2::class(namespace = "", name = "BattleCalculator.FlagField")]
#[parent(BitFieldTemplate32<BattleCalculatorFlags>)]
pub struct BattleCalculatorFlagField {}

#[unity2::enumeration(namespace = "", name = "BattleCalculator.Attributes")]
#[repr(i32)]
pub enum BattleCalculatorAttributes {
    None = 0,
    Physical = 1,
    Magic = 2,
}

#[unity2::enumeration(namespace = "", name = "BattleCalculator.TrainingResult")]
#[repr(i32)]
pub enum BattleCalculatorTrainingResult {
    Win = 0,
    Lose = 1,
}

#[unity2::enumeration(namespace = "", name = "BattleCalculator.Mode")]
#[repr(i32)]
pub enum BattleCalculatorMode {
    Battle = 0,
    JobIntro = 1,
    ClassChange = 2,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BattleCalculatorFlags: i32 {
        const InterruptOffense = 1 << 0;
        const InterruptDefense = 1 << 1;
        const Interrupting = 1 << 2;
        const ContinueBattle = 1 << 3;
        const SwapOrder = 1 << 4;
        const Dead = 1 << 5;
        const ChainAttacked = 1 << 6;
        const Commited = 1 << 7;
        const MaskInterrupt = 7;
    }
}

impl ClassIdentity for BattleCalculatorFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "BattleCalculator.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}