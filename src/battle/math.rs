use unity2::system::collections::Stack;
use unity2::system::delegate::Delegate;
use unity2::system::delegate::IDelegate;
use unity2::system::delegate::MulticastDelegate;
use unity2::system::delegate::IMulticastDelegate;
use crate::random::RandomSeed;


#[unity2::class(namespace = "App")]
pub struct BattleMath {
    #[static_field]
    #[rename(name = "s_CurrentProbability100")]
    pub current_probability_100: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_CurrentProbabilityHit")]
    pub current_probability_hit: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_Probability100")]
    pub probability_100: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_ProbabilityHit")]
    pub probability_hit: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_ProbabilityTrue")]
    pub probability_true: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_ProbabilityFalse")]
    pub probability_false: BattleMathProbability,
    #[static_field]
    #[rename(name = "s_RandomSeed")]
    pub random_seed: Stack<RandomSeed>,
    #[static_field]
    #[rename(name = "s_Simulationed")]
    pub simulationed: i32,
}

#[unity2::class(namespace = "", name = "BattleMath.Probability")]
#[parent(unity2::system::delegate::MulticastDelegate)]
pub struct BattleMathProbability {}