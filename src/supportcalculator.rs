use unity2::Array;

use crate::Unit;
use crate::map::mapfor::MapForRangeFunction;
use crate::gamedata::reliance::RelianceDataLevel;


#[unity2::class(namespace = "App")]
pub struct SupportCalculator {
    #[rename(name = "m_Unit")]
    pub unit: Unit,
    #[rename(name = "m_X")]
    pub x: i32,
    #[rename(name = "m_Z")]
    pub z: i32,
    #[rename(name = "m_Hit")]
    pub hit: i32,
    #[rename(name = "m_Avoid")]
    pub avoid: i32,
    #[rename(name = "m_Critical")]
    pub critical: i32,
    #[rename(name = "m_Secure")]
    pub secure: i32,
    #[rename(name = "m_ShowUnits")]
    pub show_units: Array<Unit>,
    #[rename(name = "m_ShowUnitLevels")]
    pub show_unit_levels: Array<RelianceDataLevel>,
    #[rename(name = "m_RangeFunction")]
    pub range_function: MapForRangeFunction,
}

impl SupportCalculator {
    pub const RANGE: i32 = 1;
    pub const MAX_SHOW_UNITS: i32 = 4;
}