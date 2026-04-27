#![allow(unused_imports)]

use unity2::{Array, Class, ClassIdentity, Il2CppString};

use crate::data::{IStructData, IStructDataMethods, StructData};
use crate::gmap::{GmapSpotState, GmapSpotEncountType};

#[unity2::class(namespace = "App")]
#[parent(StructData<ChapterData>)]
pub struct ChapterData {
    #[backing] pub cid: Il2CppString,
    #[backing] pub name: Il2CppString,
    #[backing] pub alpha: f32,
    #[backing] pub mess: Il2CppString,
    #[backing] pub event: Il2CppString,
    #[backing] pub field: Il2CppString,
    #[backing] pub script_bmap: Il2CppString,
    #[backing] pub script_encount: Il2CppString,
    #[backing] pub script_kizuna: Il2CppString,
    #[backing] pub chapter_title: Il2CppString,
    #[backing] pub terrain: Il2CppString,
    #[backing] pub dispos: Il2CppString,
    #[backing] pub next_chapter: Il2CppString,
    #[backing] pub gmap_spot: Il2CppString,
    #[backing] pub gmap_spot_state: GmapSpotState,
    #[backing] pub gmap_spot_open_condition: Il2CppString,
    #[backing] pub gmap_spot_encount: GmapSpotEncountType,
    #[backing] pub encount_jobs: Array<Il2CppString>,
    #[backing] pub reward: Il2CppString,
    #[backing] pub hold_level: u8,
    #[backing] pub progress: u8,
    #[backing] pub flag: ChapterDataFlags,
    #[backing] pub sound_field_situation: Il2CppString,
    #[backing] pub player_phase_bgm: Il2CppString,
    #[backing] pub enemy_phase_bgm: Il2CppString,
    #[backing] pub ally_phase_bgm: Il2CppString,
    #[backing] pub player_encount_bgm: Il2CppString,
    #[backing] pub enemy_encount_bgm: Il2CppString,
    #[backing] pub sortie_bgm: Il2CppString,
    #[backing] pub kizuna_bgm: Il2CppString,
    #[backing] pub help: Il2CppString,
    #[backing] pub recommended_level: u8,
    #[backing] pub nation: Il2CppString,
    #[backing] pub nation_name: Il2CppString,
    #[backing] pub net_kill_bonus_index: u8,
    #[backing] pub net_ranking_index: u8,
    #[backing] pub hub: Il2CppString,
    #[rename(name = "m_PrefixlessCid")] pub prefixless_cid: Il2CppString,
    #[rename(name = "m_ClearedFlagName")] pub cleared_flag_name: Il2CppString,
    #[rename(name = "m_GmapSpotFlagName")] pub gmap_spot_flag_name: Il2CppString,
    #[rename(name = "m_PlaceName")] pub place_name: Il2CppString,
}

#[unity2::methods]
impl ChapterData {
    #[method(name = "get_Cid")]
    fn cid(self) -> Il2CppString;

    #[method(name = "get_Name")]
    fn name(self) -> Il2CppString;

    #[method(name = "get_Mess")]
    fn mess(self) -> Il2CppString;

    #[method(name = "get_Field")]
    fn field(self) -> Il2CppString;

    #[method(name = "GetPrefixlessCid", args = 0)]
    pub fn get_prefixless_cid(self) -> Il2CppString;
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct ChapterDataFlags: i32 {
        const Sally = 1 << 0;
        const CanBack = 1 << 1;
        const Sight = 1 << 2;
        const Kizuna = 1 << 3;
        const Hub = 1 << 4;
        const Gmap = 1 << 5;
        const Continue = 1 << 6;
        const Serious = 1 << 7;
        const Casual = 1 << 8;
        const Challenge = 1 << 9;
        const Relay = 1 << 10;
        const Versus = 1 << 11;
        const TestMap = 1 << 12;
        const Opposition = 1 << 13;
        const HighRankItem = 1 << 14;
        const CanSlope = 1 << 15;
        const SideStory = 1 << 30;
        const Scenario = -2147483648; // i32::MIN
    }
}

impl ClassIdentity for ChapterDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "ChapterData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}