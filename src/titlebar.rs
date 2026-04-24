#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

// Nested enum, namespace is outer's namespace
#[unity2::enumeration(namespace = "App", name = "TitleBar.FooterType")]
#[repr(i32)]
pub enum TitleBarFooterType {
    None = 0,
    Gold = 1,
    PieceOfBond = 2,
    GoldAndBond = 3,
    Refine = 4,
    GoldAndRefine = 5,
    GoldAndBondAndRefine = 7,
    Proof = 16,
    RelayTicket = 32,
    GoldAndBondAndRelayTicket = 35,
}

#[unity2::class(namespace = "App")]
pub struct TitleBar {}

#[unity2::methods]
impl TitleBar {
    #[method(name = "get_Instance", args = 0)]
    pub fn instance() -> TitleBar;

    #[method(name = "OpenHeader", args = 3)]
    pub fn open_header_raw(
        self,
        title: Il2CppString,
        title_help: Il2CppString,
        key_help_id: Il2CppString,
    ) -> bool;

    #[method(name = "CloseHeader", args = 0)]
    pub fn close_header_raw(self) -> bool;

    #[method(name = "IsOpenHeader", args = 0)]
    pub fn is_open_header(self) -> bool;

    #[method(name = "HideFooter", args = 0)]
    pub fn hide_footer_raw(self);

    #[method(name = "ShowFooter", args = 0)]
    pub fn show_footer_raw(self);

    #[method(name = "OpenFooter", args = 1)]
    pub fn open_footer_raw(self, ty: TitleBarFooterType);
}

impl TitleBar {
    pub fn open_header(
        title: impl Into<Il2CppString>,
        title_help: impl Into<Il2CppString>,
        key_help_id: impl Into<Il2CppString>,
    ) -> bool {
        Self::instance().open_header_raw(title.into(), title_help.into(), key_help_id.into())
    }

    pub fn close_header() -> bool {
        Self::instance().close_header_raw()
    }

    pub fn hide_footer() {
        Self::instance().hide_footer_raw();
    }

    pub fn show_footer() {
        Self::instance().show_footer_raw();
    }

    pub fn open_footer(ty: TitleBarFooterType) {
        Self::instance().open_footer_raw(ty);
    }
}
