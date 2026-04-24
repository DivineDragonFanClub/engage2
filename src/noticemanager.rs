#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

use crate::mess::Mess;

#[unity2::enumeration(namespace = "App", name = "NoticeManager.Kinds")]
#[repr(i32)]
pub enum NoticeKind {
    None = 0,
    Facility = 1,
    Kizuna = 2,
    Tutorial = 3,
    Notebook = 4,
    RingList = 5,
}

#[unity2::class(namespace = "App")]
pub struct NoticeManager {}

#[unity2::methods]
impl NoticeManager {
    #[method(name = "Add", args = 2)]
    fn add_raw(kind: NoticeKind, text: Il2CppString);
}

impl NoticeManager {
    pub fn add(text: impl Into<Il2CppString>) {
        Self::add_raw(NoticeKind::Facility, text.into());
    }

    pub fn add_by_mid(label: impl Into<Il2CppString>) {
        Self::add_raw(NoticeKind::Facility, Mess::get(label));
    }
}
