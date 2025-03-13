pub mod dijkstra;
pub mod errors;
pub mod floyd_warshall;
pub mod shortest_path_algo;

#[cfg(test)]
mod tests {
    use crate::floyd_warshall::FloydWarshall;
    use crate::shortest_path_algo::{EdgeInfo, PathTrace, ShortestPathAlgo};

    #[test]
    fn test_floyd_warshall() {
        // 1 2 3
        // 2 3 1
        // 1 3 5
        let max_node_id = 3;
        let edges = vec![
            EdgeInfo { u: 0, v: 1, w: 3 },
            EdgeInfo { u: 1, v: 2, w: 1 },
            EdgeInfo { u: 0, v: 2, w: 5 },
        ];
        let mut finder = FloydWarshall::new(max_node_id, &edges, 0).unwrap();

        assert_eq!(finder.find(0, 2, true), PathTrace {
            dist: 4,
            path: Some(vec![0, 1, 2])
        });

        finder.add_edges(&vec![EdgeInfo { u: 0, v: 2, w: 3 }]);

        assert_eq!(finder.find(0, 2, true), PathTrace {
            dist: 3,
            path: Some(vec![0, 2])
        });
    }
}
