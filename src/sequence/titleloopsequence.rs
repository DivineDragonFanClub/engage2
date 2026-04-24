#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct TitleLoopSequence {}

#[unity2::enumeration(namespace = "App", name = "TitleLoopSequence.Label")]
#[repr(i32)]
pub enum TitleLoopSequenceLabel {
    Start = 0,
    StartFromMainMenu = 1,
    GrandOpening = 2,
    Title = 3,
    TitleFromMainMenu = 4,
    JobIntro = 5,
    End = 6,
}
