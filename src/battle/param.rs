use unity2::Array;


#[unity2::class(namespace = "App")]
pub struct BattleParam {
    #[static_field]
    #[rename(name = "INVALID")]
    pub invalid: f32,
    #[static_field]
    pub mins: Array<f32>,
    #[static_field]
    pub maxs: Array<f32>,
    #[static_field]
    pub clamps: Array<f32>,

    pub add: f32,
    pub scale: f32,
    pub value: f32,
}

#[unity2::enumeration(namespace = "", name = "BattleParam.Kinds")]
#[repr(i32)]
pub enum BattleParamKinds {
    Value = 0,
    Ratio = 1,
    Num = 2,
}