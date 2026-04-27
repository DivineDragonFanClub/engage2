use unity2::{Array, Il2CppString};


#[unity2::class(namespace = "App")]
pub struct UnitAnim {
    #[static_field]
    #[rename(name = "s_StateNames")]
    pub state_names: Array<Il2CppString>,
}


#[unity2::enumeration(namespace = "", name = "UnitAnim.Times")]
#[repr(i32)]
pub enum UnitAnimTimes {
    Zero = 0,
    Slow = 1,
    Normal = 2,
    Fast = 3,
    VeryFast = 4,
    VerySlow = 5,
}

#[unity2::enumeration(namespace = "", name = "UnitAnim.Types")]
#[repr(i32)]
pub enum UnitAnimTypes {
    None = 0,
    StandBy = 1,
    IdleRelax = 2,
    IdleNormal = 3,
    RunLoop = 4,
    Start = 5,
    Attack = 6,
    Shoot = 7,
    Special = 8,
    Rod = 9,
    Dance = 10,
    MagicWeapon = 11,
    Event1 = 12,
    Event2 = 13,
    Event3 = 14,
    Event4 = 15,
    Num = 16,
}