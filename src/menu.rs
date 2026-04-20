#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::proc::{IProcInst, ProcInst};

#[unity2::class(namespace = "App")]
#[parent(ProcInst)]
pub struct BasicMenu {}

#[unity2::class(namespace = "App")]
pub struct BasicMenuItem {}

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
