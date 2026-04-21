#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use super::god::GodUnit;
use super::job::JobData;
use crate::data::PersonData;

#[unity2::class(namespace = "App")]
pub struct Unit {}

#[unity2::methods]
impl Unit {
    #[method(name = "get_Person")]
    fn person(self) -> PersonData;

    #[method(name = "get_Job")]
    fn job(self) -> JobData;

    #[method(name = "get_GodUnit")]
    fn god_unit(self) -> GodUnit;

    #[method(name = "get_Hp")]
    fn hp(self) -> i32;

    #[method(name = "get_Level")]
    fn level(self) -> u8;

    #[method(name = "get_Ident")]
    fn ident(self) -> i32;
}
