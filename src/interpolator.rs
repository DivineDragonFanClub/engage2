use unity2::ClassIdentity;
use unity2::engine::Vector2;
use unity2::engine::Vector3;
use unity2::engine::Color;

#[unity2::class(namespace = "App")]
pub struct InterpolatorTime {
    #[rename(name = "m_Time")]
    #[readonly]
    pub time: f32,
    #[rename(name = "m_Tick")]
    #[readonly]
    pub tick: f32,
    #[rename(name = "m_Type")]
    #[readonly]
    pub ty: CurveType,
    #[rename(name = "m_Num")]
    #[readonly]
    pub num: i32,
    #[rename(name = "m_IsFirst")]
    #[readonly]
    pub is_first: bool,
    #[rename(name = "m_IsDirty")]
    #[readonly]
    pub is_dirty: bool,
}

#[unity2::class(namespace = "App", name = "Interpolator`1")]
#[parent(InterpolatorTime)]
pub struct Interpolator<T: ClassIdentity> {
    #[rename(name = "m_Prev")]
    pub prev: T,
    #[rename(name = "m_Next")]
    pub next: T,
}

#[unity2::class(namespace = "App")]
#[parent(Interpolator<f32>)]
pub struct InterpolatorFloat {}

#[unity2::class(namespace = "App")]
#[parent(Interpolator<i32>)]
pub struct InterpolatorInt {}

#[unity2::class(namespace = "App")]
#[parent(Interpolator<Vector2>)]
pub struct InterpolatorVector2 {}

#[unity2::class(namespace = "App")]
#[parent(Interpolator<Vector3>)]
pub struct InterpolatorVector3 {}

#[unity2::class(namespace = "App")]
#[parent(Interpolator<Color>)]
pub struct InterpolatorColor {}

#[unity2::class(namespace = "App")]
#[parent(InterpolatorFloat)]
pub struct InterpolatorRotation {}

#[unity2::class(namespace = "App")]
pub struct Curve {}

#[unity2::enumeration(namespace = "", name = "Curve.Type")]
#[repr(i32)]
pub enum CurveType {
    Linear = 0,
    Accel = 1,
    Decel = 2,
    AccelDecel = 3,
    DecelAccel = 4,
    LinearDecel = 5,
    LinearAccel = 6,
    DecelLinear = 7,
    AccelLinear = 9,
}