use unity2::Class;
use unity2::ClassIdentity;

use crate::battle::info::BattleInfo;
use crate::battle::side::BattleSideType;
use crate::battle::side::BattleSideSbyteArray;
use crate::battle::side::BattleSideShortArray;
use crate::bit::BitFieldCommon;
use crate::bit::IBitFieldCommon;
use crate::bit::BitField32;
use crate::bit::IBitField32;
use crate::bit::BitFieldTemplate32;
use crate::bit::IBitFieldTemplate32;
use crate::pool::PoolNode;
use crate::pool::IPoolNode;
use crate::pool::PoolList;
use crate::pool::IPoolList;


#[unity2::class(namespace = "App")]
#[parent(PoolNode)]
pub struct BattleScene {
    #[rename(name = "m_List")]
    pub list: BattleSceneList,
    #[rename(name = "m_Side")]
    pub side: BattleSideType,
    #[rename(name = "m_Target")]
    pub target: BattleSideType,
    #[rename(name = "m_Kind")]
    pub kind: BattleSceneKind,
    #[rename(name = "m_Skill")]
    pub skill: i32,
    #[rename(name = "m_Item")]
    pub item: i32,
    #[rename(name = "m_God")]
    pub god: i32,
    #[rename(name = "m_Index")]
    pub index: i32,
    #[rename(name = "m_Result")]
    pub result: BattleSceneFieldResult,
    #[rename(name = "m_Guardian")]
    pub guardian: BattleSideType,
    #[rename(name = "m_Hps")]
    pub hps: BattleSideShortArray,
    #[rename(name = "m_Engages")]
    pub engages: BattleSideSbyteArray,
    #[rename(name = "m_Damages")]
    pub damages: BattleSideShortArray,
}

#[unity2::class(namespace = "App")]
#[parent(PoolList<BattleScene>)]
pub struct BattleSceneList {
    #[rename(name = "m_Info")]
    pub info: BattleInfo,
    #[rename(name = "m_Index")]
    pub index: i32,
}

impl BattleSceneList {
    pub const MAX_SCENE: i32 = 512;
    pub const MAX_TIMES: i32 = 4;
}

#[unity2::class(namespace = "", name = "BattleScene.FieldResult")]
#[parent(BitFieldTemplate32<BattleSceneResult>)]
pub struct BattleSceneFieldResult {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct BattleSceneResult: i32 {
        const None = 0;
        const Hit = 1 << 0;
        const Critical = 1 << 1;
        const Guard = 1 << 2;
        const Suicide = 1 << 3;
        const Efficacy = 1 << 4;
        const Break = 1 << 5;
        const Blow = 1 << 6;
        const Bounce = 1 << 7;
        const ChainAttack = 1 << 8;
        const ChainGuard = 1 << 9;
        const DualGuard = 1 << 10;
        const EngageAttack = 1 << 11;
        const Physical = 1 << 12;
        const Magic = 1 << 13;
        const Ignore = 1 << 14;
    }
}

impl ClassIdentity for BattleSceneResult {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "BattleScene.Result";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "BattleScene.Kind")]
#[repr(i32)]
pub enum BattleSceneKind {
    None = 0,
    Attack = 1,
    Rod = 2,
    Dance = 3,
    Skill = 4,
    GiveDirect = 5,
    GiveDelay = 6,
    Strip = 7,
    Equip = 8,
    God = 9,
    Dead = 10,
    EngageAttack = 11,
    Separator = 12,
    PushBattle = 13,
    PushOrder = 14,
    PushAction = 15,
    PushAttack = 16,
    PopAttack = 17,
    PopAction = 18,
    PopOrder = 19,
    PopBattle = 20,
    Heal = 21,
    Num = 22,
}