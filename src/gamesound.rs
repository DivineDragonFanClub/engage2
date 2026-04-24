#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::combat::Character;

#[unity2::class(namespace = "App")]
pub struct GameSound {}

#[unity2::methods]
impl GameSound {
    #[method(name = "PostEvent", args = 2)]
    pub fn post_event(event_name: Il2CppString, character: Character) -> unity2::IlInstance;

    #[method(name = "IsEventLoaded", args = 1)]
    pub fn is_event_loaded(event_name: Il2CppString) -> bool;
}
