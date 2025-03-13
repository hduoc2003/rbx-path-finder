use std::fmt::Debug;
use std::ops::Add;

pub type NodeId = usize;

#[derive(Debug, PartialEq)]
pub struct PathTrace<U: G> {
    pub dist: U,
    pub path: Option<Vec<NodeId>>,
}

pub struct EdgeInfo<U: G> {
    pub u: NodeId,
    pub v: NodeId,
    pub w: U,
}

pub trait G: Add<Output = Self> + Copy + PartialOrd + Ord + Debug {}

pub trait ShortestPathAlgo<U: G> {
    const NAME: &'static str;
    fn find(&self, u: NodeId, v: NodeId, with_trace: bool) -> PathTrace<U>;
    fn add_edges(&mut self, edges: &Vec<EdgeInfo<U>>);
}

impl G for u128 {}
