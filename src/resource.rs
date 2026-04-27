use unity2::Class;
use unity2::ClassIdentity;
use unity2::Il2CppString;
use unity2::engine::GameObject;
use unity2::engine::object::Object;
use unity2::engine::object::IObject;
use unity2::engine::object::Component;
use unity2::engine::object::IComponent;
use unity2::engine::object::Behaviour;
use unity2::engine::object::IBehaviour;
use unity2::engine::object::MonoBehaviour;
use unity2::engine::object::IMonoBehaviour;

use crate::ProcInst;
use crate::filesystem::BindHolder;


#[unity2::class(namespace = "App")]
#[parent(unity2::engine::object::MonoBehaviour)]
pub struct ResourceObject {
    #[rename(name = "m_Handle")]
    pub handle: ResourceGameObject,
    #[rename(name = "m_StartCallback")]
    pub start_callback: ResourceObjectCallback,
    #[rename(name = "m_TickCallback")]
    pub tick_callback: ResourceObjectCallback,
    #[rename(name = "m_EndCallback")]
    pub end_callback: ResourceObjectCallback,
    #[rename(name = "m_Coroutine")]
    pub coroutine: ResourceObjectCoroutine,
    #[rename(name = "m_BindProc")]
    pub bind_proc: ProcInst,
    #[rename(name = "m_BindHolder")]
    pub bind_holder: BindHolder,
    #[rename(name = "m_Sequence")]
    pub sequence: ResourceObjectSequence,
    #[rename(name = "m_Flags")]
    pub flags: ResourceObjectFlags,
    #[rename(name = "m_DelayTime")]
    pub delay_time: f32,
    #[rename(name = "m_BindTime")]
    pub bind_time: f32,
    #[rename(name = "m_LifeTime")]
    pub life_time: f32,
    #[rename(name = "m_SoundLabel")]
    pub sound_label: Il2CppString,
}

#[unity2::class(namespace = "App")]
pub struct ResourceHandle {
    #[rename(name = "m_Path")]
    pub path: Il2CppString,
}

#[unity2::class(namespace = "App", name = "TResourceHandle`1")]
pub struct TResourceHandle<T: ClassIdentity> {}

impl<T: ClassIdentity> IResourceHandle for TResourceHandle<T> {}

#[unity2::class(namespace = "App")]
#[parent(TResourceHandle<GameObject>)]
pub struct ResourceGameObject {}

#[unity2::class(namespace = "", name = "ResourceObject.Callback")]
pub struct ResourceObjectCallback {}

#[unity2::class(namespace = "", name = "ResourceObject.Coroutine")]
pub struct ResourceObjectCoroutine {}

#[unity2::enumeration(namespace = "", name = "ResourceObject.Sequence")]
#[repr(i32)]
pub enum ResourceObjectSequence {
    Start = 0,
    Tick = 1,
    End = 2,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ResourceObjectFlags: i32 {
        const DoneStart = 1 << 0;
        const DoneEnd = 1 << 1;
        const Endless = 1 << 2;
        const Binding = 1 << 3;
        const CanSkip = 1 << 4;
    }
}

impl ClassIdentity for ResourceObjectFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "ResourceObject.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}