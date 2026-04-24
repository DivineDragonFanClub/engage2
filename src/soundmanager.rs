#![allow(unused_imports)]

use unity2::system::collections::{IListMethods, List};
use unity2::{object_get_class, Cast, ClassIdentity, Il2CppString, SystemObject};

use crate::combat::Character;

#[unity2::class(namespace = "App", name = "SoundSystem.SoundHandle")]
pub struct SoundHandle {}

impl SoundHandle {
    pub fn get_event_name(self) -> Il2CppString {
        let class = object_get_class(self);
        let entry = class
            .get_virtual_method("GetEventName")
            .expect("SoundHandle vtable missing `GetEventName`");
        let f: extern "C" fn(Self, &'static unity2::MethodInfo) -> Il2CppString =
            unsafe { std::mem::transmute(entry.method_ptr) };
        f(self, entry.method_info)
    }
}

#[unity2::class(namespace = "App")]
pub struct SoundManager {
    #[rename(name = "m_soundHandleList")]
    #[readonly]
    pub sound_handle_list: List<SoundHandle>,
}

#[unity2::methods]
impl SoundManager {
    #[method(name = "get_Instance", args = 0)]
    pub fn instance() -> SoundManager;

    #[method(name = "PostEvent", args = 3)]
    pub fn post_event(
        self,
        event_name: Il2CppString,
        character: Character,
        is_get_position: bool,
    ) -> SoundHandle;
}

impl SoundManager {
    pub fn is_event_playing_with_prefix(self, prefix: impl AsRef<str>) -> bool {
        let prefix = prefix.as_ref();
        self.sound_handle_list().iter().any(|handle| {
            !handle.is_null() && handle.get_event_name().to_string().starts_with(prefix)
        })
    }
}
