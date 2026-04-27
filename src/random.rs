#[unity2::class(namespace = "App")]
pub struct RandomSeed {
    #[rename(name = "Seed1")]
    pub seed_1: u32,
    #[rename(name = "Seed2")]
    pub seed_2: u32,
    #[rename(name = "Seed3")]
    pub seed_3: u32,
    #[rename(name = "Seed4")]
    pub seed_4: u32,
}

impl RandomSeed {
    pub const VERSION: i32 = 0;
}