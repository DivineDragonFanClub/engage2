#![allow(unused_imports)]

use unity2::system::collections::{IListMethods, List};
use unity2::system::delegate::Action;
use unity2::{Cast, Class, ClassIdentity, FromIlInstance, Il2CppString, OptionalMethod, SystemObject};

use crate::menu::{BasicMenu, BasicMenuItem, BasicMenuResult, IBasicMenu, IBasicMenuItem};
use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(BasicMenuItem)]
pub struct BasicDialogItem {
    #[rename(name = "m_Text")]
    pub text: Il2CppString,
}

#[unity2::methods]
impl BasicDialogItem {
    #[method(offset = 0x2455F30)]
    fn ctor(self, text: Il2CppString);

    #[method(offset = 0x2456180)]
    fn get_name(self) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
#[parent(BasicDialogItem, BasicMenuItem)]
pub struct BasicDialogItemYes {}

#[unity2::methods]
impl BasicDialogItemYes {
    #[method(offset = 0x2456340)]
    fn ctor_empty(self);

    #[method(offset = 0x2456430)]
    fn ctor_with_text(self, text: Il2CppString);
}

impl BasicDialogItemYes {
    pub fn new(text: impl AsRef<str>) -> Self {
        let item = Self::instantiate().expect("BasicDialogItemYes::new allocation failed");
        item.ctor_with_text(text.as_ref());
        item
    }

    pub fn new_empty() -> Self {
        let item = Self::instantiate().expect("BasicDialogItemYes::new_empty allocation failed");
        item.ctor_empty();
        item
    }
}

#[unity2::class(namespace = "App")]
#[parent(BasicDialogItem, BasicMenuItem)]
pub struct BasicDialogItemNo {}

#[unity2::methods]
impl BasicDialogItemNo {
    #[method(offset = 0x24561B0)]
    fn ctor_empty(self);

    #[method(offset = 0x24562A0)]
    fn ctor_with_text(self, text: Il2CppString);
}

impl BasicDialogItemNo {
    pub fn new(text: impl AsRef<str>) -> Self {
        let item = Self::instantiate().expect("BasicDialogItemNo::new allocation failed");
        item.ctor_with_text(text.as_ref());
        item
    }

    pub fn new_empty() -> Self {
        let item = Self::instantiate().expect("BasicDialogItemNo::new_empty allocation failed");
        item.ctor_empty();
        item
    }
}

pub trait TwoChoiceDialogMethods {
    extern "C" fn on_first_choice(
        _this: BasicDialogItemYes,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true)
    }
    extern "C" fn on_second_choice(
        _this: BasicDialogItemNo,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true)
    }
    extern "C" fn bcall_first(
        _this: BasicDialogItemYes,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true).with_se_cancel(true)
    }
    extern "C" fn bcall_second(
        _this: BasicDialogItemNo,
        _method_info: OptionalMethod,
    ) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true).with_se_cancel(true)
    }
}

#[unity2::class(namespace = "App")]
pub struct YesNoDialog {}

#[unity2::methods]
impl YesNoDialog {
    #[method(offset = 0x29403D0)]
    fn create_bind(
        proc: ProcInst,
        message: Il2CppString,
        yes_item: BasicDialogItemYes,
        no_item: BasicDialogItemNo,
    ) -> YesNoDialog;
}

impl YesNoDialog {
    pub fn bind<Methods: TwoChoiceDialogMethods>(
        proc: impl Into<ProcInst>,
        message: impl AsRef<str>,
        first_text: impl AsRef<str>,
        second_text: impl AsRef<str>,
    ) {
        let first_item = BasicDialogItemYes::new(first_text);
        let second_item = BasicDialogItemNo::new(second_text);

        // Clone the classes here so we don't ruin the base class with our custom dialog
        let yes_class = first_item.override_class();
        let no_class = second_item.override_class();

        let on_first = unity2::method_info!(Methods::on_first_choice, 0);
        let bcall_first = unity2::method_info!(Methods::bcall_first, 0);
        let on_second = unity2::method_info!(Methods::on_second_choice, 0);
        let bcall_second = unity2::method_info!(Methods::bcall_second, 0);

        yes_class.override_virtual_method("ACall", on_first);
        yes_class.override_virtual_method("BCall", bcall_first);
        no_class.override_virtual_method("ACall", on_second);
        no_class.override_virtual_method("BCall", bcall_second);

        let _ = Self::create_bind(proc.into(), message.as_ref(), first_item, second_item);
    }
}

#[unity2::class(namespace = "App")]
#[parent(BasicDialogItemYes, BasicDialogItem, BasicMenuItem)]
pub struct YesMenuItem {
    #[rename(name = "YesEventHandler")]
    pub yes_event_handler: Action,
}

#[unity2::methods]
impl YesMenuItem {
    #[method(offset = 0x293D8F0)]
    fn ctor_with_action(self, yes_event_handler: Action);
}

impl YesMenuItem {
    pub fn new(handler: Action) -> Self {
        let item = Self::instantiate().expect("YesMenuItem::new allocation failed");
        item.ctor_with_action(handler);
        item
    }

    pub fn new_with_label(text: impl AsRef<str>, handler: Action) -> Self {
        let item = Self::instantiate().expect("YesMenuItem::new_with_label allocation failed");
        item.ctor_with_action(handler);
        item.set_text(text.as_ref().into());
        item
    }
}

#[unity2::class(namespace = "App")]
#[parent(BasicMenu, ProcInst)]
pub struct BasicDialog {}

#[unity2::methods]
impl BasicDialog {
    #[method(offset = 0x2454120)]
    fn create_basic_dialog_bind(proc: ProcInst, menu_items: List<BasicMenuItem>) -> BasicDialog;

    #[method(offset = 0x2453D90)]
    fn set_text(self, text: Il2CppString);
}

impl BasicMenu {
    pub fn confirm(
        proc: impl Into<ProcInst>,
        message: impl AsRef<str>,
        yes_text: impl AsRef<str>,
        no_text: impl AsRef<str>,
        handler: Action,
    ) -> BasicDialog {
        let yes_item = YesMenuItem::new_with_label(yes_text, handler);
        let no_item = BasicDialogItemNo::new(no_text);

        let items = List::<BasicMenuItem>::new().expect("BasicMenu::confirm list allocation failed");
        items.add(yes_item.into());
        items.add(no_item.into());

        let dialog = BasicDialog::create_basic_dialog_bind(proc.into(), items);
        dialog.set_text(message.as_ref());
        dialog
    }
}
