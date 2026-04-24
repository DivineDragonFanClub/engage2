#![allow(unused_imports)]

use unity2::ClassIdentity;

#[unity2::class(namespace = "App")]
pub struct ResourceManager {}

#[unity2::methods]
impl ResourceManager {
    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;
}
