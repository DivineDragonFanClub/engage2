use unity2::Array;
use unity2::Class;
use unity2::ClassIdentity;
use unity2::Il2CppString;
use unity2::system::List;

use crate::ItemData;
use crate::ItemDataKinds;
use crate::WeaponMask;
use crate::bit::BitStruct;
use crate::capability::CapabilitySbyte;
use crate::data::StructData;
use crate::data::IStructData;
use crate::calculator::calculator::StructCalculatorData;
use crate::calculator::calculator::IStructCalculatorData;
use crate::calculator::calculator::CalculatorCommand;
use crate::gamedata::unititem::WeaponLevels;


#[unity2::class(namespace = "App")]
#[parent(StructCalculatorData<SkillData>)]
pub struct SkillData {
    #[backing] pub sid: Il2CppString,
    #[backing] pub name: Il2CppString,
    #[backing] pub help: Il2CppString,
    #[backing] pub command_name: Il2CppString,
    #[backing] pub command_help: Il2CppString,
    #[backing] pub command_warning: Il2CppString,
    #[backing] pub root_command_sid: Il2CppString,
    #[backing] pub icon_kind: SkillDataIconKinds,
    #[backing] pub icon_label: Il2CppString,
    #[backing] pub icon_bmap: Il2CppString,
    #[backing] pub stand: SkillDataStands,
    #[backing] pub action: SkillDataActions,
    #[backing] pub timing: SkillDataTimings,
    #[backing] pub target: SkillDataTargets,
    #[backing] pub equip_iids: Array<Il2CppString>,
    #[backing] pub group: i32,
    #[backing] pub priority: u8,
    #[backing] pub layer: SkillDataLayers,
    #[backing] pub order: i8,
    #[backing] pub cycle: SkillDataCycles,
    #[backing] pub frequency: SkillDataFrequencies,
    #[backing] pub condition: Il2CppString,
    #[backing] pub give_target: SkillDataGiveTargets,
    #[backing] pub give_condition: Il2CppString,
    #[backing] pub give_sids: Array<Il2CppString>,
    #[backing] pub remove_sids: Array<Il2CppString>,
    #[backing] pub sync_conditions: Array<Il2CppString>,
    #[backing] pub sync_sids: Array<Il2CppString>,
    #[backing] pub rebirth_sid: Il2CppString,
    #[backing] pub engage_sid: Il2CppString,
    #[backing] pub change_sids: Array<Il2CppString>,
    #[backing] pub attack_range: Il2CppString,
    #[backing] pub overlap_range: Il2CppString,
    #[backing] pub overlap_terrain: Il2CppString,
    #[backing] pub zoc_range: Il2CppString,
    #[backing] pub zoc_type: SkillDataZocs,
    #[backing] pub cooperation_skill: Il2CppString,
    #[backing] pub horse_skill: Il2CppString,
    #[backing] pub covert_skill: Il2CppString,
    #[backing] pub heavy_skill: Il2CppString,
    #[backing] pub fly_skill: Il2CppString,
    #[backing] pub magic_skill: Il2CppString,
    #[backing] pub prana_skill: Il2CppString,
    #[backing] pub dragon_skill: Il2CppString,
    #[backing] pub act_names: Array<Il2CppString>,
    #[backing] pub act_operations: Array<Il2CppString>,
    #[backing] pub act_values: Array<Il2CppString>,
    #[backing] pub around_center: SkillDataAroundCenters,
    #[backing] pub around_target: SkillDataAroundTargets,
    #[backing] pub around_condition: Il2CppString,
    #[backing] pub around_name: Il2CppString,
    #[backing] pub around_operation: Il2CppString,
    #[backing] pub around_value: Il2CppString,
    #[backing] pub bad_state: SkillDataStates,
    #[backing] pub bad_ignore: SkillDataStates,
    #[backing] pub efficacy: SkillDataAttrs,
    #[backing] pub efficacy_ignore: SkillDataAttrs,
    #[backing] pub efficacy_value: i32,
    #[backing] pub flag: SkillDataFlags,
    #[backing] pub private_flag: SkillDataPrivateFlags,
    #[backing] pub work: SkillDataWorks,
    #[backing] pub work_operation: Il2CppString,
    #[backing] pub work_value: f32,
    #[backing] pub power: i32,
    #[backing] pub life: i32,
    #[backing] pub cost: i32,
    #[backing] pub rewarp: i32,
    #[backing] pub removable: i32,
    #[backing] pub vision_count: i32,
    #[backing] pub range_target: ItemDataKinds,
    #[backing] pub range_i: i32,
    #[backing] pub range_o: i32,
    #[backing] pub range_add: i32,
    #[backing] pub range_extend: i32,
    #[backing] pub move_self: i32,
    #[backing] pub move_target: i32,
    #[backing] pub enhance_level: i8,
    #[backing] pub enhance_value: CapabilitySbyte,
    #[backing] pub weapon_prohibit: WeaponMask,
    #[backing] pub weapon_level: WeaponLevels,
    #[backing] pub effect: Il2CppString,
    #[backing] pub inheritance_cost: u16,
    #[backing] pub inheritance_sort: u16,
    pub give_skills: SkillArray,
    pub remove_skills: SkillArray,
    pub sync_skills: SkillArray,
    pub rebirth_skill: SkillData,
    pub engage_skill: SkillData,
    pub change_skills: Array<SkillData>,
    pub low_skill: SkillData,
    pub high_skill: SkillData,
    pub root_command_skill: SkillData,
    pub timing_mask: SkillDataTimingMasks,
    pub cycle_mask: SkillDataCycleMasks,
    pub sort_key: i32,
    #[rename(name = "m_ActFuncs")]
    pub act_funcs: List<SkillDataFunc>,
    #[rename(name = "m_AroundFuncs")]
    pub around_funcs: List<SkillDataFunc>,
    #[rename(name = "m_StyleSkills")]
    pub style_skills: Array<SkillData>,
    #[rename(name = "m_WeaponLevelMask")]
    pub weapon_level_mask: WeaponMask,
    #[rename(name = "m_ConditionCommand")]
    pub condition_command: CalculatorCommand,
    #[rename(name = "m_GiveConditionCommand")]
    pub give_condition_command: CalculatorCommand,
    #[rename(name = "m_AroundConditionCommand")]
    pub around_condition_command: CalculatorCommand,
    #[rename(name = "m_SyncConditionCommands")]
    pub sync_condition_commands: Array<CalculatorCommand>,
    #[rename(name = "m_EquipItems")]
    pub equip_items: List<ItemData>,
    #[rename(name = "m_DefaultEquipItem")]
    pub default_equip_item: ItemData,
    #[rename(name = "m_PrefixlessSid")]
    pub prefixless_sid: Il2CppString,

    #[static_field]
    #[rename(name = "s_EfficacySkills")]
    pub efficacy_skills: SkillArray,
    #[static_field]
    #[rename(name = "s_HeroSkill")]
    pub hero_skill: SkillData,
    #[static_field]
    #[rename(name = "s_StunSkill")]
    pub stun_skill: SkillData,
    #[static_field]
    #[rename(name = "s_DanceSkill")]
    pub dance_skill: SkillData,
    #[static_field]
    #[rename(name = "s_PoisonSkill")]
    pub poison_skill: SkillData,
    #[static_field]
    #[rename(name = "s_LeaderSkill")]
    pub leader_skill: SkillData,
    #[static_field]
    #[rename(name = "s_FangCurseSkill")]
    pub fang_curse_skill: SkillData,
    #[static_field]
    #[rename(name = "s_EnchantSkill")]
    pub enchant_skill: SkillData,
    #[static_field]
    #[rename(name = "s_EnchantmentSkill")]
    pub enchantment_skill: SkillData,
    #[static_field]
    #[rename(name = "s_ImmortalSkill")]
    pub immortal_skill: SkillData,
    #[static_field]
    #[rename(name = "s_TransporterSkill")]
    pub transporter_skill: SkillData,
    #[static_field]
    #[rename(name = "s_FullBulletSkill")]
    pub full_bullet_skill: SkillData,
    #[static_field]
    #[rename(name = "s_ChainAttackGuardSkill")]
    pub chain_attack_guard_skill: SkillData,
    #[static_field]
    #[rename(name = "s_NotTerrainDamageSkill")]
    pub not_terrain_damage_skill: SkillData,
    #[static_field]
    #[rename(name = "s_GazeDiagonallySkill")]
    pub gaze_diagonally_skill: SkillData,
    #[static_field]
    #[rename(name = "s_MultiChangeSkill")]
    pub multi_change_skill: SkillData,
}

impl SkillData {
    pub const PHASE_CYCLE: i32 = 3;
    pub const COMMAND_MASK: i32 = 6291456;
}

#[unity2::class(namespace = "App")]
pub struct SkillArray {
    #[rename(name = "m_Mask")]
    pub mask: BitStruct,
    #[rename(name = "m_List")]
    pub list: List<SkillArrayEntity>,
    #[rename(name = "m_Flags")]
    pub flags: SkillDataFlags,
    #[rename(name = "m_Cycles")]
    pub cycles: SkillDataCycleMasks,
    #[rename(name = "m_Timings")]
    pub timings: SkillDataTimingMasks,
    #[rename(name = "m_Efficacys")]
    pub efficacys: SkillDataAttrs,
    #[rename(name = "m_EfficacysIgnores")]
    pub efficacys_ignores: SkillDataAttrs,
    #[rename(name = "m_BadStates")]
    pub bad_states: SkillDataStates,
    #[rename(name = "m_BadIgnore")]
    pub bad_ignore: SkillDataStates,
    #[rename(name = "m_WeaponLevels")]
    pub weapon_levels: WeaponLevels,
    #[rename(name = "m_IsEquipSkillFirstNull")]
    pub is_equip_skill_first_null: bool,
}

impl SkillArray {
    pub const MAX_COUNT: i32 = 1280;
    pub const CAPACITY: i32 = 32;
    pub const VERSION: i32 = 1;
}

#[unity2::class(namespace = "", name = "SkillData.Func")]
pub struct SkillDataFunc {
    #[rename(name = "Name")]
    pub name: Il2CppString,
    #[rename(name = "Operation")]
    pub operation: SkillDataOperations,
    #[rename(name = "SetCommand")]
    pub set_command: CalculatorCommand,
    #[rename(name = "GetCommand")]
    pub get_command: CalculatorCommand,
}

#[unity2::class(namespace = "", name = "SkillArray.Enumerator")]
pub struct SkillArrayEnumerator {
    #[rename(name = "m_Array")]
    pub array: SkillArray,
    #[rename(name = "m_Current")]
    pub current: SkillData,
    #[rename(name = "m_Index")]
    pub index: i32,
}

#[unity2::class(namespace = "", name = "SkillArray.Entity")]
pub struct SkillArrayEntity {
    #[rename(name = "Value")]
    pub value: u32,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Phase")]
#[repr(i32)]
pub enum SkillDataPhase {
    Current = 0,
    Other = 1,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkillDataAttrs: i32 {
        const None = 0;
        const Walk = 1 << 0;
        const Horse = 1 << 1;
        const Heavy = 1 << 2;
        const Fly = 1 << 3;
        const Dragon = 1 << 4;
        const Evil = 1 << 5;
        const Morph = 1 << 6;
        const Mediuth = 1 << 7;
        const Duma = 1 << 8;
        const Loptous = 1 << 9;
        const Veld = 1 << 10;
        const Idenn = 1 << 11;
        const Nergal = 1 << 12;
        const Fodeth = 1 << 13;
        const Ashnard = 1 << 14;
        const Astarte = 1 << 15;
        const Gimle = 1 << 16;
        const Hydra = 1 << 17;
        const Nemesis = 1 << 18;
    }
}

impl ClassIdentity for SkillDataAttrs {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "SkillData.Attrs";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "SkillData.Timings")]
#[repr(i32)]
pub enum SkillDataTimings {
    None = 0,
    Always = 1,
    BattleBefore = 2,
    BattleDetail = 3,
    BattleInvoke = 4,
    BattleStart = 5,
    OrderStart = 6,
    ActionStart = 7,
    AttackStart = 8,
    AttackBranch = 9,
    HitBefore = 10,
    HitAfter = 11,
    HitAffect = 12,
    AttackEnd = 13,
    ActionEnd = 14,
    OrderEnd = 15,
    BattleEnd = 16,
    BattleResult = 17,
    BattleAfter = 18,
    Around = 19,
    Support = 20,
    BattleCommand = 21,
    ActionCommand = 22,
    OverlapCommand = 23,
    SupportCommand = 24,
    FixedNone = 25,
    FixedDone = 26,
    PhaseStart = 27,
}

#[unity2::enumeration(namespace = "", name = "SkillData.TimingMasks")]
#[repr(i32)]
pub enum SkillDataTimingMasks {
    None = 0,
    Full = 1048575,
}

#[unity2::enumeration(namespace = "", name = "SkillData.CycleMasks")]
#[repr(i32)]
pub enum SkillDataCycleMasks {
    None = 0,
    Full = 1048575,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Stands")]
#[repr(i32)]
pub enum SkillDataStands {
    None = 0,
    Offence = 1,
    Defence = 2,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Actions")]
#[repr(i32)]
pub enum SkillDataActions {
    None = 0,
    Offence = 1,
    Defence = 2,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Targets")]
#[repr(i32)]
pub enum SkillDataTargets {
    Target = 0,
    Enemy = 1,
    Friend = 2,
    Destroy = 3,
    Pierce = 4,
    Range = 5,
    Around = 6,
    Overlap = 7,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Cycles")]
#[repr(i32)]
pub enum SkillDataCycles {
    None = 0,
    Map = 1,
    PhaseBefore = 2,
    PhaseAfter = 3,
    Fixed = 4,
    Engaged = 5,
    Battled = 6,
    BattledOf = 7,
    BattledDf = 8,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Operations")]
#[repr(i32)]
pub enum SkillDataOperations {
    None = 0,
    Equal = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Works")]
#[repr(i32)]
pub enum SkillDataWorks {
    None = 0,
    ItemHealScale = 1,
    JobGrowChange = 2,
    TotalGrowChange = 3,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Zocs")]
#[repr(i32)]
pub enum SkillDataZocs {
    None = 0,
    CostMin = 1,
    CostMax = 2,
    NotMove = 3,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Layers")]
#[repr(i32)]
pub enum SkillDataLayers {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkillDataStates: i32 {
        const None = 0;
        const Poison = 1 << 0;
        const DeadlyPoison = 1 << 1;
        const SeverePoison = 1 << 2;
        const Heal = 1 << 3;
        const Sleep = 1 << 4;
        const Silence = 1 << 5;
        const Charm = 1 << 6;
        const Confusion = 1 << 7;
        const Freeze = 1 << 8;
        const Weakness = 1 << 9;
        const Stun = 1 << 10;
        const Interact = 1 << 11;
        const Decoy = 1 << 12;
        const NotEnhance = 1 << 13;
        // No entries for 2^14 = 16384 and 2^15 = 32768.
        const Enhance = 1 << 16;
        const Immovable = 1 << 17;
        const NotMove = 1 << 18;
        const NotWeaponWeight = 1 << 19;
        const NotChainAttacked = 1 << 20;
        const IgnoreDebug = -2147483648; // i32::MIN
        const PoisonMask = 7;
    }
}

impl ClassIdentity for SkillDataStates {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "SkillData.States";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkillDataFlags: i64 {
        const Invisible = 1 << 0;
        const EngageAttack = 1 << 1;
        const EngageCharge = 1 << 2;
        const EngageLink = 1 << 3;
        const EngageWait = 1 << 4;
        const EngageSummon = 1 << 5;
        const IgnoreEngageAttacking = 1 << 6;
        const IgnoreNoEngageAttacking = 1 << 7;
        const EnableChaining = 1 << 8;
        const EnableDestory = 1 << 9; // Note the typo
        const EnableCannon = 1 << 10;
        const EnableRod = 1 << 11;
        const IgnoreAlone = 1 << 12;
        const IgnoreMultiAttacking = 1 << 13;
        const IgnoreTraining = 1 << 14;
        const IgnoreTraial = 1 << 15; // Note the typo
        const IgnoreSimulation = 1 << 16;
        const ExclusiveDance = 1 << 17;
        const RevengeAutoEquip = 1 << 18;
        const SwapOrder = 1 << 19;
        const InterruptOrder = 1 << 20;
        const ContinueBattle = 1 << 21;
        const ForceLateOrder = 1 << 22;
        const EachSupport = 1 << 23;
        const Reactable = 1 << 24;
        const Remagicable = 1 << 25;
        const BeforeMove = 1 << 26;
        const AllowChainAttack = 1 << 27;
        const AllowChainGuard = 1 << 28;
        const AllowEngageGuard = 1 << 29;
        const ForceChainAttack = 1 << 30;
        const JoinChainAttack = 1 << 31;
        const RangeReliance = 1 << 32;
        const PickupReliance = 1 << 33;
        const MoveCostFree = 1 << 34;
        const MoveEnemyPass = 1 << 35;
        const ResetDisorder = 1 << 36;
        const ItemHealAround = 1 << 37;
        const ItemHealGive = 1 << 38;
        const SelfHealRod = 1 << 39;
        const OnlyRecvoerRod = 1 << 40; // Note the typo
        const DecayEnhance = 1 << 41;
        const SubEngageCountLimit = 1 << 42;
        const ReverseCount = 1 << 43;
        const ReCooking = 1 << 44;
        const BasisSkill = 1 << 45;
        const Unstoppable = 1 << 46;
        const HideChangeGod = 1 << 47;
        const OverExpChange = 1 << 48;
        const MoveFly = 1 << 49;
        const ViewRestriction = 1 << 50;
        // No entries for 2^51 and 2^52
        const HasIconBmap = 1 << 53;
        const HasContract = 1 << 54;
        const HauntChainAttack = 1 << 55;
        const HasRootCommand = 1 << 56;
        const HasZOC = 1 << 57;
        const HasWork = 1 << 58;
        const HasVision = 1 << 59;
        const NotCondition = 1 << 60;
        const HasCondition = 1 << 61;
        const HasEnhance = 1 << 62;
        const HasRangeTarget = -9223372036854775808; // i64::MIN
        const IgnoreMask = 127168;
    }
}

impl ClassIdentity for SkillDataFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "SkillData.Flags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkillDataPrivateFlags: i64 {
        const StyleSkill = 1 << 0;
        const CanOverride = 1 << 1;
        const HasEffect = 1 << 2;
    }
}

impl ClassIdentity for SkillDataPrivateFlags {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "SkillData.PrivateFlags";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}

#[unity2::enumeration(namespace = "", name = "SkillData.IconKinds")]
#[repr(i32)]
pub enum SkillDataIconKinds {
    None = 0,
    Efficacy = 1,
    Category = 2,
}

#[unity2::enumeration(namespace = "", name = "SkillData.GiveTargets")]
#[repr(i32)]
pub enum SkillDataGiveTargets {
    Target = 0,
    Slf = 1, // Renamed from "Self"
    Chain = 2,
    Around = 3,
    Dance = 4,
}

#[unity2::enumeration(namespace = "", name = "SkillData.Categorys")]
#[repr(i32)]
pub enum SkillDataCategorys {
    None = 0,
    Person = 1,
    Job = 2,
    Item = 3,
    Equip = 4,
    God = 5,
    Ring = 6,
    Hub = 7,
    Support = 8,
    Battle = 9,
    Private = 10,
    Inheritance = 11,
    Command = 12,
}

#[unity2::enumeration(namespace = "", name = "SkillData.AroundTargets")]
#[repr(i32)]
pub enum SkillDataAroundTargets {
    None = 0,
    Friend = 1,
    Enemy = 2,
    Both = 3,
}

#[unity2::enumeration(namespace = "", name = "SkillData.AroundCenters")]
#[repr(i32)]
pub enum SkillDataAroundCenters {
    None = 0,
    Slf = 1, // Renamed from Self
    Target = 2,
    Link = 3,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct SkillDataFrequencies: i64 {
        const Every = 1 << 0;
        const First = 1 << 1;
        const Last = 1 << 2;
        const Mask = 7;
    }
}

impl ClassIdentity for SkillDataFrequencies {
    const NAMESPACE: &'static str = "";
    const NAME: &'static str = "SkillData.Frequencies";

    fn class() -> unity2::Class {
        Class::lookup(Self::NAMESPACE, Self::NAME)
    }
}