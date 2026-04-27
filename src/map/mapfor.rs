use unity2::Array;


#[unity2::class(namespace = "App")]
pub struct MapFor {
    #[static_field]
    #[readonly]
    pub rhombus_x: Array<i32>,
    #[static_field]
    #[readonly]
    pub rhombus_z: Array<i32>,
}

#[unity2::class(namespace = "", name = "MapFor.RangeFunction")]
pub struct MapForRangeFunction {}