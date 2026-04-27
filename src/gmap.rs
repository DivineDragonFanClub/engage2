#![allow(unused_imports)]

use unity2::engine::GameObject;
use unity2::engine::object::{Camera, RectTransform, Sprite};
use unity2::engine::ui::Image;
use unity2::system::List;
use unity2::engine::object::{
    Object, Component, Behaviour, MonoBehaviour,
    IObject, IComponent, IBehaviour, IMonoBehaviour,
};
use unity2::{Array, ClassIdentity, Il2CppString};

use crate::Unit;
use crate::data::SingletonClass;
use crate::data::ISingletonClass;

use crate::gamedata::dispos::ChapterData;
use crate::interpolator::{InterpolatorFloat, InterpolatorVector3};
use crate::uniticon::UnitIcon;

#[unity2::class(namespace = "App")]
pub struct GmapSpot {
    #[rename(name = "m_GlobalFlagName")]
    pub global_flag_name: Il2CppString,
    #[rename(name = "m_Chapters")]
    pub chapters: List<ChapterData>,
    #[rename(name = "m_GameObject")]
    pub game_object: GameObject,
    #[rename(name = "m_Controller")]
    pub controller: GmapSpotController,
    #[rename(name = "m_MapObject")]
    pub map_object: GameObject,
    #[rename(name = "m_NextSpots")]
    pub next_spots: Array<GmapSpot>,
    #[rename(name = "m_MobUnit")]
    pub mob_unit: GmapMobUnit,
    #[rename(name = "m_EncountIcon")]
    pub encount_icon: EncountIcon,
    #[backing] pub reserve_dispos: bool,
    #[backing] pub reserve_delete_mob: bool,
}

#[unity2::methods]
impl GmapSpot {
    #[method(name = "get_Chapter", args = 0)]
    pub fn chapter(self) -> ChapterData;

    #[method(name = "get_GmapSpotID", args = 0)]
    pub fn gmap_spot_id(self) -> Il2CppString;
}

#[unity2::class(namespace = "App")]
pub struct GmapMapInfoContent {
    #[rename(name = "m_MapInfoImage")]
    pub map_info_image: Image,
    #[rename(name = "m_MapInfoSprite")]
    pub map_info_sprite: Sprite,
}

#[unity2::methods]
impl GmapMapInfoContent {
    #[method(name = "SetMapInfo", args = 1)]
    pub fn set_map_info(self, gmap_spot: GmapSpot);
}

#[unity2::class(namespace = "App")]
#[parent(unity2::engine::object::MonoBehaviour)]
pub struct GmapSpotController {}

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<GmapSpotManager>)]
pub struct GmapSpotManager {
    #[rename(name = "m_SpotArray")]
    pub spot_array: Array<GmapSpot>,
    #[static_field]
    #[readonly]
    pub encount_mob_max: i32,
}

#[unity2::class(namespace = "App")]
#[parent(SingletonClass<GmapCamera>)]
pub struct GmapCamera {
    #[rename(name = "m_SphereCenter")]
    pub sphere_center: GameObject,
    #[rename(name = "m_DefaultAngleX")]
    pub default_angle_x: f32,
    #[rename(name = "m_DefaultAngleZ")]
    pub default_angle_z: f32,
    #[rename(name = "m_AngleCurve")]
    pub angle_curve: GameObject,
    #[rename(name = "m_DistanceCurve")]
    pub distance_curve: GameObject,
    #[rename(name = "m_ZoomSpeed")]
    pub zoom_speed: f32,
    #[rename(name = "m_Camera")]
    pub camera: Camera,
    #[rename(name = "m_ZoomParam")]
    pub zoom_param: Array<f32>,
    #[rename(name = "m_ZoomDir")]
    pub zoom_dir: f32,
    #[rename(name = "m_Position")]
    pub position: InterpolatorVector3,
    #[rename(name = "m_AngleX")]
    pub angle_x: InterpolatorFloat,
    #[rename(name = "m_Distance")]
    pub distance: InterpolatorFloat,
    #[rename(name = "m_IsRStickZoom")]
    pub is_r_stick_zoom: bool,
    #[rename(name = "m_PrevZoom")]
    pub prev_zoom: f32,
    #[backing]
    pub is_camera_tracking: bool,
}

#[unity2::class(namespace = "App")]
pub struct GmapMobUnit {
    #[rename(name = "m_Unit")]
    pub unit: Unit,
}

#[unity2::class(namespace = "", name = "EncountIcon")]
#[parent(unity2::engine::object::MonoBehaviour)]
pub struct EncountIcon {
    #[static_field]
    #[backing]
    pub root_object: GameObject,
    #[static_field]
    #[backing]
    pub camera: GmapCamera,
    #[static_field]
    #[rename(name = "s_IsVisible")]
    pub is_visible: bool,
    
    #[rename(name = "m_UnitIcon")]
    pub unit_icon: UnitIcon,
    #[rename(name = "m_IsColliderHit")]
    pub is_collider_hit: bool,
    #[rename(name = "m_GmapSpot")]
    pub gmap_spot: GmapSpot,
    #[rename(name = "m_RectTransform")]
    pub rect_transform: RectTransform,
    #[rename(name = "m_ChildObjects")]
    pub child_objects: List<GameObject>,
    #[rename(name = "m_Initialized")]
    pub initialized: bool,
}

#[unity2::enumeration(namespace = "", name = "GmapSpot.EncountPersonType")]
#[repr(i32)]
pub enum GmapSpotEncountPersonType {
    None = 0,
    Morph = 1,
    Filene = 2,
    Brodia = 3,
    Solum = 4,
    Ircion = 5,
    Troublemaker = 6,
}

#[unity2::enumeration(namespace = "", name = "GmapSpot.EncountType")]
#[repr(i32)]
pub enum GmapSpotEncountType {
    None = 0,
    Exturmination = 1, // Note the misspelling
    TrainingFilene = 2,
    TrainingBrodia = 3,
    TrainingSolum = 4,
    TrainingIrcion = 5,
}

#[unity2::enumeration(namespace = "", name = "GameSpot.Direction")]
#[repr(i32)]
pub enum GmapSpotDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[unity2::enumeration(namespace = "", name = "GmapSpot.State")]
#[repr(i32)]
pub enum GmapSpotState {
    ReserveHide = 0,
    Hide = 1,
    ReserveActive = 2,
    Active = 3,
    ReserveCannotEnter = 4,
    CannotEnter = 5,
    ReserveBroken = 6,
    Broken = 7,
    CanSearch = 8,
}