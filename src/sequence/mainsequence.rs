#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct MainSequence {}
