use crate::{inspector::PokeInspector, map::mapoverlap::MapOverlapData};


#[unity2::class(namespace = "App")]
pub struct BattleDestory {
    pub inspector: PokeInspector,
    pub overlap: MapOverlapData,
}