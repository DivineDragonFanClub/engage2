use unity2::Class;
use unity2::ClassIdentity;

use crate::battle::destory::BattleDestory;
use crate::battle::detail::BattleDetail;
use crate::battle::info::BattleInfo;
use crate::battle::scene::BattleSceneResult;
use crate::battle::side::BattleSideType;
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::gamedata::unititem::UnitItem;
use crate::gamedata::skill::SkillArray;
use crate::gamedata::terrain::TerrainData;
use crate::pool::PoolHierarchy;
use crate::supportcalculator::SupportCalculator;
use crate::Unit;


#[unity2::class(namespace = "App")]
pub struct BattleInfoSide {
    #[static_field]
    #[readonly]
    pub continue_condition: i32,

    #[rename(name = "m_Info")]
    pub info: BattleInfo,
    #[rename(name = "m_SideType")]
    pub side_type: BattleSideType,
    #[rename(name = "m_Unit")]
    pub unit: Unit,
    #[rename(name = "m_UnitItem")]
    pub unit_item: UnitItem,
    #[rename(name = "m_SpecifiedItem")]
    pub specified_item: UnitItem,
    #[rename(name = "m_X")]
    pub x: i32,
    #[rename(name = "m_Z")]
    pub z: i32,
    #[rename(name = "m_Terrain")]
    pub terrain: TerrainData,
    #[rename(name = "m_Overlap")]
    pub overlap: TerrainData,
    #[rename(name = "m_Status")]
    pub status: BattleInfoSideBitFieldStatus,
    #[rename(name = "m_Detail")]
    pub detail: BattleDetail,
    #[rename(name = "m_Hierarchy")]
    pub hierarchy: PoolHierarchy<BattleDetail>,
    #[rename(name = "m_Support")]
    pub support: SupportCalculator,
    #[rename(name = "m_Parent")]
    pub parent: BattleInfoSide,
    #[rename(name = "m_Reverse")]
    pub reverse: BattleInfoSide,
    #[rename(name = "m_Destroy")]
    pub destroy: BattleDestory,
    #[rename(name = "m_MaskSkill")]
    pub mask_skill: SkillArray,
    #[backing]
    pub level: i32,
    #[backing]
    pub hp: i32,
    #[backing]
    pub gain_exp: i32,
    #[backing]
    pub gain_gold: i32,
    #[backing]
    pub drop_item_ratio: f32,
    #[backing]
    pub pickup_item_index: i32,
    #[backing]
    pub damage: i32,
    #[backing]
    pub heal: i32,
    #[backing]
    pub battle_times: i32,
    #[backing]
    pub total_order: i32,
    #[backing]
    pub total_action: i32,
    #[backing]
    pub total_attack: i32,
    #[backing]
    pub total_damage: i32,
    #[backing]
    pub total_result: BattleSceneResult,
    #[backing]
    pub temporary: i32,
    #[backing]
    pub stun: i32,
    #[backing]
    pub engage_count: i32,
    #[backing]
    pub engage_first_count: i32,
    #[backing]
    pub blown_distance: i32,
    #[backing]
    pub weapon_expend: i32,
    #[backing]
    pub expend_count: i32,
}

#[unity2::class(namespace = "", name = "BattleInfoSide.BitFieldStatus")]
#[parent(BitFieldTemplate32<BattleInfoSideStatus>)]
pub struct BattleInfoSideBitFieldStatus {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BattleInfoSideStatus: i32 {
        const Offense = 1 << 0;
        const Defense = 1 << 1;
        const ChainAttack = 1 << 2;
        const ChainGuard = 1 << 3;
        const EngageLink = 1 << 4;
        const IgnorePostion = 1 << 5;
        const IgnoreRange = 1 << 6;
        const Magic = 1 << 7;
        const Rod = 1 << 8;
        const HealRod = 1 << 9;
        const InterferenceRod = 1 << 10;
        const LongRange = 1 << 11;
        const NotWeapon = 1 << 12;
        const NotAttack = 1 << 13;
        const NotStun = 1 << 14;
        const MoveChainAttack = 1 << 15;
        const HauntChainAttack = 1 << 16;
        const ExpBattle = 1 << 17;
        const ExpDestroy = 1 << 18;
        const ExpRod = 1 << 19;
        const ExpRodMiss = 1 << 20;
        const GiveExpBattle = 1 << 21;
        // Note the absence of a 2^22 = 4194304 value.
        const Gained = 1 << 23;
        const Dead = 1 << 24;
        const ChainAttacked = 1 << 25;
        const ChainGuarded = 1 << 26;
        const Blown = 1 << 27;
        const Bounced = 1 << 28;
        const Breaked = 1 << 29;
        const Interrupting = 1 << 30;
        const ChangeDragon = -2147483648; // i32::MIN
        const MaskNoAttack = 12544;
        const MaskExp = 1966080;
        const MaskChain = 12;
    }
}

impl ClassIdentity for BattleInfoSideStatus {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "BattleInfoSide.Status";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}