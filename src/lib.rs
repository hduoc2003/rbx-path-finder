pub mod dijkstra;
pub mod errors;
pub mod floyd_warshall;
pub mod shortest_path_algo;

#[cfg(test)]
mod tests {
    use crate::dijkstra::Dijkstra;
    use crate::floyd_warshall::FloydWarshall;
    use crate::shortest_path_algo::{EdgeInfo, PathTrace, ShortestPathAlgo};

    #[test]
    fn test_all() {
        // 1 2 3
        // 2 3 1
        // 1 3 5
        let max_node_id = 3;
        let edges = vec![
            EdgeInfo { u: 0, v: 1, w: 3 },
            EdgeInfo { u: 1, v: 2, w: 1 },
            EdgeInfo { u: 0, v: 2, w: 5 },
        ];
        let additional_edges = vec![EdgeInfo { u: 0, v: 2, w: 3 }];

        let case_1 = Some(PathTrace {
            dist: 4,
            path: Some(vec![0, 1, 2]),
        });
        let case_2 = Some(PathTrace {
            dist: 3,
            path: Some(vec![0, 2]),
        });

        let mut floyd = FloydWarshall::new(max_node_id, &edges, u128::MAX / 2).unwrap();
        let mut dijkstra = Dijkstra::new(max_node_id, &edges, u128::MAX / 2, max_node_id);

        assert_eq!(floyd.find(0, 2, true), case_1);
        assert_eq!(dijkstra.find(0, 2, true), case_1);

        floyd.add_edges(&additional_edges);
        dijkstra.add_edges(&additional_edges);

        assert_eq!(floyd.find(0, 2, true), case_2);
        assert_eq!(dijkstra.find(0, 2, true), case_2);
    }
}
