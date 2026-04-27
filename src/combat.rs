#![allow(unused_imports)]

use unity2::engine::color::Color;
use unity2::system::collections::List;
use unity2::{Array, ClassIdentity, Il2CppString, OptionalMethod};

use crate::gamedata::{JobData, PersonData, unit::Unit};

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct HitType: i32 {
        const Critical = 1;
        const Miss = 2;
        const Guard = 4;
        const Hit = 8;
        const Parry = 16;
        const Knockoff = 64;
        const Heal = 128;
        const ChainGuard = 256;
        const DualGuard = 512;
        const HitStop = 268;
        const GuardType = 260;
        const MissType = 82;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Detail: i32 {
        const FirstAttack = 1;
        const LastAttack = 2;
        const Rush = 4;
        const Efficacy = 8;
        const EngageAttack = 16;
        const Break = 32;
        const Smash = 64;
        const StandingDie = 128;
        const DamageDisplayed = 256;
        const ChainAtk = 4096;
        const ChainAtk2 = 8192;
        const ChainGrd1 = 16384;
        const ChainGrd2 = 32768;
        const ChainGrd3 = 65536;
        const ChainGrd4 = 131072;
        const ChainGrd = 245760;
    }
}

#[unity2::enumeration(namespace = "Combat", name = "DamageEffectLevel")]
#[repr(i32)]
pub enum DamageEffectLevel {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[unity2::enumeration(namespace = "Combat", name = "MagicBulletSettings.ArrivalType")]
#[repr(i32)]
pub enum ArrivalType {
    Flying = 0,
    ConstantTime = 1,
}

#[unity2::class(namespace = "Combat")]
pub struct Phase {
    #[rename(name = "m_Kind")]
    pub kind: i32,
    #[rename(name = "m_HitType")]
    pub hit_type: HitType,
    #[rename(name = "m_Detail")]
    pub detail: Detail,
    #[rename(name = "m_AttackSide")]
    pub attack_side: i32,
    #[rename(name = "m_AttackHash")]
    pub attack_hash: i32,
    #[rename(name = "m_DamageHash")]
    pub damage_hash: i32,
}

#[unity2::methods]
impl Phase {
    #[method(name = "get_DamageEffectLevel", args = 0)]
    pub fn damage_effect_level(self) -> DamageEffectLevel;

    #[method(name = "get_IsCritical", args = 0)]
    pub fn is_critical(self) -> bool;

    #[method(name = "get_IsPlayerSideAttack", args = 0)]
    pub fn is_player_side_attack(self) -> bool;

    #[method(name = "get_IsEnemySideAttack", args = 0)]
    pub fn is_enemy_side_attack(self) -> bool;

    #[method(name = "IsDeadSomeone", args = 0)]
    pub fn is_dead_someone(self) -> bool;

    #[method(name = "IsDeadDamager", args = 0)]
    pub fn is_dead_damager(self) -> bool;

    #[method(name = "IsDead", args = 1)]
    pub fn is_dead(self, side: i32) -> bool;
}

#[unity2::class(namespace = "Combat")]
pub struct CharacterGameStatus {
    #[rename(name = "m_Appearance")]
    pub appearance: CharacterAppearance,
    #[rename(name = "m_Side")]
    pub side: i32,
    #[rename(name = "m_Unit")]
    pub unit: Unit,
    #[rename(name = "m_Person")]
    pub person: PersonData,
    #[rename(name = "m_Job")]
    pub job: JobData,
    #[rename(name = "m_MaxHp")]
    pub max_hp: i32,
    #[rename(name = "m_Hp")]
    pub hp: i32,
    #[rename(name = "m_MaxStun")]
    pub max_stun: i32,
    #[rename(name = "m_StunValue")]
    pub stun_value: i32,
    #[rename(name = "m_EngageCount")]
    pub engage_count: i32,
    #[rename(name = "m_MapX")]
    pub map_x: i32,
    #[rename(name = "m_MapY")]
    pub map_y: i32,
    #[rename(name = "m_BattleX")]
    pub battle_x: i32,
    #[rename(name = "m_BattleY")]
    pub battle_y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AssetTableSound {
    pub voice_id: Il2CppString,
    pub footstep_id: Il2CppString,
    pub material_id: Il2CppString,
}

#[unity2::class(namespace = "Combat")]
pub struct CharacterAppearance {
    #[rename(name = "m_AnimsetNames")]
    pub animset_names: List<Il2CppString>,
    #[rename(name = "m_MaskColor100")]
    pub mask_color_100: Color,
    #[rename(name = "m_MaskColor075")]
    pub mask_color_075: Color,
    #[rename(name = "m_MaskColor050")]
    pub mask_color_050: Color,
    #[rename(name = "m_MaskColor025")]
    pub mask_color_025: Color,
    #[rename(name = "m_SkinColor")]
    pub skin_color: Color,
    #[rename(name = "m_GradColor")]
    pub grad_color: Color,
    #[rename(name = "m_HairColor")]
    pub hair_color: Color,
    #[rename(name = "m_ToonShadowColor")]
    pub toon_shadow_color: Color,
    // Literal C# field name, public and unprefixed
    #[rename(name = "Sound")]
    pub sound: AssetTableSound,
    #[rename(name = "m_WeaponStyle")]
    pub weapon_style: i32,
}

#[unity2::methods]
impl CharacterAppearance {
    #[method(name = "CreateForSound", args = 1)]
    pub fn create_for_sound(unit: crate::gamedata::unit::Unit) -> CharacterAppearance;
}

#[unity2::class(namespace = "Combat")]
pub struct Character {
    #[rename(name = "m_Side")]
    pub side: i32,
    #[rename(name = "m_ChainId")]
    pub chain_id: i32,
    #[rename(name = "m_GameStatus")]
    pub game_status: CharacterGameStatus,
    #[rename(name = "m_IsDoneSetup")]
    pub is_done_setup: bool,
    #[rename(name = "m_EnemySide")]
    pub enemy_side: i32,
    #[rename(name = "m_GroundLevel")]
    pub ground_level: f32,
    #[backing(name = "PlayingEvent")]
    pub playing_event: AnimationEvent,
}

#[unity2::methods]
impl Character {
    #[method(name = "get_Side", args = 0)]
    pub fn get_side(self) -> i32;

    #[method(name = "get_Phase", args = 0)]
    pub fn get_phase(self) -> Phase;
}

#[unity2::class(namespace = "Combat")]
pub struct CharacterSound {}

#[unity2::methods]
impl CharacterSound {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> Character;
}

#[unity2::class(namespace = "Combat")]
pub struct Magic {
    #[rename(name = "m_Settings")]
    pub magic_bullet_settings: MagicBulletSettings,
}

#[unity2::class(namespace = "Combat")]
pub struct MagicBulletSettings {
    #[rename(name = "m_HomeNodeName")]
    pub home_node_name: Il2CppString,
    #[rename(name = "m_TargetNodeName")]
    pub target_node_name: Il2CppString,
    #[rename(name = "m_Float")]
    pub float: f32,
    #[rename(name = "m_ArrivalType")]
    pub arrival_type: ArrivalType,
    #[rename(name = "m_MoveSpeed")]
    pub move_speed: f32,
}

#[unity2::class(namespace = "Combat")]
pub struct MagicSignal {
    #[rename(name = "m_Level")]
    pub level: i32,
    #[rename(name = "m_Frame")]
    pub frame: f32,
    #[rename(name = "m_Command")]
    pub command: i32,
    #[rename(name = "m_ParentName")]
    pub parent_name: Il2CppString,
    #[rename(name = "m_Connect")]
    pub connect: i32,
    #[rename(name = "m_IntParameter")]
    pub int_parameter: i32,
    #[rename(name = "m_FloatParameter")]
    pub float_parameter: f32,
    #[rename(name = "m_StringParameter")]
    pub string_parameter: Il2CppString,
}

#[unity2::class(namespace = "Combat")]
pub struct MagicSignalProcessor {
    #[rename(name = "m_Character")]
    pub character: Character,
}

#[unity2::methods]
impl MagicSignalProcessor {
    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> Magic;
}

#[unity2::class(namespace = "UnityEngine")]
pub struct AnimationEvent {}

#[unity2::methods]
impl AnimationEvent {
    #[method(name = "get_stringParameter", args = 0)]
    pub fn get_string_parameter(self) -> Il2CppString;
}

#[unity2::class(namespace = "Combat")]
pub struct Side {}

#[unity2::methods]
impl Side {
    #[method(name = "IsMaster", args = 1)]
    pub fn is_master(i: i32) -> bool;

    #[method(name = "IsChainAtk", args = 1)]
    pub fn is_chain_atk(i: i32) -> bool;
}

#[unity2::class(namespace = "Combat")]
pub struct RuntimeAnimUtil {}

#[unity2::methods]
impl RuntimeAnimUtil {
    #[method(name = "IsEvasion", args = 1)]
    pub fn is_evasion(hash: i32) -> bool;

    #[method(name = "IsParry", args = 1)]
    pub fn is_parry(hash: i32) -> bool;

    #[method(name = "IsGuard", args = 1)]
    pub fn is_guard(hash: i32) -> bool;
}
