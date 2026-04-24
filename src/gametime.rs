#![allow(unused_imports)]

use unity2::ClassIdentity;

#[unity2::class(namespace = "App")]
pub struct GameTime {}

#[unity2::methods]
impl GameTime {
    #[method(name = "get_Time", args = 0)]
    pub fn get_time() -> f32;
}

pub fn get_time() -> f32 {
    GameTime::get_time()
}
