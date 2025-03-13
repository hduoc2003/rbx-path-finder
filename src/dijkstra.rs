use std::cmp::Reverse;

use priority_queue::PriorityQueue;

use crate::shortest_path_algo::{EdgeInfo, EdgeWeight, NodeId, PathTrace, ShortestPathAlgo};

#[derive(Clone)]
struct PathCache<U: EdgeWeight> {
    count: usize,
    dist: Vec<U>,
    trace: Vec<Option<NodeId>>,
}

pub struct Dijkstra<U: EdgeWeight> {
    n: NodeId,
    caches: Vec<Option<PathCache<U>>>,
    adj: Vec<Vec<(NodeId, U)>>,
    highest_total_weight: U,
    max_caches: usize,
    cached_nodes: Vec<NodeId>,
}

impl<U: EdgeWeight> Dijkstra<U> {
    pub fn new(
        max_node_id: NodeId,
        edges: &Vec<EdgeInfo<U>>,
        highest_total_weight: U,
        max_caches: usize,
    ) -> Dijkstra<U> {
        let mut adj = vec![vec![]; max_node_id];
        for e in edges {
            adj[e.u].push((e.v, e.w));
            adj[e.v].push((e.u, e.w));
        }

        Dijkstra {
            n: max_node_id,
            caches: vec![None; max_node_id],
            adj,
            highest_total_weight,
            max_caches,
            cached_nodes: vec![],
        }
    }

    fn get_trace_path(s: NodeId, t: NodeId, trace: &[Option<NodeId>]) -> Option<Vec<NodeId>> {
        let mut path = vec![];
        let mut v = t;
        while v != s {
            path.push(v);
            trace[v]?;
            v = trace[v].unwrap();
        }
        path.push(s);
        path.reverse();
        Some(path)
    }

    fn read_cache(&mut self, s: NodeId, t: NodeId, with_trace: bool) -> Option<PathTrace<U>> {
        if self.caches[s].is_some() {
            self.caches[s].as_mut()?.count += 1;
            Some(PathTrace {
                dist: self.caches[s].as_ref()?.dist[t],
                path: if with_trace {
                    Self::get_trace_path(s, t, &self.caches[s].as_ref()?.trace)
                } else {
                    None
                },
            })
        } else {
            None
        }
    }

    fn add_cache(&mut self, s: NodeId, dist: Vec<U>, trace: Vec<Option<NodeId>>) {
        let cached_node = &mut self.cached_nodes;
        if cached_node.len() >= self.max_caches {
            let useless = cached_node
                .iter()
                .min_by_key(|node| self.caches[**node].as_ref().unwrap().count)
                .copied();
            if let Some(useless) = useless {
                cached_node.retain(|node| node != &useless);
            }
        }
        
        self.caches[s] = Some(PathCache {
            count: 1,
            dist,
            trace,
        });
        self.cached_nodes.push(s);
    }
}

impl<U: EdgeWeight> ShortestPathAlgo<U> for Dijkstra<U> {
    const NAME: &'static str = "Dijkstra";

    fn find(&mut self, s: NodeId, t: NodeId, with_trace: bool) -> Option<PathTrace<U>> {
        if let Some(cache) = self.read_cache(s, t, with_trace) {
            return Some(cache);
        }
        if let Some(mut cache) = self.read_cache(t, s, with_trace) {
            if cache.path.is_some() {
                let mut path = cache.path.unwrap();
                path.reverse();
                cache.path = Some(path);
            }
            return Some(cache);
        }

        let mut pq = PriorityQueue::new();
        let mut dist = vec![self.highest_total_weight; self.n];
        let mut trace = vec![None; self.n];

        dist[s] = U::zero();
        pq.push(s, Reverse(dist[s]));

        while let Some((u, du)) = pq.pop() {
            let du = du.0;
            if du != dist[u] {
                continue;
            }
            for (v, w) in &self.adj[u] {
                let v = *v;
                let w = *w;
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    trace[v] = Some(u);
                    pq.push(v, Reverse(dist[v]));
                }
            }
        }

        trace[t]?;
        let result = Some(PathTrace {
            dist: dist[t],
            path: if with_trace {
                Self::get_trace_path(s, t, &trace)
            } else {
                None
            },
        });

        self.add_cache(s, dist, trace);

        result
    }

    fn add_edges(&mut self, edges: &Vec<EdgeInfo<U>>) {
        for e in edges {
            self.adj[e.u].push((e.v, e.w));
            self.adj[e.v].push((e.u, e.w));
        }
        
        for cached_node in &self.cached_nodes {
            self.caches[*cached_node] = None;
        }
    }
}
