#![allow(unused_imports)]

use unity2::system::collections::List;
use unity2::{
    Array, Cast, Class, ClassIdentity, FromIlInstance, Il2CppString, OptionalMethod, SystemObject,
};

use crate::menu::{BasicMenu, BasicMenuItem, BasicMenuResult, IBasicMenu, IBasicMenuItem};
use crate::proc::{IProcInst, ProcInst};

#[unity2::enumeration(namespace = "App", name = "BasicMenuItem.Attribute")]
#[repr(i32)]
pub enum BasicMenuItemAttribute {
    Enable = 1,
    Disable = 2,
    Hide = 4,
    Blank = 8,
    Select = 16,
}

#[unity2::enumeration(namespace = "", name = "ConfigBasicMenuItem.ConfigMethodKind")]
#[repr(i32)]
pub enum ConfigMethodKind {
    Switch = 0,
    Gauge = 1,
}

#[unity2::class(namespace = "")]
#[parent(BasicMenu)]
pub struct ConfigMenu {}

#[unity2::methods]
impl ConfigMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: ProcInst);
}

#[unity2::class(namespace = "")]
#[parent(BasicMenuItem)]
pub struct ConfigBasicMenuItem {
    #[rename(name = "m_ConfigMethod")]
    pub config_method: ConfigMethodKind,
    #[rename(name = "m_TitleText")]
    pub title_text: Il2CppString,
    #[rename(name = "m_CommandText")]
    pub command_text: Il2CppString,
    #[rename(name = "m_HelpText")]
    pub help_text: Il2CppString,
    #[rename(name = "m_IsArrow")]
    pub is_arrow: bool,
    #[rename(name = "m_IsCommandIcon")]
    pub is_command_icon: bool,
    #[rename(name = "m_GaugeRatio")]
    pub gauge_ratio: f32,
}

#[unity2::methods]
impl ConfigBasicMenuItem {
    #[method(offset = 0x25379A0)]
    fn ctor(self);

    #[method(name = "UpdateText", args = 0)]
    pub fn update_text(self);

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self);

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self);

    #[method(name = "IsConfigMethod", args = 1)]
    pub fn is_config_method(self, kind: ConfigMethodKind) -> bool;

    #[method(offset = 0x2537920)]
    pub fn change_key_value_i(value: i32, min: i32, max: i32, step: i32) -> i32;

    #[method(offset = 0x2537970)]
    pub fn change_key_value_f(value: f32, min: f32, max: f32, step: f32) -> f32;
}

pub trait ConfigBasicMenuItemSwitchMethods {
    fn init_content(_this: ConfigBasicMenuItem) {}
    extern "C" fn custom_call(this: ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_command_text(this: ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn a_call(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new()
    }
    extern "C" fn build_attribute(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
}

pub trait ConfigBasicMenuItemCommandMethods {
    fn init_content(_this: ConfigBasicMenuItem) {}
    extern "C" fn custom_call(this: ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_command_text(this: ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn on_select(this: ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.on_select();
        this.set_is_arrow(false);
        this.on_deselect();
    }
    extern "C" fn on_deselect(this: ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.on_select();
        this.set_is_arrow(false);
        this.on_deselect();
    }
    extern "C" fn a_call(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new()
    }
    extern "C" fn build_attribute(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
}

pub trait ConfigBasicMenuItemGaugeMethods {
    fn init_content(_this: ConfigBasicMenuItem) {}
    extern "C" fn custom_call(this: ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_help_text(this: ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn a_call(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new()
    }
    extern "C" fn build_attribute(_this: ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::Enable
    }
}

impl ConfigBasicMenuItem {
    pub fn new() -> Self {
        let item = <Self as FromIlInstance>::instantiate()
            .expect("ConfigBasicMenuItem::new allocation failed");
        item.ctor();
        item
    }

    pub fn change_key_value_b(value: bool) -> bool {
        Self::change_key_value_i(value as i32, 0, 1, 1) == 1
    }

    pub fn new_switch<M: ConfigBasicMenuItemSwitchMethods>(
        title: impl Into<Il2CppString>,
    ) -> Self {
        let item = Self::new();
        M::init_content(item);

        item.set_config_method(ConfigMethodKind::Switch);
        install_switch_overrides::<M>(item);

        item.set_title_text(title.into());
        M::set_command_text(item, None);
        M::set_help_text(item, None);

        item
    }

    pub fn new_gauge<M: ConfigBasicMenuItemGaugeMethods>(
        title: impl Into<Il2CppString>,
    ) -> Self {
        let item = Self::new();
        M::init_content(item);

        item.set_config_method(ConfigMethodKind::Gauge);
        install_gauge_overrides::<M>(item);

        item.set_title_text(title.into());
        M::set_help_text(item, None);

        item
    }

    pub fn new_command<M: ConfigBasicMenuItemCommandMethods>(
        title: impl Into<Il2CppString>,
    ) -> Self {
        let item = Self::new();
        M::init_content(item);

        item.set_config_method(ConfigMethodKind::Switch);
        item.set_is_arrow(false);
        item.set_is_command_icon(true);
        install_command_overrides::<M>(item);

        item.set_title_text(title.into());
        M::set_command_text(item, None);
        M::set_help_text(item, None);

        item
    }
}

fn install_switch_overrides<M: ConfigBasicMenuItemSwitchMethods>(item: ConfigBasicMenuItem) {
    let class = item.override_class();
    class.override_virtual_method("CustomCall", unity2::method_info!(M::custom_call, 0));
    class.override_virtual_method("ACall", unity2::method_info!(M::a_call, 0));
    class.override_virtual_method("BuildAttribute", unity2::method_info!(M::build_attribute, 0));
}

fn install_gauge_overrides<M: ConfigBasicMenuItemGaugeMethods>(item: ConfigBasicMenuItem) {
    let class = item.override_class();
    class.override_virtual_method("CustomCall", unity2::method_info!(M::custom_call, 0));
    class.override_virtual_method("ACall", unity2::method_info!(M::a_call, 0));
    class.override_virtual_method("BuildAttribute", unity2::method_info!(M::build_attribute, 0));
}

fn install_command_overrides<M: ConfigBasicMenuItemCommandMethods>(item: ConfigBasicMenuItem) {
    let class = item.override_class();
    class.override_virtual_method("CustomCall", unity2::method_info!(M::custom_call, 0));
    class.override_virtual_method("OnSelect", unity2::method_info!(M::on_select, 0));
    class.override_virtual_method("OnDeselect", unity2::method_info!(M::on_deselect, 0));
    class.override_virtual_method("ACall", unity2::method_info!(M::a_call, 0));
    class.override_virtual_method("BuildAttribute", unity2::method_info!(M::build_attribute, 0));
}

pub extern "C" fn restore_parent_on_dispose(this: ProcInst, _method_info: OptionalMethod) {
    let parent = this.super_();
    if parent.is_null() {
        return;
    }
    if let Some(slot) = parent.get_class().raw().get_virtual_method("OpenAnimeAll") {
        let f: extern "C" fn(ProcInst, &'static unity2::MethodInfo) =
            unsafe { std::mem::transmute(slot.method_ptr) };
        f(parent, slot.method_info);
    }
}
