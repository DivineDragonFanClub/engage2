use unity2::Class;
use unity2::ClassIdentity;
use unity2::Il2CppString;
use unity2::engine::Color;

use crate::data::StructData;
use crate::data::IStructData;


#[unity2::class(namespace = "App")]
#[parent(StructData<TerrainData>)]
pub struct TerrainData {
    #[backing] pub tid: Il2CppString,
    #[backing] pub name: Il2CppString,
    #[backing] pub cost_name: Il2CppString,
    #[backing] pub cost_type: i32,
    #[backing] pub layer: TerrainDataLayers,
    #[backing] pub prohibition: TerrainDataProhibitions,
    #[backing] pub command: TerrainDataCommands,
    #[backing] pub sight: u8,
    #[backing] pub destroyer: TerrainDataDestroyers,
    #[backing(name = "Hp_N")] pub hp_n: i32,
    #[backing(name = "Hp_H")] pub hp_h: i32,
    #[backing(name = "Hp_L")] pub hp_l: i32,
    #[backing] pub defense: i8,
    #[backing] pub avoid: i8,
    #[backing] pub player_defense: i8,
    #[backing] pub enemy_defense: i8,
    #[backing] pub player_avoid: i8,
    #[backing] pub enemy_avoid: i8,
    #[backing] pub heal: i8,
    #[backing] pub life: u8,
    #[backing] pub move_cost: u8,
    #[backing] pub fly_cost: u8,
    #[backing] pub move_first: i8,
    #[backing] pub offset: f32,
    #[backing] pub color_r: u8,
    #[backing] pub color_g: u8,
    #[backing] pub color_b: u8,
    #[backing] pub color: Color,
    #[backing] pub change_tid: Il2CppString,
    #[backing] pub change_encount: Il2CppString,
    #[backing] pub height: f32,
    #[backing] pub put_effect: Il2CppString,
    #[backing] pub minimap: Il2CppString,
    #[backing] pub cannon_skill: Il2CppString,
    #[backing] pub cannon_shells_n: u8,
    #[backing] pub cannon_shells_h: u8,
    #[backing] pub cannon_shells_l: u8,
    #[backing] pub flag: TerrainDataFlags,
    #[backing] pub put_allow: u8,
    #[backing] pub ascii_name: Il2CppString,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct TerrainDataFlags: i32 {
        const Door = 1 << 0;
        const Treasure = 1 << 1;
        const Visit = 1 << 2;
        const BowCannon = 1 << 3;
        const MagicCannon = 1 << 4;
        const FireCannon = 1 << 5;
        // No entry for 2^6 = 64
        const NoShadow = 1 << 7;
        const FootSmoke = 1 << 8;
        const FootPrint = 1 << 9;
        const Roof = 1 << 10;
        const SightMasking = 1 << 11;
        const NotStun = 1 << 12;
        const NotEngageAdd = 1 << 13;
        const FlyEnable = 1 << 14;
        const EngageHeal = 1 << 15;
        const NotTarget = 1 << 16;
        const NotWarp = 1 << 17;
        const DamageHalfDisplay = 1 << 18;
        const HideBreakIcon = 1 << 19;
        const ShowPhaseIcon = 1 << 20;
        // No entries for 2^21 through 2^28
        const Immobile = 1 << 29;
        const Minimap = 1 << 30;
        const HelpSpot = -2147483648; // i32::MIN
    }
}

impl ClassIdentity for TerrainDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "TerrainData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "TerrainData.Destroyers")]
#[repr(i32)]
pub enum TerrainDataDestroyers {
    None = 0,
    Player = 1,
    Enemy = 2,
}

#[unity2::enumeration(namespace = "", name = "TerrainData.Prohibitions")]
#[repr(i32)]
pub enum TerrainDataProhibitions {
    None = 0,
    All = 1,
    Ground = 2,
    Near = 3,
}

#[unity2::enumeration(namespace = "", name = "TerrainData.Layers")]
#[repr(i32)]
pub enum TerrainDataLayers {
    Lower = 0,
    Upper = 1,
}

#[unity2::enumeration(namespace = "", name = "TerrainData.Commands")]
#[repr(i32)]
pub enum TerrainDataCommands {
    None = 0,
    TorchOn = 1,
    TorchOff = 2,
}