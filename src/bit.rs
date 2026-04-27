use bitflags::Flags;
use unity2::{Array, ClassIdentity};


#[unity2::class(namespace = "App")]
pub struct BitStruct {
    #[rename(name = "m_Bits")]
    pub bits: Array<u8>,
}

#[unity2::class(namespace = "App")]
pub struct BitFieldHelper {}

#[unity2::class(namespace = "App")]
pub struct BitFieldCommon {}

#[unity2::class(namespace = "App", name = "BitField32")]
#[parent(BitFieldCommon)]
pub struct BitField32<T: ClassIdentity + Flags + Copy> {
    #[rename(name = "m_Value")]
    pub value: T,
}

#[unity2::class(namespace = "App", name = "BitField64")]
#[parent(BitFieldCommon)]
pub struct BitField64<T: ClassIdentity + Flags + Copy> {
    #[rename(name = "m_Value")]
    pub value: T,
}

#[unity2::class(namespace = "App", name = "BitFieldTemplate32`1")]
#[parent(BitField32<T>, BitFieldCommon)]
pub struct BitFieldTemplate32<T: ClassIdentity + Flags + Copy> {}

#[unity2::class(namespace = "App", name = "BitFieldTemplate64`1")]
#[parent(BitField64<T>, BitFieldCommon)]
pub struct BitFieldTemplate64<T: ClassIdentity + Flags + Copy> {}