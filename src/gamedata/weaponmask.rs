#![allow(unused_imports)]

use unity2::{Class, ClassIdentity};

#[unity2::class(namespace = "App")]
pub struct WeaponMask {
    #[rename(name = "m_Value")]
    pub value: i32,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeaponMaskFlag: i32 {
        const None = 1 << 0;
        const Sword = 1 << 1;
        const Lance = 1 << 2;
        const Axe = 1 << 3;
        const Bow = 1 << 4;
        const Dagger = 1 << 5;
        const Magic = 1 << 6;
        const Rod = 1 << 7;
        const Fist = 1 << 8;
        const Special = 1 << 9;
    }
}

impl ClassIdentity for WeaponMaskFlag {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "WeaponMask.Flag";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}
