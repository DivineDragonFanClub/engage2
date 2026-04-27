use crate::{resource::ResourceObject, unitpool::ForceType};


#[unity2::class(namespace = "", name = "MapOverlap.Data")]
pub struct MapOverlapData {
    #[backing]
    pub x: i32,
    #[backing]
    pub z: i32,
    #[backing]
    pub index: i32,
    #[backing]
    pub hp: i32,
    #[backing]
    pub life: i32,
    #[backing]
    pub turn: i32,
    #[backing]
    pub phase: ForceType,
    #[backing]
    pub effect: ResourceObject,
}