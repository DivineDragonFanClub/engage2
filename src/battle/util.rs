use unity2::system::collections::Stack;

use crate::battle::calculator::BattleCalculator;
use crate::battle::info::BattleInfo;


#[unity2::class(namespace = "App")]
pub struct BattleUtil {}

#[unity2::class(namespace = "", name = "BattleUtil.CalcScope")]
pub struct BattleUtilCalcScope {
    #[static_field]
    #[rename(name = "s_Stack")]
    pub stack: Stack<BattleCalculator>,

    pub info: BattleInfo,
    pub calc: BattleCalculator,
}

#[unity2::enumeration(namespace = "App", name = "AttackType")]
#[repr(i32)]
pub enum AttackType {
    Slash = 0,
    Sting = 1,
    Blow = 2,
    Num = 3,
}

#[unity2::enumeration(namespace = "App", name = "DamageLevel")]
#[repr(i32)]
pub enum DamageLevel {
    None = 0,
    Low = 1,
    Middle = 2,
    High = 3,
    Efficacy = 4,
    Critical = 5,
    Num = 6,
}