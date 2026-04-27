use unity2::Array;

use crate::battle::calculator::BattleCalculatorAttributes;
use crate::battle::param::BattleParam;
use crate::battle::param::IBattleParam;
use crate::calculator::calculator::CalculatorCommand;
use crate::capability::CapabilityInt;
use crate::gamedata::skill::SkillArray;
use crate::gamedata::skill::SkillDataLayers;
use crate::pool::PoolNode;
use crate::pool::IPoolNode;


#[unity2::class(namespace = "App")]
#[parent(PoolNode)]
pub struct BattleDetail {
    #[backing]
    pub capability: CapabilityInt,
    #[rename(name = "m_BaseParams")]
    pub base_params: Array<i32>,
    #[rename(name = "m_BattleParams")]
    pub battle_params: Array<BattleParam>,
    #[backing]
    pub attack_attribute: BattleCalculatorAttributes,
    #[backing]
    pub skill_layers: SkillDataLayers,
    #[rename(name = "m_ActiveSkill")]
    pub active_skill: SkillArray,
}

// BattleDetail.CommandParams
#[unity2::class(namespace = "", name = "BattleDetail.CommandParam")]
#[parent(BattleParam)]
pub struct BattleDetailCommandParam {
    #[rename(name = "m_Command")]
    pub command: CalculatorCommand,
}

#[unity2::class(namespace = "", name = "BattleDetail.SimplePowerParam")]
#[parent(BattleDetailCommandParam)]
pub struct BattleDetailSimplePowerlParam {}

#[unity2::class(namespace = "", name = "BattleDetail.SimpleHitParam")]
#[parent(BattleDetailCommandParam)]
pub struct BattleDetailSimpleHitParam {}

#[unity2::class(namespace = "", name = "BattleDetail.SimpleCriticalParam")]
#[parent(BattleDetailCommandParam)]
pub struct BattleDetailSimpleCriticalParam {}

// BattleDetail.DetailParams
#[unity2::class(namespace = "", name = "BattleDetail.DetailParam")]
#[parent(BattleDetailCommandParam)]
pub struct BattleDetailDetailParam {
    #[rename(name = "m_Detail")]
    pub detail: BattleDetail,
}

#[unity2::class(namespace = "", name = "BattleDetail.UnitAttackParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailUnitAttackParam {}

#[unity2::class(namespace = "", name = "BattleDetail.UnitDefenseParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailUnitDefenseParam {}

#[unity2::class(namespace = "", name = "BattleDetail.AttackParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailAttackParam {}

#[unity2::class(namespace = "", name = "BattleDetail.DefenseParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailDefenseParam {}

#[unity2::class(namespace = "", name = "BattleDetail.HitParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailHitParam {
    #[rename(name = "m_InterferenceRod")]
    pub interference_rod: CalculatorCommand,
}

#[unity2::class(namespace = "", name = "BattleDetail.AvoidParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailAvoidParam {}

#[unity2::class(namespace = "", name = "BattleDetail.CriticalParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailCriticalParam {}

#[unity2::class(namespace = "", name = "BattleDetail.SecureParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailSecureParam {}

#[unity2::class(namespace = "", name = "BattleDetail.ContinuousParam")]
#[parent(BattleDetailDetailParam)]
pub struct BattleDetailContinuousParam {}

#[unity2::enumeration(namespace = "", name = "BattleDetail.BaseParams")]
#[repr(i32)]
pub enum BattleDetailBaseParams {
    BattleCount = 0,
    ActionCount = 1,
    AttackCount = 2,
    TerrainDefense = 3,
    TerrainAvoid = 4,
    SupportHit = 5,
    SupportAvoid = 6,
    SupportCritical = 7,
    SupportSecure = 8,
    WeaponAttack = 9,
    WeaponEfficacy = 10,
    WeaponHit = 11,
    WeaponAvoid = 12,
    WeaponCritical = 13,
    WeaponSecure = 14,
    WeaponWeight = 15,
    SkillCorrect = 16,
    GodSkillCorrect = 17,
    BlowRatio = 18,
    BlowDistance = 19,
    Num = 20,
}

#[unity2::enumeration(namespace = "", name = "BattleDetail.BattleParams")]
#[repr(i32)]
pub enum BattleDetailBattleParams {
    UnitAttack = 0,
    UnitDefense = 1,
    Attack = 2,
    Defense = 3,
    Hit = 4,
    Avoid = 5,
    Critical = 6,
    Secure = 7,
    Continuous = 8,
    SimplePower = 9,
    SimpleHit = 10,
    SimpleCritical = 11,
    Num = 12,
}