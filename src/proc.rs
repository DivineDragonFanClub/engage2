#![allow(unused_imports)]

use unity2::{Cast, Class, ClassIdentity, FromIlInstance, Il2CppString, Array, IlInstance, MethodInfo};

#[unity2::class(namespace = "App")]
pub struct ProcInst {
    #[rename(name = "m_Descs")]
    #[readonly]
    pub descs: Array<ProcDesc>,
    #[rename(name = "m_DescIndex")]
    pub desc_index: i32,
    #[rename(name = "m_Name")]
    pub name: Il2CppString,
    #[rename(name = "m_HashCode")]
    pub hashcode: i32,
    #[rename(name = "m_Super")]
    #[readonly]
    pub super_: ProcInst,
    #[rename(name = "m_Child")]
    #[readonly]
    pub child: ProcInst,
    #[rename(name = "m_Prev")]
    #[readonly]
    pub prev: ProcInst,
    #[rename(name = "m_Next")]
    #[readonly]
    pub next: ProcInst,
    #[rename(name = "m_Persistent")]
    #[readonly]
    pub persistent: ProcVoidMethod,
    #[rename(name = "m_State")]
    pub state: i32,
    #[rename(name = "m_Suspend")]
    pub suspend: i32,
    #[rename(name = "m_WaitTime")]
    pub wait_time: f32,
    #[rename(name = "m_TickTime")]
    pub tick_time: f32,
    #[rename(name = "m_Stack")]
    #[readonly]
    pub stack: IlInstance,
}

#[unity2::methods]
impl ProcInst {
    #[method(name = "CreateInternal")]
    fn create_internal(
        self,
        parent: ProcInst,
        desc: Array<ProcDesc>,
        name: Il2CppString,
        is_bind: bool,
    ) -> ProcInst;
}

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct BasicMenu {}

#[unity2::class(namespace = "App")]
pub struct ProcVoidMethod {
    #[readonly]
    pub method_ptr: *mut u8,
    #[rename(name = "m_target")]
    #[readonly]
    pub target: IlInstance,
    #[readonly]
    pub method_info: *const MethodInfo,
}

#[unity2::methods]
impl ProcVoidMethod {
    #[method(name = "Invoke")]
    fn invoke(self);

    #[method(name = ".ctor", args = 2)]
    fn ctor(self, target: IlInstance, method_info: *const MethodInfo);
}

impl ProcVoidMethod {
    pub fn from_raw_parts(
        target: IlInstance,
        method_info: &'static MethodInfo,
    ) -> Option<Self> {
        let instance = <Self as FromIlInstance>::instantiate()?;
        instance.ctor(target, method_info as *const _);
        Some(instance)
    }

    pub fn from_fn(
        target: IlInstance,
        callback: extern "C" fn(IlInstance, Option<&'static MethodInfo>),
    ) -> Option<Self> {
        let mi: &'static mut MethodInfo = Box::leak(Box::new(MethodInfo::new()));
        mi.method_ptr = callback as *mut u8;
        mi.parameters_count = 0;
        Self::from_raw_parts(target, mi)
    }
}

#[unity2::enumeration(namespace = "", name = "MainSequence.Label")]
#[repr(i32)]
pub enum MainSequenceLabel {
    None = 0,
    Startup = 1,
    TitleLoop = 2,
    TitleLoopFromMainMenu = 3,
    MainMenu = 4,
    Chapter = 5,
    Gmap = 6,
    Kizuna = 7,
    Hub = 8,
    HubToSavePosition = 9,
    Ending = 10,
    NextChapter = 11,
    Map = 12,
    Complete = 13,
    GameOver = 14,
    ChapterSave = 15,
    AfterChapterSave = 16,
    SetSaveDataLoadTarget = 17,
    SaveDataLoad = 18,
    SaveDataLoadFailed = 19,
    SaveDataVersionFailed = 20,
    DataLoadFailed = 21,
    AfterLoadFailed = 22,
    ContentsResume = 23,
    RelayDebug = 24,
    Relay = 25,
    Versus = 26,
    Challenge = 27,
    BackToTitle = 28,
    End = 29,
}

#[unity2::enumeration]
#[repr(i32)]
pub enum ProcDescType {
    End = 0,
    Halt = 1,
    Jump = 2,
    Label = 3,
    Push = 4,
    Pop = 5,
    Persistent = 6,
    WaitTime = 7,
    WaitFrame = 8,
    Yield = 9,
    Call = 10,
    Tick = 11,
    Args = 12,
    WaitFunc = 13,
    JumpFunc = 14,
    User = 15,
    Max = 16,
}

#[unity2::class(namespace = "App")]
pub struct ProcDesc {
    #[rename(name = "m_Type")]
    pub desc_type: ProcDescType,
}

// App.Proc is a static-only utility class, Jump(int) and Jump(string) collide under name lookup, so
// they fall back to raw offsets from dump.cs, the documented escape hatch for overload ambiguity
#[unity2::class(namespace = "App")]
pub struct Proc {
    #[static_field]
    #[rename(name = "s_Roots")]
    pub roots: Array<ProcInst>,
}

#[unity2::methods]
impl Proc {
    #[method(name = "Initialize")]
    fn initialize();

    #[method(name = "Sweep")]
    fn sweep();

    #[method(name = "FindByName")]
    fn find_by_name(name: Option<&Il2CppString>) -> ProcInst;

    #[method(name = "KillByName")]
    fn kill_by_name(name: Option<&Il2CppString>);

    #[method(name = "KillByBind")]
    fn kill_by_bind(bound: ProcInst);

    #[method(name = "GetRoot")]
    fn get_root(root_type: i32) -> ProcInst;

    #[method(name = "GetRootHi")]
    fn get_root_hi() -> ProcInst;

    #[method(name = "GetRootDef")]
    fn get_root_def() -> ProcInst;

    #[method(name = "GetRootLow")]
    fn get_root_low() -> ProcInst;

    #[method(name = "End")]
    fn end() -> ProcDesc;

    #[method(name = "Halt")]
    fn halt() -> ProcDesc;

    #[method(offset = 0x281AD90)]
    fn jump_int(label: i32) -> ProcDesc;

    #[method(offset = 0x281AE30)]
    fn jump_string(label: Il2CppString) -> ProcDesc;

    #[method(name = "Label")]
    fn label(label: i32) -> ProcDesc;
}
