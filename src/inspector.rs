use unity2::{Array, Il2CppString};

use crate::script::DynValue;
use crate::script::ScriptUtil;
use crate::script::IScriptUtil;


#[unity2::class(namespace = "App")]
#[parent(ScriptUtil)]
pub struct MapInspector {
    #[rename(name = "m_Kind")]
    pub kind: MapInspectorKind,
    #[rename(name = "m_Condition")]
    pub condition: DynValue,
    #[rename(name = "m_Function")]
    pub function: DynValue,
    #[rename(name = "m_Args")]
    pub args: Array<DynValue>,
}

#[unity2::class(namespace = "App")]
#[parent(MapInspector)]
pub struct PokeInspector {
    #[rename(name = "m_X")]
    pub x: i32,
    #[rename(name = "m_Z")]
    pub z: i32,
    #[rename(name = "m_W")]
    pub w: i32,
    #[rename(name = "m_H")]
    pub h: i32,
    #[rename(name = "m_MaxHp")]
    pub max_hp: i32,
    #[rename(name = "m_Person")]
    pub person: i32,
    #[rename(name = "m_HpLabel")]
    pub hp_label: Il2CppString,
}

#[unity2::enumeration(namespace = "", name = "MapInspector.Kind")]
#[repr(i32)]
pub enum MapInspectorKind {
    None = 0,
    Turn = 1,
    TurnAfter = 2,
    TurnEnd = 3,
    Area = 4,
    Tbox = 5,
    Door = 6,
    Torch = 7,
    Visit = 8,
    Escape = 9,
    Destroy = 10,
    Breakdown = 11,
    BreakdownEnemy = 12,
    Waypoint = 13,
    Command = 14,
    Die = 15,
    ReviveBefore = 16,
    ReviveAfter = 17,
    Fixed = 18,
    Talk = 19,
    BattleBefore = 20,
    BattleTalk = 21,
    BattleAfter = 22,
    Pickup = 23,
    TargetSelect = 24,
    UnitCommandPrepare = 25,
    UnitCommandInterrupt = 26,
    EngageBefore = 27,
    EngageAfter = 28,
    Cannon = 29,
    HelpSpot = 30,
    Num = 31,
}