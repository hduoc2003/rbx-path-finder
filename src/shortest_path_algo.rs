use std::fmt::Debug;
use std::ops::Add;

pub type NodeId = usize;

#[derive(Debug, PartialEq)]
pub struct PathTrace<U: EdgeWeight> {
    pub dist: U,
    pub path: Option<Vec<NodeId>>,
}

pub struct EdgeInfo<U: EdgeWeight> {
    pub u: NodeId,
    pub v: NodeId,
    pub w: U,
}

pub trait ZeroWeight {
    fn zero() -> Self;
}

pub trait EdgeWeight: Add<Output = Self> + Copy + PartialOrd + Ord + Debug + ZeroWeight {}

pub trait ShortestPathAlgo<U: EdgeWeight> {
    const NAME: &'static str;
    fn find(&mut self, u: NodeId, v: NodeId, with_trace: bool) -> Option<PathTrace<U>>;
    fn add_edges(&mut self, edges: &Vec<EdgeInfo<U>>);
}

impl ZeroWeight for u128 {
    fn zero() -> Self {
        0
    }
}
impl EdgeWeight for u128 {}
