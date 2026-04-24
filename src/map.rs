#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::data::{ISingletonClass, ISingletonClassMethods, SingletonClass};

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<MapImage>)]
pub struct MapImage {
    #[backing(name = "PlayAreaX")]
    pub playarea_x: i32,
    #[backing(name = "PlayAreaZ")]
    pub playarea_z: i32,
    #[backing(name = "PlayAreaX1")]
    pub playarea_x1: i32,
    #[backing(name = "PlayAreaZ1")]
    pub playarea_z1: i32,
}

#[unity2::methods]
impl MapImage {
    #[method(name = "SetSize", args = 2)]
    pub fn set_size(self, w: i32, h: i32);

    #[method(name = "get_X", args = 0)]
    pub fn x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn z(self) -> i32;

    #[method(name = "get_W", args = 0)]
    pub fn w(self) -> i32;

    #[method(name = "get_H", args = 0)]
    pub fn h(self) -> i32;
}
