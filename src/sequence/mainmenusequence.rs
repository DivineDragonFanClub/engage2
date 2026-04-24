#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ISingletonProcInst, ISingletonProcInstMethods, ProcInst, SingletonProcInst};

#[unity2::class(namespace = "App")]
#[parent(SingletonProcInst<MainMenuSequence>)]
pub struct MainMenuSequence {
    #[rename(name = "m_NextSequence")]
    pub next_sequence: i32,
}

#[unity2::methods]
impl MainMenuSequence {
    #[method(name = "JumpToNextSequence", args = 0)]
    pub fn jump_to_next_sequence(self);
}
