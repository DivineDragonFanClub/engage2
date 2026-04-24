#![allow(unused_imports)]

use unity2::ClassIdentity;

#[unity2::class(namespace = "App")]
pub struct DLCManager {}

#[unity2::methods]
impl DLCManager {
    #[method(name = "HasContent", args = 1)]
    pub fn has_content(content: i32) -> bool;
}
