#![allow(unused_imports)]

use unity2::{ClassIdentity, Il2CppString};

#[unity2::class(namespace = "App")]
pub struct VibrationManager {}

#[unity2::methods]
impl VibrationManager {
    #[method(name = "OneShot", args = 6)]
    pub fn one_shot(
        self,
        time: f32,
        amplitude_magnitude: f32,
        amp_low: f32,
        amp_high: f32,
        freq_low: f32,
        freq_high: f32,
    );

    #[method(name = "PlayByVibrationFileName", args = 4)]
    pub fn play_by_vibration_file_name(
        self,
        vib_file_name: Il2CppString,
        amplitude_magnitude: f32,
        time: f32,
        is_loop: bool,
    );
}

pub const FREQ_LOW: f32 = 160.0;
pub const FREQ_HIGH: f32 = 300.0;

pub fn vibrate(
    time: f32,
    amplitude_magnitude: f32,
    amp_low: f32,
    amp_high: f32,
    freq_low: f32,
    freq_high: f32,
) {
    use crate::pad::Pad;
    Pad::vibration().one_shot(time, amplitude_magnitude, amp_low, amp_high, freq_low, freq_high);
}
