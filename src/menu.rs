#![allow(unused_imports)]

use unity2::system::collections::{IListMethods, List};
use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
pub struct BasicMenuContent {}

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct BasicMenu {
    #[rename(name = "m_fullMenuItemList")]
    #[readonly]
    pub full_menu_item_list: List<BasicMenuItem>,
    #[rename(name = "m_menuItemList")]
    #[readonly]
    pub menu_item_list: List<BasicMenuItem>,
    #[rename(name = "m_reservedShowRowNum")]
    pub reserved_show_row_num: i32,
}

impl BasicMenu {
    pub fn add_item(self, item: impl Into<BasicMenuItem>) {
        self.full_menu_item_list().add(item.into());
    }
}

#[unity2::class(namespace = "App")]
pub struct BasicMenuItem {
    #[rename(name = "m_menu")]
    #[readonly]
    pub menu: BasicMenu,
    #[rename(name = "m_name")]
    pub name: unity2::Il2CppString,
    #[rename(name = "m_index")]
    #[readonly]
    pub index: i32,
    #[rename(name = "m_fullIndex")]
    #[readonly]
    pub full_index: i32,
}

#[unity2::methods]
impl BasicMenuItem {
    #[method(name = ".ctor", args = 0)]
    fn ctor(self);
}

use crate::configmenu::BasicMenuItemAttribute;

pub trait BasicMenuItemMethods {
    extern "C" fn get_name(
        this: BasicMenuItem,
        method_info: unity2::OptionalMethod,
    ) -> unity2::Il2CppString;
    extern "C" fn a_call(
        _this: BasicMenuItem,
        _method_info: unity2::OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new()
    }
    extern "C" fn b_call(
        _this: BasicMenuItem,
        _method_info: unity2::OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true).with_se_cancel(true)
    }
    extern "C" fn build_attribute(
        _this: BasicMenuItem,
        _method_info: unity2::OptionalMethod,
    ) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
}

impl Default for BasicMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicMenuItem {
    pub fn new() -> Self {
        let item = <Self as unity2::FromIlInstance>::instantiate()
            .expect("BasicMenuItem::new allocation failed");
        item.ctor();
        item
    }

    pub fn new_impl<M: BasicMenuItemMethods>() -> Self {
        use unity2::Cast;
        let item = Self::new();
        let class = item.override_class();
        class.override_virtual_method("GetName", unity2::method_info!(M::get_name, 0));
        class.override_virtual_method("ACall", unity2::method_info!(M::a_call, 0));
        class.override_virtual_method("BCall", unity2::method_info!(M::b_call, 0));
        class.override_virtual_method("BuildAttribute", unity2::method_info!(M::build_attribute, 0));
        item
    }

    pub fn new_impl_from_template<M: BasicMenuItemMethods>(template: BasicMenuItem) -> Self {
        use unity2::Cast;
        use unity2::FromIlInstance;
        let cloned_class = template.get_class().clone_for_override();
        let item = <Self as FromIlInstance>::instantiate_with_class(cloned_class)
            .expect("BasicMenuItem::new_impl_from_template allocation failed");
        item.ctor();
        cloned_class.override_virtual_method("GetName", unity2::method_info!(M::get_name, 0));
        cloned_class.override_virtual_method("ACall", unity2::method_info!(M::a_call, 0));
        cloned_class.override_virtual_method("BCall", unity2::method_info!(M::b_call, 0));
        cloned_class.override_virtual_method("BuildAttribute", unity2::method_info!(M::build_attribute, 0));
        item
    }
}

#[unity2::methods]
impl BasicMenu {
    #[method(offset = 0x24533F0)]
    pub fn ctor(self, menu_item_list: List<BasicMenuItem>, menu_content: BasicMenuContent);

    #[method(offset = 0x2454310)]
    pub fn bind_parent_menu(self);
}

impl BasicMenu {
    pub fn close_anime_all(self) {
        virtual_call_0(self, "CloseAnimeAll");
    }

    pub fn close_anime(self) {
        virtual_call_0(self, "CloseAnime");
    }

    pub fn open_anime_all(self) {
        virtual_call_0(self, "OpenAnimeAll");
    }

    pub fn open_anime(self) {
        virtual_call_0(self, "OpenAnime");
    }

    pub fn create_default_desc(self) -> unity2::Array<crate::proc::ProcDesc> {
        let class = unity2::object_get_class(self);
        let entry = class
            .get_virtual_method("CreateDefaultDesc")
            .expect("BasicMenu vtable missing `CreateDefaultDesc`");
        let f: extern "C" fn(Self, &'static unity2::MethodInfo) -> unity2::Array<crate::proc::ProcDesc> =
            unsafe { std::mem::transmute(entry.method_ptr) };
        f(self, entry.method_info)
    }
}

fn virtual_call_0(this: BasicMenu, method_name: &str) {
    let class = unity2::object_get_class(this);
    let entry = class
        .get_virtual_method(method_name)
        .unwrap_or_else(|| panic!("BasicMenu vtable missing `{}`", method_name));
    let f: extern "C" fn(BasicMenu, &'static unity2::MethodInfo) =
        unsafe { std::mem::transmute(entry.method_ptr) };
    f(this, entry.method_info)
}

impl BasicMenu {
    pub fn build(menu_item_list: List<BasicMenuItem>, menu_content: BasicMenuContent) -> Self {
        let menu = <Self as unity2::FromIlInstance>::instantiate()
            .expect("BasicMenu::build allocation failed");
        menu.ctor(menu_item_list, menu_content);
        menu
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct BasicMenuResult(u16);

impl BasicMenuResult {
    const CLOSE_THIS: u16 = 1 << 0;
    const CLOSE_PARENT: u16 = 1 << 1;
    const CLOSE_ALL: u16 = 1 << 2;
    const DELETE_THIS: u16 = 1 << 3;
    const DELETE_PARENT: u16 = 1 << 4;
    const DELETE_ALL: u16 = 1 << 5;
    const SE_DECIDE: u16 = 1 << 7;
    const SE_DECIDE2: u16 = 1 << 8;
    const SE_CANCEL: u16 = 1 << 9;
    const SE_MISS: u16 = 1 << 11;
    const SE_CURSOR: u16 = 1 << 12;
    const DO_NOTHING: u16 = 1 << 13;

    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn bits(self) -> u16 {
        self.0
    }

    fn set(mut self, mask: u16, v: bool) -> Self {
        if v { self.0 |= mask; } else { self.0 &= !mask; }
        self
    }

    pub const fn close_this(self) -> bool { self.0 & Self::CLOSE_THIS != 0 }
    pub const fn close_parent(self) -> bool { self.0 & Self::CLOSE_PARENT != 0 }
    pub const fn close_all(self) -> bool { self.0 & Self::CLOSE_ALL != 0 }
    pub const fn delete_this(self) -> bool { self.0 & Self::DELETE_THIS != 0 }
    pub const fn delete_parent(self) -> bool { self.0 & Self::DELETE_PARENT != 0 }
    pub const fn delete_all(self) -> bool { self.0 & Self::DELETE_ALL != 0 }
    pub const fn do_nothing(self) -> bool { self.0 & Self::DO_NOTHING != 0 }

    pub fn with_close_this(self, v: bool) -> Self { self.set(Self::CLOSE_THIS, v) }
    pub fn with_close_parent(self, v: bool) -> Self { self.set(Self::CLOSE_PARENT, v) }
    pub fn with_close_all(self, v: bool) -> Self { self.set(Self::CLOSE_ALL, v) }
    pub fn with_delete_this(self, v: bool) -> Self { self.set(Self::DELETE_THIS, v) }
    pub fn with_delete_parent(self, v: bool) -> Self { self.set(Self::DELETE_PARENT, v) }
    pub fn with_delete_all(self, v: bool) -> Self { self.set(Self::DELETE_ALL, v) }
    pub fn with_se_decide(self, v: bool) -> Self { self.set(Self::SE_DECIDE, v) }
    pub fn with_se_decide2(self, v: bool) -> Self { self.set(Self::SE_DECIDE2, v) }
    pub fn with_se_cancel(self, v: bool) -> Self { self.set(Self::SE_CANCEL, v) }
    pub fn with_se_miss(self, v: bool) -> Self { self.set(Self::SE_MISS, v) }
    pub fn with_se_cursor(self, v: bool) -> Self { self.set(Self::SE_CURSOR, v) }
    pub fn with_do_nothing(self, v: bool) -> Self { self.set(Self::DO_NOTHING, v) }

    pub fn se_cursor() -> Self { Self::new().with_se_cursor(true) }
    pub fn se_decide() -> Self { Self::new().with_se_decide(true) }
    pub fn close_decide() -> Self { Self::new().with_close_this(true).with_se_decide(true) }
    pub fn se_miss() -> Self { Self::new().with_se_miss(true) }
    pub fn close_parent_decide() -> Self { Self::new().with_close_parent(true).with_se_decide(true) }
    pub fn delete_decide() -> Self { Self::new().with_delete_this(true).with_se_decide(true) }
    pub fn close_cancel() -> Self { Self::new().with_se_cancel(true).with_close_this(true) }
}
