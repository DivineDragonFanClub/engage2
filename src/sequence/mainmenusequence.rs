#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ISingletonProcInst, ISingletonProcInstMethods, ProcInst, SingletonProcInst};

#[unity2::class(namespace = "App")]
#[parent(SingletonProcInst<MainMenuSequence>, ProcInst)]
pub struct MainMenuSequence {}
