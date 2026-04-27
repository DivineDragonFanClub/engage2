use unity2::ClassIdentity;
use unity2::system::List;
use unity2::system::collections::Stack;


#[unity2::class(namespace = "App")]
pub struct Pool {}


// Nested types
#[unity2::class(namespace = "", name = "Pool.Node")]
pub struct PoolNode {}

#[unity2::class(namespace = "", name = "Pool.List`1")]
pub struct PoolList<T: ClassIdentity> {
    #[rename(name = "m_List")]
    pub list: List<T>,
    #[rename(name = "m_Pool")]
    pub pool: Stack<T>,
}

#[unity2::class(namespace = "", name = "Pool.Stack`1")]
pub struct PoolStack<T: ClassIdentity> {
    #[rename(name = "m_Pool")]
    pub pool: Stack<T>,
}

#[unity2::class(namespace = "", name = "Pool.Hierarchy`1")]
pub struct PoolHierarchy<T: ClassIdentity> {
    #[rename(name = "m_Pool")]
    pub pool: Stack<T>,
    #[rename(name = "m_Used")]
    pub used: Stack<T>,
}