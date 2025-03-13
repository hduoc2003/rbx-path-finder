# rbx-path-finder

This library implements two algorithms: **Floyd-Warshall** and **Dijkstra** to find the shortest path between two nodes
in a graph with **no negative weights**. Additional edges can be added later.

## Running Tests

To run tests, use:

```sh
cargo test
```

## Project Structure

```
.
└── src/
    ├── shortest_path_algo.rs – Defines general traits and types.
    ├── dijkstra.rs - Implements Dijkstra's algorithm.
    └── floyd_warshall.rs - Implements the Floyd-Warshall algorithm.
```

## Future Enhancements

Additional algorithms to be implemented:

- **Bellman-Ford** – Supports graphs with negative weight edges.
- **Topological Order DP** – Optimized for directed acyclic graphs (DAGs).