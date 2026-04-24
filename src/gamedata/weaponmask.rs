#![allow(unused_imports)]

use unity2::ClassIdentity;

#[unity2::class(namespace = "App")]
pub struct WeaponMask {
    #[rename(name = "m_Value")]
    pub value: i32,
}
