#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{
    IProcInst, IProcSceneSequence, IProcSceneSequenceMethods, ISingletonProcInst,
    ISingletonProcInstMethods, ProcInst, ProcSceneSequence, SingletonProcInst,
};

#[unity2::class(namespace = "App")]
#[parent(ProcSceneSequence<MapSequence>)]
pub struct MapSequence {}
