use unity2::{Class, ClassIdentity};


#[unity2::class(namespace = "App")]
pub struct Dir {}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct DirType: u8 {
        const Left = 1 << 0;
        const Right = 1 << 1;
        const Down = 1 << 2;
        const Up = 1 << 3;
        // Nothing set for 2^4 = 16
        const Center = 1 << 5;
        // Nothing set for 2^6 = 64
        const Terminate = 1 << 7;
        const None = 0;
        const UpLeft = 9;
        const UpRight = 10;
        const DownLeft = 5;
        const DownRight = 6;
    }
}

impl ClassIdentity for DirType {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "Dir.Type";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}