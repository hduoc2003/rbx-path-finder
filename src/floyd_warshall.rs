use crate::errors::Error;
use crate::shortest_path_algo::{EdgeInfo, EdgeWeight, NodeId, PathTrace, ShortestPathAlgo};

pub struct FloydWarshall<U: EdgeWeight = u128> {
    n: NodeId,
    dist: Vec<Vec<U>>,
    trace: Vec<Vec<NodeId>>,
    highest_total_weight: U,
}

impl<U: EdgeWeight> FloydWarshall<U> {
    const MAX_NODE_ID: NodeId = 500;
    pub fn new(
        max_node_id: NodeId,
        edges: &Vec<EdgeInfo<U>>,
        highest_total_weight: U,
    ) -> Result<Self, Error> {
        if max_node_id > Self::MAX_NODE_ID {
            Err(Error::GraphTooLarge)
        } else {
            let mut trace = Self::init_trace(max_node_id);
            let mut dist = Self::init_dist(max_node_id, edges, highest_total_weight);

            Self::run_core(max_node_id, &mut trace, &mut dist);

            Ok(Self {
                n: max_node_id,
                dist,
                trace,
                highest_total_weight,
            })
        }
    }

    fn init_trace(max_node_id: NodeId) -> Vec<Vec<NodeId>> {
        let mut trace = Vec::with_capacity(max_node_id);
        for node in 0..max_node_id {
            trace.push(vec![node; max_node_id]);
        }
        trace
    }

    fn init_dist(
        max_node_id: NodeId,
        edges: &Vec<EdgeInfo<U>>,
        highest_total_weight: U,
    ) -> Vec<Vec<U>> {
        let mut dist = vec![vec![highest_total_weight; max_node_id]; max_node_id];
        for e in edges {
            dist[e.u][e.v] = e.w;
            dist[e.v][e.u] = e.w;
        }
        for u in 0..max_node_id {
            dist[u][u] = U::zero();
        }
        dist
    }

    fn run_core(max_node_id: NodeId, trace: &mut [Vec<NodeId>], dist: &mut [Vec<U>]) {
        for k in 0..max_node_id {
            for u in 0..max_node_id {
                for v in 0..max_node_id {
                    if dist[u][v] > dist[u][k] + dist[k][v] {
                        dist[u][v] = dist[u][k] + dist[k][v];
                        trace[u][v] = trace[k][v];
                    }
                }
            }
        }
    }
}

impl<U: EdgeWeight> ShortestPathAlgo<U> for FloydWarshall<U> {
    const NAME: &'static str = "Floyd-Warshall";

    fn find(&mut self, u: NodeId, mut v: NodeId, with_trace: bool) -> Option<PathTrace<U>> {
        if self.dist[u][v] == self.highest_total_weight {
            return None;
        }
        let dist = self.dist[u][v];
        let path = if with_trace {
            let mut path = vec![];
            while v != u {
                path.push(v);
                v = self.trace[u][v];
            }
            path.push(u);
            path.reverse();

            Some(path)
        } else {
            None
        };
        Some(PathTrace { dist, path })
    }

    fn add_edges(&mut self, edges: &Vec<EdgeInfo<U>>) {
        for e in edges {
            let uv = &mut self.dist[e.u][e.v];
            if *uv > e.w {
                *uv = e.w;
                self.trace[e.u][e.v] = e.u;
                self.trace[e.v][e.u] = e.v;
            }
            self.dist[e.v][e.u] = *uv;
        }
        Self::run_core(self.n, &mut self.trace, &mut self.dist);
    }
}
