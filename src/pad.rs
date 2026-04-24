#![allow(unused_imports)]

use unity2::ClassIdentity;

use crate::data::{ISingletonClass, SingletonClass};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NpadButton: i64 {
        const A             = 1 << 0;
        const B             = 1 << 1;
        const X             = 1 << 2;
        const Y             = 1 << 3;
        const STICK_L       = 1 << 4;
        const STICK_R       = 1 << 5;
        const L             = 1 << 6;
        const R             = 1 << 7;
        const ZL            = 1 << 8;
        const ZR            = 1 << 9;
        const PLUS          = 1 << 10;
        const MINUS         = 1 << 11;
        const LEFT          = 1 << 12;
        const UP            = 1 << 13;
        const RIGHT         = 1 << 14;
        const DOWN          = 1 << 15;
        const STICK_L_LEFT  = 1 << 16;
        const STICK_L_UP    = 1 << 17;
        const STICK_L_RIGHT = 1 << 18;
        const STICK_L_DOWN  = 1 << 19;
        const STICK_R_LEFT  = 1 << 20;
        const STICK_R_UP    = 1 << 21;
        const STICK_R_RIGHT = 1 << 22;
        const STICK_R_DOWN  = 1 << 23;
        const LEFT_SL       = 1 << 24;
        const LEFT_SR       = 1 << 25;
        const RIGHT_SL      = 1 << 26;
        const RIGHT_SR      = 1 << 27;
    }
}

#[unity2::enumeration]
#[repr(i32)]
pub enum PadMode {
    Switch = 0,
    Xbox = 1,
    XboxAbSwap = 2,
    Ps3 = 3,
    Elecom = 4,
    CyberGadget = 5,
    Keyboard = 6,
}

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<Pad>)]
pub struct Pad {}

#[unity2::methods]
impl Pad {
    #[method(name = "IsButton")]
    pub fn is_button(buttons: NpadButton) -> bool;

    #[method(name = "IsTrigger")]
    pub fn is_trigger(buttons: NpadButton) -> bool;

    #[method(name = "IsRepeat")]
    pub fn is_repeat(buttons: NpadButton) -> bool;

    #[method(name = "IsRelease")]
    pub fn is_release(buttons: NpadButton) -> bool;

    #[method(name = "GetStickLX", args = 0)]
    pub fn get_stick_lx() -> f32;
    #[method(name = "GetStickLY", args = 0)]
    pub fn get_stick_ly() -> f32;
    #[method(name = "GetStickRX", args = 0)]
    pub fn get_stick_rx() -> f32;
    #[method(name = "GetStickRY", args = 0)]
    pub fn get_stick_ry() -> f32;

    #[method(name = "GetStickLX", args = 1)]
    pub fn get_stick_lx_dead(allowance: f32) -> f32;
    #[method(name = "GetStickLY", args = 1)]
    pub fn get_stick_ly_dead(allowance: f32) -> f32;
    #[method(name = "GetStickRX", args = 1)]
    pub fn get_stick_rx_dead(allowance: f32) -> f32;
    #[method(name = "GetStickRY", args = 1)]
    pub fn get_stick_ry_dead(allowance: f32) -> f32;

    #[method(name = "GetStepCount")]
    pub fn get_step_count() -> i32;
    #[method(name = "GetHoldCount")]
    pub fn get_hold_count() -> i32;

    #[method(name = "SetEnableControllerSupport")]
    pub fn set_enable_controller_support(is_enable: bool);

    #[method(name = "Vibration", args = 0)]
    pub fn vibration() -> crate::vibrationmanager::VibrationManager;
}
