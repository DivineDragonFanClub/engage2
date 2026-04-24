#![allow(unused_imports)]

use unity2::engine::color::Color;
use unity2::ClassIdentity;

#[unity2::enumeration(namespace = "App", name = "Fade.Layer")]
#[repr(i32)]
pub enum FadeLayer {
    System = 0,
    Skip = 1,
    Talk = 2,
    Combat = 3,
    Current = 4,
}

#[unity2::class(namespace = "App")]
pub struct Fade {}

#[unity2::methods]
impl Fade {
    #[method(name = "Initialize", args = 0)]
    fn initialize();

    #[method(name = "FadeIn", args = 1)]
    fn fade_in(duration: f32);

    #[method(name = "FadeOut", args = 1)]
    fn fade_out(duration: f32);

    #[method(name = "FadeIn", args = 2)]
    fn fade_in_on_layer(layer: FadeLayer, duration: f32);

    #[method(name = "GetImageColor", args = 1)]
    fn get_image_color(layer: FadeLayer) -> Color;

    #[method(name = "SetImageColor", args = 2)]
    fn set_image_color(layer: FadeLayer, color: Color);

    #[method(name = "PushLayer", args = 1)]
    fn push_layer(layer: FadeLayer);

    #[method(name = "PopLayer", args = 0)]
    fn pop_layer();

    #[method(name = "IsActive", args = 0)]
    fn is_active() -> bool;

    #[method(name = "IsActive", args = 1)]
    fn is_active_on_layer(layer: FadeLayer) -> bool;

    #[method(name = "BlackIn", args = 2)]
    pub fn black_in(duration: f32, layer: FadeLayer) -> crate::proc::ProcDesc;

    #[method(name = "BlackOut", args = 2)]
    pub fn black_out(duration: f32, layer: FadeLayer) -> crate::proc::ProcDesc;

    #[method(name = "WhiteIn", args = 2)]
    pub fn white_in(duration: f32, layer: FadeLayer) -> crate::proc::ProcDesc;

    #[method(name = "WhiteOut", args = 2)]
    pub fn white_out(duration: f32, layer: FadeLayer) -> crate::proc::ProcDesc;
}
