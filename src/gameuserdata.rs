#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::data::{ISingletonClass, ISingletonClassMethods, SingletonClass};
use crate::gamevariable::GameVariable;

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<GameUserData>)]
pub struct GameUserData {}

#[unity2::methods]
impl GameUserData {
    #[method(name = "get_Variable")]
    fn get_variable(self) -> GameVariable;

    #[method(name = "get_Sequence")]
    fn get_sequence_raw(self) -> i32;
}

impl GameUserData {
    pub fn get_sequence() -> i32 {
        GameUserData::instance().get_sequence_raw()
    }
}
