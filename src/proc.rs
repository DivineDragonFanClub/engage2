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

    #[method(offset = 0x280A6B0)]
    pub fn create_bind(
        self,
        super_: ProcInst,
        descs: Array<ProcDesc>,
        name: Il2CppString,
    ) -> ProcInst;
}

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
        callback: extern "C" fn(IlInstance, unity2::OptionalMethod),
    ) -> Option<Self> {
        let mi = unity2::method_info_for_fn(callback as *mut u8, 0);
        Self::from_raw_parts(target, mi)
    }
}

#[unity2::class(namespace = "App")]
pub struct ProcVoidFunction {
    #[readonly]
    pub method_ptr: *mut u8,
    #[rename(name = "m_target")]
    #[readonly]
    pub target: IlInstance,
    #[readonly]
    pub method_info: *const MethodInfo,
}

#[unity2::methods]
impl ProcVoidFunction {
    #[method(name = "Invoke", args = 1)]
    fn invoke(self, inst: ProcInst);

    #[method(name = ".ctor", args = 2)]
    fn ctor(self, target: IlInstance, method_info: *const MethodInfo);
}

impl ProcVoidFunction {
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
        callback: extern "C" fn(ProcInst, unity2::OptionalMethod),
    ) -> Option<Self> {
        let mi = unity2::method_info_for_fn(callback as *mut u8, 1);
        Self::from_raw_parts(target, mi)
    }
}

#[unity2::class(namespace = "App")]
pub struct ProcBoolMethod {
    #[readonly]
    pub method_ptr: *mut u8,
    #[rename(name = "m_target")]
    #[readonly]
    pub target: IlInstance,
    #[readonly]
    pub method_info: *const MethodInfo,
}

#[unity2::methods]
impl ProcBoolMethod {
    #[method(name = "Invoke")]
    fn invoke(self) -> bool;

    #[method(name = ".ctor", args = 2)]
    fn ctor(self, target: IlInstance, method_info: *const MethodInfo);
}

impl ProcBoolMethod {
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
        callback: extern "C" fn(IlInstance, unity2::OptionalMethod) -> bool,
    ) -> Option<Self> {
        let mi = unity2::method_info_for_fn(callback as *mut u8, 0);
        Self::from_raw_parts(target, mi)
    }
}

#[unity2::enumeration(namespace = "App", name = "MainSequence.Label")]
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

impl ProcDesc {
    pub fn get_label(self) -> i32 {
        let class = unity2::object_get_class(self);
        let entry = class
            .get_virtual_method("get_Label")
            .expect("ProcDesc vtable missing `get_Label`");
        let f: extern "C" fn(Self, &'static unity2::MethodInfo) -> i32 =
            unsafe { std::mem::transmute(entry.method_ptr) };
        f(self, entry.method_info)
    }

    #[inline]
    pub fn get_desc_type_method(self) -> ProcDescType {
        IProcDesc::desc_type(self)
    }
}

#[unity2::class(namespace = "App")]
pub struct Proc {
    #[static_field]
    #[rename(name = "s_Roots")]
    pub roots: Array<ProcInst>,
}

#[unity2::methods]
impl Proc {
    #[method(name = "Initialize")]
    pub fn initialize();

    #[method(name = "Sweep")]
    pub fn sweep();

    #[method(name = "FindByName")]
    pub fn find_by_name(name: Option<&Il2CppString>) -> ProcInst;

    #[method(name = "KillByName")]
    pub fn kill_by_name(name: Option<&Il2CppString>);

    #[method(name = "KillByBind")]
    pub fn kill_by_bind(bound: ProcInst);

    #[method(name = "GetRoot")]
    pub fn get_root(root_type: i32) -> ProcInst;

    #[method(name = "GetRootHi")]
    pub fn get_root_hi() -> ProcInst;

    #[method(name = "GetRootDef")]
    pub fn get_root_def() -> ProcInst;

    #[method(name = "GetRootLow")]
    pub fn get_root_low() -> ProcInst;

    #[method(name = "End")]
    pub fn end() -> ProcDesc;

    #[method(name = "Halt")]
    pub fn halt() -> ProcDesc;

    #[method(offset = 0x281AD90)]
    pub fn jump_int(label: i32) -> ProcDesc;

    #[method(offset = 0x281AE30)]
    pub fn jump_string(label: Il2CppString) -> ProcDesc;

    #[method(offset = 0x281AEB0)]
    pub fn label(label: i32) -> ProcDesc;

    #[method(offset = 0x280A7A0)]
    pub fn call_method(method: ProcVoidMethod) -> ProcDesc;

    #[method(offset = 0x281B250)]
    pub fn call_function(function: ProcVoidFunction) -> ProcDesc;

    #[method(offset = 0x280A840)]
    pub fn wait_while_true(method: ProcBoolMethod) -> ProcDesc;

    #[method(name = "WaitTime", args = 1)]
    pub fn wait_time(seconds: f32) -> ProcDesc;

    #[method(name = "Vsync", args = 1)]
    pub fn vsync(mode: i32) -> ProcDesc;

    #[method(name = "WaitIsLoading", args = 0)]
    pub fn wait_is_loading() -> ProcDesc;
}

#[unity2::class(namespace = "App", name = "SingletonProcInst`1")]
#[parent(ProcInst)]
pub struct SingletonProcInst<T: ClassIdentity> {}

#[unity2::methods]
impl<T: ClassIdentity> SingletonProcInst<T> {
    #[method(name = "get_Instance")]
    pub fn instance() -> T;
}

#[unity2::class(namespace = "App", name = "ProcSceneSequence`1")]
#[parent(SingletonProcInst<T>, ProcInst)]
pub struct ProcSceneSequence<T: ClassIdentity> {}

#[unity2::methods]
impl<T: ClassIdentity> ProcSceneSequence<T> {
    #[method(name = "get_SceneName")]
    pub fn scene_name(self) -> Il2CppString;

    #[method(name = "set_SceneName")]
    pub fn set_scene_name(self, value: Il2CppString);
}

/// Patches a proc's vanilla ProcDesc array by injecting entries at specific positions
///
/// Insertions applied in descending position order so callers don't reason about index shift
pub struct ProcDescPatch {
    original: Vec<ProcDesc>,
    patches: Vec<(usize, Vec<ProcDesc>)>,
}

impl ProcDescPatch {
    pub fn new(original: Array<ProcDesc>) -> Self {
        Self {
            original: original.iter().collect(),
            patches: Vec::new(),
        }
    }

    /// Inject `descs` starting at `position` in the original array
    pub fn insert(mut self, position: usize, descs: impl IntoIterator<Item = ProcDesc>) -> Self {
        self.patches.push((position, descs.into_iter().collect()));
        self
    }

    /// Apply every queued insertion and materialize the patched array
    pub fn finish(mut self) -> Array<ProcDesc> {
        self.patches.sort_by(|a, b| b.0.cmp(&a.0));
        for (pos, descs) in self.patches {
            self.original.splice(pos..pos, descs);
        }
        Array::<ProcDesc>::from_slice(&self.original)
            .expect("ProcDescPatch::finish: ProcDesc array allocation failed")
    }
}
