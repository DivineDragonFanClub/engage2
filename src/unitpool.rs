#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::gamedata::unit::{IUnit, IUnitMethods, Unit};

// Nested enum, namespace is outer's namespace
#[unity2::enumeration(namespace = "App", name = "Force.Type")]
#[repr(i32)]
pub enum ForceType {
    Player = 0,
    Enemy = 1,
    Ally = 2,
    Absent = 3,
    Dead = 4,
    Lost = 5,
    Temporary = 6,
    Empty = 7,
}

#[unity2::class(namespace = "App")]
pub struct Force {
    #[rename(name = "m_Head")]
    #[readonly]
    pub head: Unit,
    #[rename(name = "m_Tail")]
    #[readonly]
    pub tail: Unit,
    #[rename(name = "m_Type")]
    #[readonly]
    pub ty: ForceType,
}

#[unity2::methods]
impl Force {
    #[method(name = "get_First", args = 0)]
    pub fn first(self) -> Unit;

    #[method(name = "get_Last", args = 0)]
    pub fn last(self) -> Unit;
}

impl Force {
    pub fn iter(self) -> ForceIter {
        ForceIter { current: self.first() }
    }
}

pub struct ForceIter {
    current: Unit,
}

impl Iterator for ForceIter {
    type Item = Unit;

    fn next(&mut self) -> Option<Unit> {
        use unity2::Cast;
        if self.current.is_null() {
            return None;
        }
        let unit = self.current;
        self.current = unit.next();
        Some(unit)
    }
}

#[unity2::class(namespace = "App")]
pub struct UnitPool {}

#[unity2::methods]
impl UnitPool {
    #[method(name = "GetForce", args = 1)]
    pub fn get_force(index: i32) -> Force;

    #[method(name = "Get", args = 1)]
    pub fn get(index: i32) -> Unit;

    #[method(name = "GetHero", args = 1)]
    pub fn get_hero(consider_relay: bool) -> Unit;
}
