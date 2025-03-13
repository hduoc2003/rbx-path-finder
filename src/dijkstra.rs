use crate::shortest_path_algo::{EdgeInfo, NodeId, PathTrace, ShortestPathAlgo, G};

pub struct Dijkstra {
    n: NodeId
}

impl<U: G> ShortestPathAlgo<U> for Dijkstra {
    const NAME: &'static str = "Dijkstra";

    fn find(&self, u: NodeId, v: NodeId, with_trace: bool) -> PathTrace<U> {
        todo!()
    }

    fn add_edges(&mut self, edges: &Vec<EdgeInfo<U>>) {
        todo!()
    }
}
