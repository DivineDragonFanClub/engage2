use unity2::{Array, ClassIdentity, Il2CppString};


#[unity2::class(namespace = "App")]
pub struct CapabilityDefinition {
    #[static_field]
    #[readonly]
    pub names: Array<Il2CppString>,
    #[static_field]
    #[readonly]
    pub helps: Array<Il2CppString>,
}

impl CapabilityDefinition {
    pub const NUM: i32 = 11;
    pub const CC_NUM: i32 = 8;
    pub const GROW_NUM: i32 = 9;
}

#[unity2::class(namespace = "App", name = "CapabilityBase`1")]
// #[parent(CapabilityDefinition)]
pub struct CapabilityBase<T: ClassIdentity> {
    #[rename(name = "m_Data")]
    pub data: Array<T>,
}

impl<T: ClassIdentity> CapabilityBase<T> {
    pub const VERSION: i32 = 0;
}
impl<T: ClassIdentity> ICapabilityDefinition for CapabilityBase<T> {}

#[unity2::class(namespace = "App")]
#[parent(CapabilityBase<u8>)]
pub struct Capability {}

impl Capability {
    pub const MIN: i32 = 0;
    pub const MAX: i32 = 255;
}

#[unity2::class(namespace = "App")]
#[parent(CapabilityBase<i8>)]
pub struct CapabilitySbyte {}

impl CapabilitySbyte {
    pub const MIN: i32 = -120;
    pub const MAX: i32 = 120;
}

#[unity2::class(namespace = "App")]
#[parent(CapabilityBase<i16>)]
pub struct CapabilityShort {}

impl CapabilityShort {
    pub const MIN: i32 = -10000;
    pub const MAX: i32 = 10000;
}

#[unity2::class(namespace = "App")]
#[parent(CapabilityBase<i32>)]
pub struct CapabilityInt {}

impl CapabilityInt {
    pub const MIN: i32 = -10000;
    pub const MAX: i32 = 10000;
}

#[unity2::class(namespace = "App")]
#[parent(CapabilityBase<f32>)]
pub struct CapabilityFloat {}


// Enums
#[unity2::enumeration(namespace = "", name = "CapabilityDefinition.Type")]
#[repr(i32)]
pub enum CapabilityDefinitionType {
    None = (-1),
    Hp = 0,
    Str = 1,
    Tech = 2,
    Quick = 3,
    Luck = 4,
    Def = 5,
    Magic = 6,
    Mdef = 7,
    Phys = 8,
    Sight = 9,
    Move = 10,
    Num = 11,
    // CcNum = 8,
    // GrowNum = 9,
}