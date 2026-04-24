#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct HubSequence {}

#[unity2::methods]
impl HubSequence {
    #[method(name = "GiftGet", args = 2)]
    pub fn gift_get(self, reward_id: Il2CppString, message_id: Il2CppString);
}
