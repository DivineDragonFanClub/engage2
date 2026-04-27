#![allow(unused_imports)]

use unity2::{Array, Cast, Class, ClassIdentity, FromIlInstance, Il2CppString, IlInstance, MethodInfo, OptionalMethod};

use crate::gamedata::item::ItemData;
use crate::gamedata::unit::Unit;
use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "MoonSharp.Interpreter")]
pub struct DynValue {}

#[unity2::methods]
impl DynValue {
    #[method(name = "NewBoolean", args = 1)]
    pub fn new_boolean(value: bool) -> DynValue;

    #[method(name = "NewNumber", args = 1)]
    pub fn new_number(value: f64) -> DynValue;

    #[method(name = "NewString", args = 1)]
    pub fn new_string(value: Il2CppString) -> DynValue;

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> i32;

    #[method(name = "get_Boolean", args = 0)]
    pub fn boolean(self) -> bool;

    #[method(name = "get_Number", args = 0)]
    pub fn number(self) -> f64;

    #[method(name = "get_String", args = 0)]
    pub fn string_value(self) -> Il2CppString;
}

#[unity2::class(namespace = "MoonSharp.Interpreter")]
pub struct Table {}

#[unity2::methods]
impl Table {
    #[method(offset = 0x2D26B40)]
    pub fn get(self, key: Il2CppString) -> DynValue;

    #[method(name = "Set", args = 2)]
    pub fn set(self, key: Il2CppString, value: DynValue);
}

#[unity2::class(namespace = "MoonSharp.Interpreter")]
pub struct Script {
    #[rename(name = "m_GlobalTable")]
    #[readonly]
    pub global_table: Table,
}

#[unity2::methods]
impl Script {
    #[method(name = "DoStream", args = 3)]
    pub fn do_stream(
        self,
        stream: IlInstance,
        global_context: IlInstance,
        code_friendly_name: Il2CppString,
    ) -> DynValue;
}

#[unity2::class(namespace = "System.IO")]
pub struct MemoryStream {}

#[unity2::methods]
impl MemoryStream {
    #[method(offset = 0x3371160)]
    pub fn ctor(self, buffer: Array<u8>);
}

#[unity2::class(namespace = "App")]
#[parent(Script)]
pub struct EventScript {
    #[backing]
    pub name: Il2CppString,
}

#[unity2::methods]
impl EventScript {
    #[method(name = "get_Instance", args = 0)]
    pub fn instance() -> EventScript;

    #[method(name = "Load", args = 1)]
    pub fn load(path: Il2CppString);

    #[method(name = "Unload", args = 0)]
    pub fn unload();

    #[method(name = "RegistAction", args = 2)]
    pub fn regist_action_raw(self, func: EventScriptActionArgs, name: Il2CppString);

    #[method(name = "RegistFunction", args = 2)]
    pub fn regist_function_raw(self, func: EventScriptFunctionArgs, name: Il2CppString);

    #[method(name = "Call", args = 2)]
    pub fn call(self, name: Il2CppString, args: Array<DynValue>);

    #[method(name = "GetFunc", args = 1)]
    pub fn get_func(self, name: Il2CppString) -> DynValue;
}

#[unity2::class(namespace = "App", name = "EventScript.ActionArgs")]
pub struct EventScriptActionArgs {
    #[rename(name = "method_ptr")]
    pub method_ptr: *mut u8,
    #[rename(name = "invoke_impl")]
    pub invoke_impl: *mut u8,
    #[rename(name = "m_target")]
    pub target: IlInstance,
    #[rename(name = "method")]
    pub method: *const MethodInfo,
}

impl EventScriptActionArgs {
    pub fn from_fn(callback: extern "C" fn(Array<DynValue>, OptionalMethod)) -> Option<Self> {
        let mi = clone_invoke_method_info(Self::class(), callback as *mut u8);
        let instance = <Self as FromIlInstance>::instantiate()?;
        instance.set_method_ptr(callback as *mut u8);
        instance.set_target(IlInstance::null());
        instance.set_method(mi as *const _);
        Some(instance)
    }
}

#[unity2::class(namespace = "App", name = "EventScript.FunctionArgs")]
pub struct EventScriptFunctionArgs {
    #[rename(name = "method_ptr")]
    pub method_ptr: *mut u8,
    #[rename(name = "invoke_impl")]
    pub invoke_impl: *mut u8,
    #[rename(name = "m_target")]
    pub target: IlInstance,
    #[rename(name = "method")]
    pub method: *const MethodInfo,
}

impl EventScriptFunctionArgs {
    pub fn from_fn(
        callback: extern "C" fn(Array<DynValue>, OptionalMethod) -> DynValue,
    ) -> Option<Self> {
        let mi = clone_invoke_method_info(Self::class(), callback as *mut u8);
        let instance = <Self as FromIlInstance>::instantiate()?;
        instance.set_method_ptr(callback as *mut u8);
        instance.set_target(IlInstance::null());
        instance.set_method(mi as *const _);
        Some(instance)
    }
}

fn clone_invoke_method_info(class: Class, method_ptr: *mut u8) -> &'static MethodInfo {
    use std::collections::HashMap;
    use std::sync::Mutex;

    static CACHE: Mutex<Option<HashMap<(usize, usize), &'static MethodInfo>>> = Mutex::new(None);

    let mut guard = CACHE.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);
    let key = (class.raw() as *const _ as usize, method_ptr as usize);
    map.entry(key).or_insert_with(|| {
        let invoke = class
            .raw()
            .get_method_from_name("Invoke", 1)
            .expect("delegate class missing `Invoke(args)` method");
        let mut cloned: MethodInfo = *invoke;
        cloned.method_ptr = method_ptr;
        Box::leak(Box::new(cloned))
    })
}

impl EventScript {
    pub fn register_action(
        self,
        name: impl Into<Il2CppString>,
        callback: extern "C" fn(Array<DynValue>, OptionalMethod),
    ) {
        let action_args =
            EventScriptActionArgs::from_fn(callback).expect("ActionArgs allocation failed");
        self.regist_action_raw(action_args, name.into());
    }

    pub fn register_function(
        self,
        name: impl Into<Il2CppString>,
        callback: extern "C" fn(Array<DynValue>, OptionalMethod) -> DynValue,
    ) {
        let function_args =
            EventScriptFunctionArgs::from_fn(callback).expect("FunctionArgs allocation failed");
        self.regist_function_raw(function_args, name.into());
    }
}

#[unity2::class(namespace = "App")]
pub struct ScriptSystem {}

#[unity2::class(namespace = "App")]
pub struct ScriptUtil {
    #[static_field]
    #[rename(name = "MAX_CURSOR_STACK")]
    pub max_cursor_stack: i32
}

#[unity2::methods]
impl ScriptUtil {
    #[method(name = "GetSequence", args = 0)]
    pub fn get_sequence() -> ProcInst;

    #[method(name = "Yield", args = 0)]
    pub fn r#yield();

    #[method(name = "TryGetInt", args = 3)]
    pub fn try_get_int(args: Array<DynValue>, index: i32, default: i32) -> i32;

    #[method(name = "TryGetString", args = 3)]
    pub fn try_get_string(
        args: Array<DynValue>,
        index: i32,
        default: Il2CppString,
    ) -> Il2CppString;

    #[method(name = "TryGetUnit", args = 3)]
    pub fn try_get_unit(args: Array<DynValue>, index: i32, warning: bool) -> Unit;

    #[method(name = "TryGetItem", args = 3)]
    pub fn try_get_item(args: Array<DynValue>, index: i32, warning: bool) -> ItemData;

    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(args: Array<DynValue>, index: i32) -> DynValue;

    #[method(name = "TryGetType", args = 2)]
    pub fn try_get_type(args: Array<DynValue>, index: i32) -> i32;
}

pub trait DynValueArgs {
    fn try_get_i32(self, index: i32) -> i32;
    fn try_get_string(self, index: i32) -> Il2CppString;
    fn try_get_unit(self, index: i32) -> Unit;
    fn try_get_item(self, index: i32) -> ItemData;
}

impl DynValueArgs for Array<DynValue> {
    fn try_get_i32(self, index: i32) -> i32 {
        ScriptUtil::try_get_int(self, index, i32::MAX)
    }

    fn try_get_string(self, index: i32) -> Il2CppString {
        ScriptUtil::try_get_string(self, index, Il2CppString::from_il_instance(IlInstance::null()))
    }

    fn try_get_unit(self, index: i32) -> Unit {
        ScriptUtil::try_get_unit(self, index, true)
    }

    fn try_get_item(self, index: i32) -> ItemData {
        ScriptUtil::try_get_item(self, index, true)
    }
}

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct EventSequence {}

#[unity2::methods]
impl EventSequence {
    #[method(name = "AddCoroutine", args = 2)]
    pub fn add_coroutine(self, func: DynValue, args: Array<DynValue>) -> EventSequence;
}
