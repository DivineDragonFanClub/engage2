#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::ProcInst;

#[unity2::class(namespace = "App")]
pub struct Database {}

#[unity2::methods]
impl Database {
    #[method(name = "Load", args = 0)]
    pub fn load();

    #[method(name = "Completed", args = 0)]
    pub fn completed();

    #[method(name = "IsCompleted", args = 0)]
    pub fn is_completed() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    #[method(name = "Reload", args = 1)]
    pub fn reload(super_: ProcInst);

    #[method(name = "Release", args = 0)]
    pub fn release();
}
